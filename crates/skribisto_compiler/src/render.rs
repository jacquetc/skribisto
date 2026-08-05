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
use skribisto_model::language;
use skribisto_model::numbering::{self, Numbered, NumberingRules};
use skribisto_model::scene_break::{self, SceneBreakTier};
use text_document::{
    DocxExportOptions, EpubExportOptions, MarkdownExportOptions, PdfExportOptions,
    PlainTextExportOptions, TextDirection, TextDocument,
};

use crate::headings::{self, Level};
use crate::preset::{
    DirectionMode, EpigraphPlacement, ExportFormat, HeadingLanguage, HeadingScheme, LineSpacing,
    PageSize, Preset, SceneBreak,
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
                // Empty, and correctly so: a Skribisto manuscript is prose. Nothing in the
                // app inserts an image into a document — there is no image affordance in any
                // editor and no `ContentRole` that carries one — so there are no blobs to
                // register. If images ever land, this is where their bytes join the export.
                images: Default::default(),
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
                // Empty, and correctly so: a Skribisto manuscript is prose. Nothing in the
                // app inserts an image into a document — there is no image affordance in any
                // editor and no `ContentRole` that carries one — so there are no blobs to
                // register. If images ever land, this is where their bytes join the export.
                images: Default::default(),
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
        // Empty for the same reason as the EPUB and PDF registries: a Skribisto manuscript
        // is prose, and nothing in the app puts an image into a document.
        images: Default::default(),
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
        // Empty ⇒ `DocxExportOptions::resolved_heading_styles` falls back to
        // `DocxHeadingStyle::default_ramp` scaled off the body size, which is
        // exactly the output this function produced before the field existed.
        heading_styles: Vec::new(),
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
        ExportFormat::Markdown => {
            doc.to_markdown_with(MarkdownExportOptions { page_breaks: true })?
        }
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

    let work_rtl = is_rtl_row(preset, req.work_lang);

    // A title page is built *after* the body — it carries the manuscript's word count,
    // which is only known once the rows have been walked — and prepended. Decided up
    // front all the same, because the body's first block has to know a page is coming
    // above it.
    let title_page = preset.book_title_page && rows.iter().any(|r| r.item.sub_role.opens_book());

    // A page break queued for the next block emitted, whatever that turns out to be: a
    // structural heading, or the row's own prose when the preset suppresses the heading.
    // It survives a row that emits nothing (a Book opener under a title page emits no
    // heading at all), so the break lands on the next thing that *is* printed rather than
    // being lost at that seam.
    let mut pending_break = false;
    // Whether anything will be printed above the block about to be emitted. A page break
    // on the very first block would open on a blank page in the formats that take it
    // literally, and the title page counts here even though `out` is still empty.
    let mut anything_above = title_page;

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
            && let Some(prose) = content_of(row.contents, role)
        {
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
            contributed |= emitted;
        }

        // 4. The synopsis, if the preset keeps it.
        if preset.include_synopses
            && let Some(syn) = content_of(row.contents, ContentRole::SynopsisText)
        {
            // A synopsis is commentary, not the scene's prose — never
            // scanned for markers, and its words are not the manuscript's.
            let (_, emitted) = push_prose(
                &mut out,
                syn,
                row_rtl,
                preset,
                false,
                &mut pending_attrs,
                &[],
            );
            contributed |= emitted;
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
        out.insert_str(0, &render_title_page(req, preset, work_rtl, words));
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
/// encoding of the key — [`dir_pair`] and every attribute set below build from
/// it, so the spelling lives in one place.
fn dir_pair(rtl: bool) -> Option<&'static str> {
    rtl.then_some("direction=rtl")
}

/// One `{k=v k=v}` line for a block, or nothing when it carries no attributes.
///
/// A Djot block gets exactly **one** attribute set, so every key that applies to a block
/// has to be merged here — a second `{…}` line is parsed as its own empty block, which is
/// how an attribute can appear in the output and still do nothing at all.
fn attr_line(rtl: bool, extra: &[String]) -> String {
    let mut attrs: Vec<&str> = Vec::new();
    attrs.extend(dir_pair(rtl));
    attrs.extend(extra.iter().map(String::as_str));
    if attrs.is_empty() {
        String::new()
    } else {
        format!("{{{}}}\n", attrs.join(" "))
    }
}

fn push_heading(out: &mut String, level: u8, text: &str, rtl: bool, extra: &[String]) {
    out.push_str(&attr_line(rtl, extra));
    for _ in 0..level.clamp(1, 6) {
        out.push('#');
    }
    out.push(' ');
    out.push_str(text.trim());
    out.push_str("\n\n");
}

fn push_para(out: &mut String, text: &str, rtl: bool, extra: &[String]) {
    out.push_str(&attr_line(rtl, extra));
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
    lead: &[String],
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
    if !rtl && pending.is_empty() && lead.is_empty() && !could_hold_marker {
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
    // `lead` (a queued page break) belongs to the first block that actually reaches the
    // output, and to that one alone — hence the flag rather than a plain closure capture.
    let mut lead_pending = !lead.is_empty();
    let flush = |out: &mut String,
                 range: Option<(usize, usize)>,
                 pending: &mut Vec<String>,
                 lead_pending: &mut bool| {
        let Some((a, b)) = range else { return false };
        let text = trimmed[a..b.min(trimmed.len())].trim_end();
        if text.trim().is_empty() {
            return false;
        }
        let mut attrs: Vec<String> = Vec::new();
        attrs.extend(dir_pair(rtl).map(str::to_string));
        if *lead_pending {
            attrs.extend(lead.iter().cloned());
            *lead_pending = false;
        }
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
                if flush(out, Some(offsets[i]), pending, &mut lead_pending) {
                    emitted_prose = true;
                }
            } else {
                run_start.get_or_insert(offsets[i].0);
            }
            words += block.split_whitespace().count();
            continue;
        };
        if flush(
            out,
            run_start.take().map(|a| (a, offsets[i].0)),
            pending,
            &mut lead_pending,
        ) {
            emitted_prose = true;
        }
        let style = match tier {
            SceneBreakTier::Minor => &preset.scene_break,
            SceneBreakTier::Major => &preset.major_scene_break,
        };
        push_scene_break(out, style, preset, rtl, pending);
    }
    if flush(
        out,
        run_start.take().map(|a| (a, trimmed.len())),
        pending,
        &mut lead_pending,
    ) {
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

/// The title page: the word count, then the title about a third of the way down, then the
/// byline under it.
///
/// The vertical drop is a real `top_margin` on the title block rather than a run of empty
/// paragraphs. Empty paragraphs are what a word processor's user would do and what every
/// format then renders differently — and a blank Djot block is not even representable,
/// since consecutive blank lines collapse into the ordinary block separator.
///
/// Nothing here carries a page break. It is the top of the document, and the *body's*
/// first block is what breaks away from it.
fn render_title_page(req: &RenderRequest, preset: &Preset, rtl: bool, words: usize) -> String {
    let w = &req.gathered.work;
    let lang = match &preset.heading_language {
        HeadingLanguage::Fixed(l) => l.clone(),
        HeadingLanguage::Auto => req.work_lang.to_string(),
    };
    let mut page = String::new();

    // Upper right, the manuscript-submission convention. Suppressed for an empty
    // manuscript: "about 0 words" is a statement no title page should make.
    if preset.title_page_word_count && words > 0 {
        push_para(
            &mut page,
            &escape_block_leading(&headings::word_count_note(&lang, words, preset.digit_style)),
            rtl,
            &["alignment=right".to_string()],
        );
    }

    if !w.title.trim().is_empty() {
        push_heading(
            &mut page,
            1,
            w.title.trim(),
            rtl,
            &[
                "alignment=center".to_string(),
                format!("top_margin={}", title_drop_px(preset)),
            ],
        );
    }

    // The author's name is *data*, never markup — escaped so a name that happens to start
    // like a list marker survives intact. The preposition is generated furniture and so is
    // localized; the name itself never is.
    if !w.author_name.trim().is_empty() {
        let byline = format!("{} {}", headings::by(&lang), w.author_name.trim());
        push_para(
            &mut page,
            &escape_block_leading(&byline),
            rtl,
            &["alignment=center".to_string()],
        );
    }
    page
}

/// How far down the page a title page's title sits, in logical pixels.
///
/// About a third of the way into the *text* area — the traditional placement, and what
/// Shunn asks for on a novel's title page. Derived from the preset's own page size and
/// margins so it stays a third whether the manuscript is A4 or A5.
fn title_drop_px(preset: &Preset) -> i64 {
    let page_h_in = match preset.page_size {
        PageSize::A4 => 297.0 / 25.4,
        PageSize::Letter => 11.0,
        PageSize::A5 => 210.0 / 25.4,
    };
    let text_h_in =
        (page_h_in - preset.margin.top_in as f64 - preset.margin.bottom_in as f64).max(0.0);
    // 96 logical pixels to the inch, the unit the block attributes use.
    ((text_h_in / 3.0) * 96.0).round().max(0.0) as i64
}

/// Emit one row's epigraph, returning whether anything reached the output.
///
/// `lead` is a page break this epigraph is opening the chapter with, if any. It rides the
/// quotation's own attribute line rather than a line in front of it — see
/// [`mark_epigraph`] — and falls back to an ordinary block attribute when the epigraph was
/// typed as a bare paragraph with no `>` to hang it on.
fn push_epigraph(
    out: &mut String,
    epi: &str,
    lead: &[String],
    rtl: bool,
    preset: &Preset,
    pending_attrs: &mut Vec<String>,
    row: &Row<'_>,
) -> bool {
    // Mark the quote as an epigraph, not merely as quoted text. `semantic_role` rides the
    // blockquote's first block — djot block attributes are the only channel — and
    // text-document lifts it onto the frame, which is what lets EPUB emit
    // `epub:type="epigraph"`, DOCX give it a named style and Typst use its own attribution
    // slot. Without it every writer sees an ordinary blockquote and none can say what it is.
    let (marked, is_quote) = mark_epigraph(epi, lead);
    // Never scanned for break markers: an epigraph is quoted matter, not the scene flow a
    // break divides. Its words are not the manuscript's either, so the count is discarded
    // exactly as the synopsis's is — an epigraph must not move a pace target.
    let (_, emitted) = push_prose(
        out,
        &marked,
        rtl,
        preset,
        false,
        pending_attrs,
        // Already inside the quotation when there was one to put it in.
        if is_quote { &[] } else { lead },
    );
    if emitted {
        // New Hart's Rule: the first line after a heading, an epigraph or a section break
        // carries no first-line indent. Queued the same way a scene break queues it, and
        // consumed by this row's own prose — `push_prose` only lets manuscript prose
        // (`scan_markers`) inherit the queue, so a synopsis cannot swallow it by mistake.
        //
        // **Only when this row has prose of its own.** A Part or a Book has none, so the
        // queue would outlive the row and land on whatever prose came next — a following
        // Scene's opening paragraph, which is a different paragraph entirely and is
        // entitled to its indent. Relying on "the next structural heading clears it" is not
        // enough: the binder is organisational, so nothing guarantees a heading row comes
        // next.
        if main_prose_role(&row.item.sub_role)
            .is_some_and(|role| content_of(row.contents, role).is_some())
        {
            pending_attrs.push("text_indent=0".to_string());
        }
    }
    emitted
}

/// Take a queued page break for the block about to be emitted, if there is one and if
/// there is anything above it to end a page on.
///
/// Both flags are cleared either way. A break the document's very first block cannot use
/// is *spent*, not carried forward: it was asked for "before this content", and this
/// content is already at the top.
///
/// Deliberately not offered to an epigraph. An epigraph's blocks live inside a blockquote,
/// and a page break in there is at best awkward and at worst — in Typst, where the quote
/// is a single content argument — malformed. A row whose heading is suppressed lets the
/// break fall through to its prose instead, which is the paragraph a reader would call
/// the top of the page anyway.
fn take_break(pending: &mut bool, anything_above: &mut bool) -> Vec<String> {
    let take = *pending && *anything_above;
    *pending = false;
    *anything_above = true;
    if take {
        vec!["page_break_before=true".to_string()]
    } else {
        Vec::new()
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

/// The manuscript's numbering, computed once over the **whole** gathered tree.
///
/// This replaces a two-pass design that got the same question wrong two different ways: a
/// running counter bumped inside the row loop (which only ever saw rows a given export had
/// already been filtered down to), plus a `seed_counters` replay for scoped exports (which
/// walked the tree unfiltered). Mark a chapter non-exportable and the two disagreed —
/// "Export Book" renumbered every later chapter, "Export Chapter 7" did not. There is now
/// one pass, in [`skribisto_model::numbering`], and the renderer only looks its answer up.
///
/// Deliberately built from `req.gathered`, never from `req.include`: the export's own
/// selection must not change what number a chapter carries.
/// Empty when the manuscript does not number (`Work.number_chapters`) — the gate lives
/// *here*, at the single source of the numbers, rather than downstream at the schemes.
///
/// Gating the schemes alone was not enough and shipped a hole: `TitleOnly` falls back to
/// the numeral when a title is blank (`title.or(numbered)`), and a Book's scheme is
/// hardcoded rather than read from the preset, so it never passed through a clamp at all.
/// An untitled book or chapter therefore still printed "Book 1" / "Chapter 3" into the
/// exported file of a manuscript whose writer had switched numbering off. With no numbers
/// in the map there is nothing for any scheme to fall back to, and the guarantee holds for
/// every level and every scheme by construction.
fn manuscript_numbers(req: &RenderRequest) -> HashMap<u64, Numbered> {
    if !req.gathered.work.number_chapters {
        return HashMap::new();
    }
    numbering::number_map(
        &crate::item_metas(req.gathered),
        NumberingRules {
            part_resets_chapter: req.gathered.work.part_resets_chapter,
        },
    )
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
    contents
        .iter()
        .find(|c| c.role == role && c.activated && !c.data.is_empty())
        .map(|c| c.data.as_str())
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
                // Explicit, because `Work` derives `Default` and `bool::default()` is
                // `false` — a fixture leaning on `..Default::default()` here would test an
                // *unnumbered* manuscript while claiming to test an ordinary one. Same
                // trap `new_work_uc` guards against for real projects.
                number_chapters: true,
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

    // ── pagination ──

    /// Every chapter opens a page. The rule that makes a manuscript read as chapters
    /// rather than as one unbroken column, and the one Shunn requires outright.
    #[test]
    fn each_chapter_opens_a_new_page() {
        let g = flat_book();
        let p = preset("manuscript-shunn");
        assert!(p.chapter_starts_page);
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        assert!(
            dj.contains("page_break_before=true"),
            "the chapter must open a page: {dj}"
        );
    }

    /// …and turning it off really turns it off, rather than being a knob that reads well
    /// and does nothing.
    #[test]
    fn a_preset_that_declines_page_breaks_gets_none() {
        let g = flat_book();
        let p = Preset {
            book_starts_page: false,
            part_starts_page: false,
            chapter_starts_page: false,
            paratext_starts_page: false,
            book_title_page: false,
            ..preset("neutral")
        };
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        assert!(!dj.contains("page_break_before"), "{dj}");
    }

    /// A break on the very first block would open the document on a blank page in the
    /// formats that take it literally. Exporting one chapter on its own is exactly that
    /// case, and it is the common one — "Export Chapter" from the binder.
    #[test]
    fn the_first_block_of_an_export_never_carries_a_break() {
        let g = flat_book();
        let p = Preset {
            book_title_page: false,
            ..preset("manuscript-shunn")
        };
        // The chapter alone: its heading is the first thing in the document.
        let dj = render_to_string(&req(&g, &[101], &p, ExportFormat::Djot)).unwrap();
        assert!(
            !dj.contains("page_break_before"),
            "nothing above it to end a page on: {dj}"
        );
    }

    /// The break rides the *structure*, not the heading text. A preset that prints no
    /// chapter heading at all still opens each chapter on its own page — the break simply
    /// lands on the chapter's first paragraph instead of on a title.
    #[test]
    fn a_headingless_chapter_still_opens_a_page_on_its_prose() {
        let g = flat_book();
        let p = Preset {
            chapter_heading: HeadingScheme::None,
            book_title_page: false,
            ..preset("manuscript-shunn")
        };
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        let brk = dj
            .find("page_break_before")
            .unwrap_or_else(|| panic!("no break in: {dj}"));
        let prose = dj.find("The wind rose").expect("the chapter prose");
        assert!(brk < prose, "the break must lead the prose: {dj}");
    }

    /// A paratext has no heading to carry a break, so its own first paragraph opens the
    /// page. A dedication sharing a page with the end of the copyright notice is not a
    /// dedication.
    #[test]
    fn a_paratext_opens_its_own_page() {
        let mut g = flat_book();
        g.binders[0].items.push(iwc(
            103,
            SR::Paratext,
            "en",
            vec![c(9, ContentRole::ParatextText, "For my mother.")],
        ));
        let p = Preset {
            book_title_page: false,
            ..preset("neutral")
        };
        let dj = render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
        let brk = dj.rfind("page_break_before").expect("a break");
        let dedication = dj.find("For my mother").expect("the paratext");
        assert!(brk < dedication, "{dj}");
    }

    /// The flowing formats keep the break too — they are files being written out, and the
    /// style already decided there is a page boundary here.
    #[test]
    fn the_flowing_formats_carry_the_break_they_were_given() {
        let g = flat_book();
        let p = Preset {
            book_title_page: false,
            ..preset("manuscript-shunn")
        };
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(
            txt.contains('\u{000C}'),
            "a form feed is a page break in a .txt: {txt:?}"
        );

        let md = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Markdown)).unwrap();
        assert!(md.contains("break-before: page"), "{md}");
    }

    /// …and a style that declines them leaves both formats clean.
    #[test]
    fn a_style_without_breaks_leaves_no_trace_in_the_flowing_formats() {
        let g = flat_book();
        let p = Preset {
            book_starts_page: false,
            part_starts_page: false,
            chapter_starts_page: false,
            paratext_starts_page: false,
            book_title_page: false,
            ..preset("neutral")
        };
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(!txt.contains('\u{000C}'), "{txt:?}");
        let md = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Markdown)).unwrap();
        assert!(!md.contains("<div"), "{md}");
    }

    /// [`flat_book`] with an epigraph on its chapter. A local variant rather than a change
    /// to the shared fixture, which a dozen other tests measure against.
    fn flat_book_with_epigraph(epi: &str) -> Gathered {
        let mut g = flat_book();
        for it in &mut g.binders[0].items {
            if it.item.id == 101 {
                it.contents.push(c(9, ContentRole::EpigraphText, epi));
            }
        }
        g
    }

    // ── where the epigraph sits ──

    /// The default is the documented convention: chapter title, then epigraph, then body.
    #[test]
    fn by_default_the_epigraph_follows_the_chapter_title() {
        let g = flat_book_with_epigraph("> A quotation.");
        let p = Preset {
            book_title_page: false,
            ..preset("neutral")
        };
        assert_eq!(p.epigraph_placement, EpigraphPlacement::AfterHeading);
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        let title = txt.find("Storms").expect("the chapter title");
        let epi = txt.find("A quotation").expect("the epigraph");
        let body = txt.find("The wind rose").expect("the prose");
        assert!(title < epi && epi < body, "{txt}");
    }

    /// …and the other placement really moves it above the title, rather than being a
    /// setting that reads well and changes nothing.
    #[test]
    fn the_other_placement_puts_the_epigraph_above_the_title() {
        let g = flat_book_with_epigraph("> A quotation.");
        let p = Preset {
            epigraph_placement: EpigraphPlacement::BeforeHeading,
            book_title_page: false,
            ..preset("neutral")
        };
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        let title = txt.find("Storms").expect("the chapter title");
        let epi = txt.find("A quotation").expect("the epigraph");
        let body = txt.find("The wind rose").expect("the prose");
        assert!(epi < title && title < body, "{txt}");
    }

    /// With the epigraph leading, *it* opens the page — and the break has to ride the
    /// quotation's own attribute line. An attribute line in front of a `>` block attaches
    /// to no block at all, so a break written there would vanish silently.
    #[test]
    fn a_leading_epigraph_carries_the_chapters_page_break() {
        let g = flat_book_with_epigraph("> A quotation.");
        let p = Preset {
            epigraph_placement: EpigraphPlacement::BeforeHeading,
            book_title_page: false,
            ..preset("manuscript-shunn")
        };
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        let brk = dj
            .find("page_break_before")
            .unwrap_or_else(|| panic!("no break in:\n{dj}"));
        let title = dj.find("Storms").expect("the chapter title");
        assert!(
            brk < title,
            "the break opens the epigraph, not the title:\n{dj}"
        );
        // Inside the quotation, on the same line as the role it shares a block with.
        let line = dj
            .lines()
            .find(|l| l.contains("page_break_before"))
            .expect("the attribute line");
        assert!(
            line.trim_start().starts_with('>'),
            "the break must sit inside the quotation, not in front of it: {line:?}"
        );
        assert!(line.contains("semantic_role=epigraph"), "{line:?}");
    }

    /// An epigraph typed as a bare paragraph has no quotation to hang the break on, so it
    /// takes it the ordinary way rather than losing it.
    #[test]
    fn a_bare_paragraph_epigraph_still_gets_its_break() {
        let g = flat_book_with_epigraph("No angle bracket here.");
        let p = Preset {
            epigraph_placement: EpigraphPlacement::BeforeHeading,
            book_title_page: false,
            ..preset("manuscript-shunn")
        };
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        let line = dj
            .lines()
            .find(|l| l.contains("page_break_before"))
            .unwrap_or_else(|| panic!("no break in:\n{dj}"));
        assert!(!line.trim_start().starts_with('>'), "{line:?}");
        assert!(
            dj.find("page_break_before") < dj.find("No angle bracket"),
            "{dj}"
        );
    }

    /// Only the first quotation opens the page: a chapter opens once, however many
    /// epigraphs it carries.
    #[test]
    fn only_the_first_quotation_carries_the_break() {
        let g = flat_book_with_epigraph("> First.\n\n> Second.");
        let p = Preset {
            epigraph_placement: EpigraphPlacement::BeforeHeading,
            book_title_page: false,
            ..preset("manuscript-shunn")
        };
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        assert_eq!(dj.matches("page_break_before").count(), 1, "{dj}");
    }

    /// …while every quotation is still marked as an epigraph. Asserted on `mark_epigraph`
    /// itself rather than on a rendered document: `render_to_string` round-trips through
    /// the parser, which folds adjacent `>` groups into one frame, so the round-tripped
    /// djot carries one role marker no matter how many the compiler wrote.
    #[test]
    fn every_quotation_is_marked_but_only_the_first_takes_the_extras() {
        let (out, marked) = mark_epigraph(
            "> First.\n\n> Second.",
            &["page_break_before=true".to_string()],
        );
        assert!(marked);
        assert_eq!(out.matches("semantic_role=epigraph").count(), 2, "{out}");
        assert_eq!(out.matches("page_break_before").count(), 1, "{out}");
    }

    /// An epigraph with no `>` at all reports that it took nothing, so the caller knows to
    /// place the break itself.
    #[test]
    fn a_bare_paragraph_reports_that_it_carried_nothing() {
        let (out, marked) = mark_epigraph("Just a line.", &["page_break_before=true".to_string()]);
        assert!(!marked);
        assert!(!out.contains("page_break_before"), "{out}");
        assert!(!out.contains("semantic_role"), "{out}");
    }

    /// The *last* paratext of a run needs a break after it, not only one before. What
    /// follows front matter is usually ordinary prose — a prologue, an opening scene —
    /// with no structural opener of its own to break on, so a break-before rule alone
    /// leaves the last page of the front matter running straight into the body.
    #[test]
    fn the_body_starts_a_page_after_the_last_paratext() {
        let mut g = flat_book();
        // Front matter, then a plain Scene: exactly the shape the bug showed up in.
        g.binders[0].items.insert(
            1,
            iwc(
                103,
                SR::Paratext,
                "en",
                vec![c(9, ContentRole::ParatextText, "For my mother.")],
            ),
        );
        g.binders[0].items.insert(
            2,
            iwc(
                104,
                SR::Scene,
                "en",
                vec![c(10, ContentRole::SceneText, "The prologue opens.")],
            ),
        );
        let p = Preset {
            book_title_page: false,
            ..preset("neutral")
        };
        let dj =
            render_to_string(&req(&g, &[100, 103, 104, 101, 102], &p, ExportFormat::Djot)).unwrap();
        let dedication = dj.find("For my mother").expect("the paratext");
        let prologue = dj.find("The prologue opens").expect("the scene after it");
        let after = dj[dedication..prologue]
            .find("page_break_before")
            .unwrap_or_else(|| panic!("nothing breaks between them:\n{dj}"));
        let _ = after;
    }

    // ── the title page ──

    /// The title is centred and dropped down the page, not flush at the top left. This is
    /// the whole visible difference between a title page and a first line.
    #[test]
    fn the_title_page_is_centred_and_dropped_down_the_page() {
        let g = flat_book();
        let p = preset("manuscript-shunn");
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        let title_line = dj
            .lines()
            .position(|l| l.contains("My Novel"))
            .expect("the title");
        let attrs = dj.lines().nth(title_line.saturating_sub(1)).unwrap_or("");
        assert!(
            attrs.contains("alignment=center"),
            "attrs were {attrs:?}\n{dj}"
        );
        assert!(attrs.contains("top_margin="), "attrs were {attrs:?}\n{dj}");
    }

    /// The body starts on a page of its own. Without this the first chapter runs on
    /// underneath the byline, which is the complaint that started all of this.
    #[test]
    fn the_body_breaks_away_from_the_title_page() {
        let g = flat_book();
        let p = preset("manuscript-shunn");
        let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
        let title = dj.find("My Novel").expect("the title");
        let brk = dj
            .find("page_break_before")
            .expect("a break after the title page");
        assert!(brk > title, "the break belongs below the title page: {dj}");
        // …and the title page itself never carries one: there is no page above it.
        assert!(
            !dj[..title].contains("page_break_before"),
            "nothing may break above the title: {dj}"
        );
    }

    /// Shunn puts the rounded word count at the top right, and it is the first thing an
    /// editor looks at.
    #[test]
    fn a_submission_title_page_carries_the_rounded_word_count() {
        let g = flat_book();
        let p = preset("manuscript-shunn");
        assert!(p.title_page_word_count);
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        // The fixture is a dozen words, so Shunn's under-10k rule rounds to 100.
        assert!(txt.contains("about 100 words"), "{txt}");
        assert!(
            txt.find("about 100 words") < txt.find("My Novel"),
            "the count sits above the title: {txt}"
        );
    }

    /// A trade title page carries the title and the byline and nothing else — no editor
    /// is reading it, and a word count on a finished book is noise.
    #[test]
    fn a_trade_title_page_has_no_word_count() {
        let g = flat_book();
        let p = Preset {
            book_title_page: true,
            title_page_word_count: false,
            ..preset("neutral")
        };
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(!txt.contains("words"), "{txt}");
        assert!(txt.contains("My Novel"), "{txt}");
    }

    /// The byline preposition is generated furniture, so it localizes; the name never does.
    #[test]
    fn the_byline_is_localized_and_the_name_is_not() {
        let g = flat_book();
        let p = Preset {
            heading_language: HeadingLanguage::Fixed("fr".into()),
            ..preset("manuscript-shunn")
        };
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("par A. Writer"), "{txt}");
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

    /// The PDF arm carries an epigraph through Typst. It renders as a `#quote(block: true)`
    /// like any blockquote, so what needs pinning is that the *whole* path survives it:
    /// the epigraph reaches Typst, Typst compiles it, and the file is real. A malformed
    /// block would fail the Typst compile rather than quietly drop the quotation, so a
    /// PDF that exists and is larger than the same book without one is the honest signal
    /// available from this side of the boundary.
    #[cfg(feature = "pdf")]
    #[test]
    fn pdf_export_carries_an_epigraph() {
        let with = book_with_epigraph();
        let without = {
            let mut g = book_with_epigraph();
            g.binders[0].items[1]
                .contents
                .retain(|c| c.role != ContentRole::EpigraphText);
            g
        };
        let p = preset("manuscript-shunn");

        let render_bytes = |g: &Gathered, tag: &str| {
            let path =
                std::env::temp_dir().join(format!("skrib-epi-{tag}-{}.pdf", std::process::id()));
            render_to_file(
                &req(g, &[100, 101], &p, ExportFormat::Pdf),
                &path,
                &|_| {},
                &AtomicBool::new(false),
            )
            .unwrap_or_else(|e| panic!("{tag}: {e:#}"));
            let bytes = std::fs::read(&path).unwrap();
            let _ = std::fs::remove_file(&path);
            bytes
        };

        let a = render_bytes(&with, "with");
        let b = render_bytes(&without, "without");
        assert!(a.starts_with(b"%PDF-"), "valid PDF magic bytes");
        assert!(
            a.len() > b.len(),
            "the epigraph must reach the page: {} bytes with, {} without",
            a.len(),
            b.len()
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
    // Paratexts
    // ─────────────────────────────────────────────────────────────────────────

    /// A chapter, a paratext between it and the next, and a second chapter — the shape
    /// that proves a paratext neither takes a number nor disturbs the ones around it.
    fn book_with_paratext() -> Gathered {
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
                    vec![c(2, ContentRole::SceneText, "The wind rose.")],
                ),
                ItemWithContents {
                    item: BinderItem {
                        id: 102,
                        role: BinderItemRole::Item,
                        sub_role: SR::Paratext,
                        title: "Acknowledgements".into(),
                        dict_language: language::parse_legacy_list("en"),
                        is_exportable: true,
                        activated: true,
                        ..Default::default()
                    },
                    contents: vec![c(
                        3,
                        ContentRole::ParatextText,
                        "With thanks to the archivists.",
                    )],
                },
                iwc(
                    103,
                    SR::ChapterScene,
                    "en",
                    vec![c(4, ContentRole::SceneText, "She walked on.")],
                ),
            ],
            "en",
        )
    }

    /// A paratext exports its **prose and nothing else**. Its title is a binder label the
    /// writer chose to find it by — "Copyright", "front matter (draft)" — not a line of
    /// the book, so writing it into the export would put the writer's private filing
    /// vocabulary on the page. A heading, if one is wanted, is written in the prose where
    /// the writer controls its wording.
    #[test]
    fn a_paratext_exports_its_prose_and_not_its_title() {
        let g = book_with_paratext();
        let p = preset("neutral");
        let out =
            render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
        assert!(
            out.contains("With thanks to the archivists."),
            "prose: {out}"
        );
        assert!(
            !out.contains("Acknowledgements"),
            "the binder label must not reach the book: {out}"
        );
    }

    /// The chapters either side of an interleaved paratext keep their own numbering.
    #[test]
    fn a_paratext_between_chapters_does_not_renumber_them() {
        let g = book_with_paratext();
        let p = preset("neutral");
        let out =
            render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
        assert!(out.contains("Chapter 1"), "{out}");
        assert!(out.contains("Chapter 2"), "{out}");
        assert!(!out.contains("Chapter 3"), "only two chapters exist: {out}");
    }

    /// A paratext is the author's, but it is not the manuscript. Its words must not move
    /// the count, or every pace goal in the project drifts by the length of the front
    /// matter with nothing looking wrong.
    #[test]
    fn a_paratext_adds_no_words_to_the_manuscript() {
        let p = preset("neutral");
        let words = |g: &Gathered, include: &[u64]| {
            assemble(
                &req(g, include, &p, ExportFormat::Djot),
                &|_| {},
                &AtomicBool::new(false),
            )
            .unwrap()
            .1
            .words
        };
        let g = book_with_paratext();
        assert_eq!(
            words(&g, &[100, 101, 102, 103]),
            words(&g, &[100, 101, 103]),
            "the paratext must not be counted"
        );
    }

    /// The preset can leave it out — the clean-submission case, where an editor wants the
    /// manuscript and not the acknowledgements.
    #[test]
    fn the_preset_can_drop_a_paratext() {
        let g = book_with_paratext();
        let mut p = preset("neutral");
        assert!(p.include_paratexts, "shipped on by default");

        p.include_paratexts = false;
        let out =
            render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
        assert!(!out.contains("Acknowledgements"), "dropped: {out}");
        assert!(!out.contains("archivists"), "body dropped too: {out}");
        assert!(
            out.contains("The wind rose."),
            "the manuscript stays: {out}"
        );
    }

    /// A preset saved before the field existed must keep its paratexts, for the same
    /// reason `include_epigraphs` must: a bare serde default reads absence as `false`.
    #[test]
    fn a_preset_saved_before_paratexts_still_keeps_them() {
        let mut v = serde_json::to_value(preset("neutral")).unwrap();
        let obj = v.as_object_mut().unwrap();
        assert!(
            obj.remove("include_paratexts").is_some(),
            "field must serialize"
        );
        let old: Preset = serde_json::from_value(v).expect("an older preset must still load");
        assert!(old.include_paratexts, "absence must read as on");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Heading numbers
    // ─────────────────────────────────────────────────────────────────────────

    /// A five-chapter book, of which only the fifth is exported.
    fn five_chapter_book() -> Gathered {
        let mut items = vec![iwc(
            100,
            SR::BookBegin,
            "en",
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        )];
        for n in 1..=5u64 {
            items.push(iwc(
                100 + n,
                SR::ChapterScene,
                "en",
                vec![
                    c(200 + n, ContentRole::ChapterTitle, &format!("Chapter {n}")),
                    c(300 + n, ContentRole::SceneText, &format!("Scene {n}.")),
                ],
            ));
        }
        gathered(items, "en")
    }

    /// Exporting one chapter must report the number it carries in the manuscript, not its
    /// position in the export. A writer sending chapter five to a reader was handed a
    /// document that called itself chapter one.
    #[test]
    fn a_scoped_export_numbers_a_chapter_as_the_book_does() {
        let g = five_chapter_book();
        let p = preset("neutral");
        // Exactly what `ScopeKind::Chapter` resolves for the fifth chapter.
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        assert!(
            out.contains("Chapter 5"),
            "the fifth chapter must be numbered five: {out}"
        );
        assert!(
            !out.contains("Chapter 1"),
            "and must not be renumbered from one: {out}"
        );
    }

    /// A title that already says the number must not have it prepended again — the exact
    /// shape the bug report showed, "Chapter 1 — Chapter 5".
    #[test]
    fn a_number_is_not_repeated_by_a_title_that_already_carries_it() {
        let g = five_chapter_book();
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        assert!(
            !out.contains("Chapter 5 — Chapter 5"),
            "the number must not appear twice: {out}"
        );
        let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
        assert_eq!(heading, "# Chapter 5", "got {heading:?}");
    }

    /// A real title still gets the number in front of it — the de-duplication above must
    /// not swallow titles generally.
    #[test]
    fn a_real_title_still_follows_its_number() {
        let mut g = five_chapter_book();
        g.binders[0].items[5].contents[0].data = "The Storm".to_string();
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        assert!(out.contains("Chapter 5 — The Storm"), "{out}");
    }

    /// A whole-project export is unchanged: its first row *is* the first row, so nothing
    /// is seeded and the chapters read 1..5 exactly as before.
    #[test]
    fn a_full_export_still_numbers_from_one() {
        let g = five_chapter_book();
        let p = preset("neutral");
        let out = render_to_string(&req(
            &g,
            &[100, 101, 102, 103, 104, 105],
            &p,
            ExportFormat::Djot,
        ))
        .unwrap();
        for n in 1..=5 {
            assert!(
                out.contains(&format!("Chapter {n}")),
                "chapter {n} missing: {out}"
            );
        }
    }

    /// Blank every chapter title in `five_chapter_book`.
    ///
    /// That fixture titles its chapters the literal "Chapter 1".."Chapter 5" — deliberately,
    /// because that is what this app's own new-project template writes and what the
    /// redundancy guard exists to collapse. It is the wrong fixture for asserting *about
    /// numerals*, though: "Chapter 4 — Chapter 5" contains the substring "Chapter 5" as a
    /// title, so a test checking "the numeral 5 is gone" would read a title as a number.
    fn untitle_chapters(g: &mut Gathered) {
        for item in g.binders[0].items.iter_mut() {
            for c in item.contents.iter_mut() {
                if c.role == ContentRole::ChapterTitle {
                    c.data.clear();
                }
            }
        }
    }

    /// **The seed-vs-selection bug.** The old design counted twice: incrementally over the
    /// rows an export had already been filtered to, and again — unfiltered — to seed a
    /// scoped export. Mark chapter three non-exportable and the two answered differently
    /// for the *same* chapter: a full book export called chapter five "Chapter 4", while
    /// exporting that chapter alone called it "Chapter 5".
    ///
    /// Now there is one pass over the whole manuscript, so both agree. Which answer they
    /// agree *on* is the second half of the fix: a chapter the writer took out of the book
    /// holds no number and leaves no gap, so five chapters minus one read 1..4.
    #[test]
    fn a_non_exportable_chapter_cannot_make_two_exports_disagree() {
        let mut g = five_chapter_book();
        untitle_chapters(&mut g);
        g.binders[0].items[3].item.is_exportable = false; // the third chapter (id 103)
        let p = preset("neutral");

        let heading_of = |out: &str| {
            out.lines()
                .find(|l| l.starts_with('#'))
                .unwrap_or_default()
                .to_string()
        };

        // Scoped: chapter five on its own.
        let scoped = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        // Full: every exportable chapter. 103 is absent, exactly as `push_swept` builds it.
        let full =
            render_to_string(&req(&g, &[100, 101, 102, 104, 105], &p, ExportFormat::Djot)).unwrap();

        assert_eq!(
            heading_of(&scoped),
            "# Chapter 4",
            "the scoped export must not count a chapter that is not in the book: {scoped}"
        );
        assert!(
            full.contains("Chapter 4") && !full.contains("Chapter 5"),
            "the full export numbers the survivors 1..4 with no gap: {full}"
        );
    }

    /// A prologue must not take "Chapter 1" from the real first chapter — and must not
    /// print a number of its own either. It keeps its title, its prose and its heading.
    #[test]
    fn an_unnumbered_chapter_neither_prints_nor_consumes_a_number() {
        let mut g = five_chapter_book();
        untitle_chapters(&mut g);
        // The first chapter becomes the prologue, titled as one.
        g.binders[0].items[1].item.exclude_from_numbering = true;
        g.binders[0].items[1].contents[0].data = "Prologue".to_string();
        let p = preset("neutral");
        let out = render_to_string(&req(
            &g,
            &[100, 101, 102, 103, 104, 105],
            &p,
            ExportFormat::Djot,
        ))
        .unwrap();

        assert!(
            out.contains("# Prologue"),
            "the prologue keeps its own heading: {out}"
        );
        assert!(
            !out.contains("Chapter 1 — Prologue"),
            "and carries no numeral of its own: {out}"
        );
        // The chapter after it is chapter one, and the last is chapter four.
        assert!(
            out.contains("Chapter 1"),
            "the chapter after a prologue is chapter one: {out}"
        );
        assert!(
            out.contains("Chapter 4") && !out.contains("Chapter 5"),
            "…and the rest shift down with it: {out}"
        );
    }

    /// `Work.number_chapters = false` has to reach the exported file, not merely the UI.
    /// The style still asks for `NumberAndTitle`; the manuscript overrides it.
    #[test]
    fn a_work_with_numbering_off_exports_titles_without_numerals() {
        let mut g = five_chapter_book();
        g.work.number_chapters = false;
        // Give the chapters real titles, so there is something left once numbers go.
        for n in 1..=5usize {
            g.binders[0].items[n].contents[0].data = format!("Title {n}");
        }
        let p = preset("neutral");
        assert_eq!(p.chapter_heading, HeadingScheme::NumberAndTitle);
        let out = render_to_string(&req(
            &g,
            &[100, 101, 102, 103, 104, 105],
            &p,
            ExportFormat::Djot,
        ))
        .unwrap();
        assert!(out.contains("# Title 1"), "titles survive: {out}");
        assert!(
            !out.contains("Chapter 1") && !out.contains(" — "),
            "no numeral and no separator anywhere: {out}"
        );
    }

    /// **The hole the first attempt at "numbering off" left open.**
    ///
    /// Gating only the *schemes* was not enough. `TitleOnly` falls back to the numeral when
    /// a title is blank, and a Book's scheme is hardcoded rather than read from the preset,
    /// so it never passed through the clamp at all — an untitled book or chapter still
    /// printed "Book 1" / "Chapter 3" into a manuscript whose writer had switched numbering
    /// off. The gate now lives at the number map, so no scheme has anything to fall back to.
    #[test]
    fn numbering_off_prints_no_numeral_even_for_untitled_rows() {
        let mut g = five_chapter_book();
        untitle_chapters(&mut g);
        // …and an untitled book, which is the case that reached the exporter unclamped.
        for c in g.binders[0].items[0].contents.iter_mut() {
            c.data.clear();
        }
        g.work.number_chapters = false;
        let ids = [100, 101, 102, 103, 104, 105];

        for scheme in [
            HeadingScheme::Numbered,
            HeadingScheme::TitleOnly,
            HeadingScheme::NumberAndTitle,
        ] {
            let mut p = preset("neutral");
            p.chapter_heading = scheme;
            p.part_heading = scheme;
            // Force the Book opener down the heading path rather than the title page.
            p.book_title_page = false;
            let out = render_to_string(&req(&g, &ids, &p, ExportFormat::Djot)).unwrap();
            for word in ["Chapter", "Part", "Book"] {
                assert!(
                    !out.contains(word),
                    "{scheme:?} leaked a generated {word} into an unnumbered manuscript: {out}"
                );
            }
        }
    }

    /// The converse, so the gate cannot be "fixed" by simply never numbering: with the
    /// switch on, an untitled chapter is still opened by its number.
    #[test]
    fn numbering_on_still_names_an_untitled_chapter_by_its_number() {
        let mut g = five_chapter_book();
        untitle_chapters(&mut g);
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
        assert_eq!(heading, "# Chapter 5", "got {heading:?}");
    }

    /// `part_resets_chapter` is a `Work` setting, and it works.
    #[test]
    fn a_part_restarts_chapters_when_the_work_asks_it_to() {
        let mut g = gathered(
            vec![
                iwc(100, SR::BookBegin, "en", vec![]),
                iwc(101, SR::Part, "en", vec![]),
                iwc(
                    102,
                    SR::ChapterScene,
                    "en",
                    vec![c(1, ContentRole::SceneText, "A.")],
                ),
                iwc(
                    103,
                    SR::ChapterScene,
                    "en",
                    vec![c(2, ContentRole::SceneText, "B.")],
                ),
                iwc(104, SR::Part, "en", vec![]),
                iwc(
                    105,
                    SR::ChapterScene,
                    "en",
                    vec![c(3, ContentRole::SceneText, "C.")],
                ),
            ],
            "en",
        );
        let p = preset("neutral");
        let ids = [100, 101, 102, 103, 104, 105];

        // Default: chapters run on across the part boundary.
        let out = render_to_string(&req(&g, &ids, &p, ExportFormat::Djot)).unwrap();
        assert!(out.contains("Chapter 3"), "continuous by default: {out}");

        // Opted in: the second part opens on chapter one again.
        g.work.part_resets_chapter = true;
        let out = render_to_string(&req(&g, &ids, &p, ExportFormat::Djot)).unwrap();
        assert!(!out.contains("Chapter 3"), "chapters restart: {out}");
        assert!(out.contains("Part 2"), "…but parts do not: {out}");
    }

    /// A title of nothing but spaces is no title, not a title made of spaces. It used to
    /// reach the composer and render "Chapter 5 —    ", dangling dash and all.
    #[test]
    fn a_blank_title_does_not_leave_a_dangling_separator() {
        let mut g = five_chapter_book();
        g.binders[0].items[5].contents[0].data = "   ".to_string();
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
        assert_eq!(heading, "# Chapter 5", "got {heading:?}");
    }

    /// The normalized guard, end to end: the shapes the old byte-comparison let through.
    #[test]
    fn a_title_restating_its_number_is_collapsed_however_it_is_spelled() {
        for title in ["chapter 5", "Chapter 5.", "Chapter\u{00A0}5", "5"] {
            let mut g = five_chapter_book();
            g.binders[0].items[5].contents[0].data = title.to_string();
            let p = preset("neutral");
            let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
            let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
            assert_eq!(heading, "# Chapter 5", "title {title:?} gave {heading:?}");
        }
    }

    /// …and a title naming a *different* number is left visibly doubled on purpose. It is
    /// not a duplicate, it is the writer's own count disagreeing with where the row now
    /// sits, and hiding half of it would hide the disagreement.
    #[test]
    fn a_title_naming_a_different_number_is_left_visible() {
        let mut g = five_chapter_book();
        g.binders[0].items[5].contents[0].data = "Chapter 3".to_string();
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        assert!(out.contains("Chapter 5 — Chapter 3"), "{out}");
    }

    /// LaTeX must not print its own counter in front of a heading this compiler already
    /// numbered. `article`'s default `secnumdepth` of 3 numbers `\section`, so a scoped
    /// chapter export rendered "1  Chapter 5" — the export's local counter and the
    /// manuscript's real number, disagreeing, side by side. The Typst backend has always
    /// suppressed its own numbering for exactly this reason; LaTeX now does too.
    #[test]
    fn latex_does_not_number_a_heading_this_compiler_already_numbered() {
        let mut g = five_chapter_book();
        untitle_chapters(&mut g);
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Latex)).unwrap();
        assert!(
            out.contains("\\setcounter{secnumdepth}{-1}"),
            "the preamble must suppress LaTeX's own numbering: {out}"
        );
        assert!(
            out.contains("Chapter 5"),
            "…while the manuscript's own number survives: {out}"
        );
    }

    /// The separator is the style's to choose.
    #[test]
    fn the_heading_separator_comes_from_the_preset() {
        let mut g = five_chapter_book();
        g.binders[0].items[5].contents[0].data = "The Storm".to_string();
        let mut p = preset("neutral");
        p.heading_separator = ": ".to_string();
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        assert!(out.contains("Chapter 5: The Storm"), "{out}");
    }

    /// A second book restarts its chapter numbering; a part inside one book does not.
    /// Trade practice runs chapters continuously across the parts of a book.
    #[test]
    fn a_new_book_restarts_chapters_but_a_new_part_does_not() {
        let g = gathered(
            vec![
                iwc(
                    100,
                    SR::BookBegin,
                    "en",
                    vec![c(1, ContentRole::BookTitle, "One")],
                ),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![c(2, ContentRole::SceneText, "a")],
                ),
                iwc(
                    102,
                    SR::Part,
                    "en",
                    vec![c(3, ContentRole::PartTitle, "Second Part")],
                ),
                iwc(
                    103,
                    SR::ChapterScene,
                    "en",
                    vec![c(4, ContentRole::SceneText, "b")],
                ),
                iwc(
                    104,
                    SR::BookBegin,
                    "en",
                    vec![c(5, ContentRole::BookTitle, "Two")],
                ),
                iwc(
                    105,
                    SR::ChapterScene,
                    "en",
                    vec![c(6, ContentRole::SceneText, "c")],
                ),
            ],
            "en",
        );
        let p = preset("neutral");

        // The chapter after the part is the book's second, not its first.
        let after_part = render_to_string(&req(&g, &[103], &p, ExportFormat::Djot)).unwrap();
        assert!(
            after_part.contains("Chapter 2"),
            "a part must not restart chapters: {after_part}"
        );

        // The chapter in the second book is that book's first.
        let second_book = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        assert!(
            second_book.contains("Chapter 1"),
            "a new book must restart chapters: {second_book}"
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
        let attrs = before
            .rfind('{')
            .expect("an attribute block before the prose");
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

    /// The end of the chain: an epigraph must reach HTML as marked-up front matter, not
    /// as an anonymous blockquote. This is what the whole `semantic_role` path exists for,
    /// and it is the only test that exercises compiler → djot → document → writer whole.
    #[test]
    fn an_epigraph_reaches_html_as_semantic_markup() {
        let g = book_with_epigraph();
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Html)).unwrap();
        assert!(
            html.contains(r#"epub:type="epigraph""#),
            "the epigraph must be marked: {html}"
        );
        assert!(
            html.contains(r#"role="doc-epigraph""#),
            "and reachable by assistive technology: {html}"
        );
        assert!(html.contains("Salt is the only"), "with its text: {html}");
    }

    /// The marker is the compiler's doing, so a scene's own blockquote — a quoted letter,
    /// a diary page — must not acquire it.
    #[test]
    fn a_quotation_inside_scene_prose_is_not_marked_as_an_epigraph() {
        let g = gathered(
            vec![iwc(
                300,
                SR::Scene,
                "en",
                vec![c(
                    1,
                    ContentRole::SceneText,
                    "She unfolded it.\n\n> Come at once.\n\nThe hand was her mother's.",
                )],
            )],
            "en",
        );
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[300], &p, ExportFormat::Html)).unwrap();
        assert!(html.contains("<blockquote>"), "still a quotation: {html}");
        assert!(!html.contains("epigraph"), "but not an epigraph: {html}");
    }

    /// Several quotations on one node arrive as one marked blockquote holding them all:
    /// this parser folds `>` groups separated by a blank line into a single frame. What
    /// matters is that none of the text escapes the marked quote and the marker is not
    /// repeated — an epigraph is one piece of front matter however many quotations the
    /// author put in it.
    #[test]
    fn every_quotation_in_an_epigraph_field_is_marked() {
        let g = gathered(
            vec![iwc(
                400,
                SR::ChapterScene,
                "en",
                vec![
                    c(1, ContentRole::ChapterTitle, "Two Quotes"),
                    c(
                        2,
                        ContentRole::EpigraphText,
                        "> First quotation.\n\n> Second quotation.",
                    ),
                    c(3, ContentRole::SceneText, "Body."),
                ],
            )],
            "en",
        );
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[400], &p, ExportFormat::Html)).unwrap();
        assert_eq!(
            html.matches(r#"epub:type="epigraph""#).count(),
            1,
            "one marked quote, not one per quotation: {html}"
        );
        for quote in ["First quotation.", "Second quotation."] {
            assert!(html.contains(quote), "{quote} missing: {html}");
        }
        assert!(
            !html.contains("semantic_role"),
            "the marker must be consumed, never rendered as text: {html}"
        );
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
                    vec![c(
                        3,
                        ContentRole::SceneText,
                        "The wind rose over the hills.",
                    )],
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
        // The nearest `{` is the semantic marker, which sits inside the quote directly
        // above its text; the direction attribute is emitted by `push_prose` for the
        // block as a whole, so look for it across everything preceding the quotation.
        assert!(
            before.contains("direction=rtl"),
            "an RTL row's epigraph must be marked rtl, got: {before:?}"
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
