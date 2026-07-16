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
use text_document::{DocxExportOptions, TextDirection, TextDocument};

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
    let (doc, _) = assemble(req, &|_| {}, &AtomicBool::new(false))?;
    text_render(&doc, req.format)
}

/// Render an HTML preview (used by the UI live preview regardless of the chosen format).
pub fn render_preview_html(req: &RenderRequest) -> Result<String> {
    let (doc, _) = assemble(req, &|_| {}, &AtomicBool::new(false))?;
    Ok(doc.to_html()?)
}

/// Assemble the compiled document for the UI live preview: the *same* `TextDocument`
/// [`render_to_file`] would render (generated headings + scene breaks + prose, with
/// per-block direction), handed back so the panel can show it in a read-only editor. This
/// is why the preview and the committed export cannot diverge — one assembly path.
pub fn render_preview_document(req: &RenderRequest) -> Result<TextDocument> {
    let (doc, _) = assemble(req, &|_| {}, &AtomicBool::new(false))?;
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
    let (doc, stats) = assemble(req, progress, cancel)?;
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
            let opts = docx_options(req.preset, &req.gathered.work.title, &req.gathered.work.author_name);
            doc.to_docx_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing DOCX '{out}': {e:#}"))?;
        }
        other => return Err(anyhow!("{other:?} export is not implemented yet")),
    }
    progress(1.0);
    Ok(stats)
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
        ExportFormat::PlainText => doc.to_plain_text()?,
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
) -> Result<(TextDocument, RenderStats)> {
    let rows = flatten(req);
    let preset = req.preset;

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
    let mut last = Emitted::Nothing;

    let work_rtl = is_rtl_row(preset, req.work_lang);

    // Optional title page (front matter) for a book export.
    if preset.book_title_page && rows.iter().any(|r| r.item.sub_role.opens_book()) {
        let w = &req.gathered.work;
        if !w.title.is_empty() {
            push_heading(&mut out, 1, &w.title, work_rtl);
            last = Emitted::Heading;
        }
        if !w.author_name.is_empty() {
            push_para(&mut out, &w.author_name, work_rtl);
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
                last = Emitted::Heading;
                contributed = true;
            }
        } else if preset.include_scene_titles && row.item.sub_role.carries_scene() {
            // A plain titled Scene (never a ChapterScene — that already emitted its chapter
            // heading above): the title heading stands in for the scene break.
            let t = row.item.title.trim();
            if !t.is_empty() {
                push_heading(&mut out, scene_title_level, t, row_rtl);
                last = Emitted::Heading;
                contributed = true;
            }
        }

        // 2. The main prose (a scene's SceneText, a note's NoteText) — appended verbatim
        //    since it is already Djot.
        if let Some(role) = main_prose_role(&row.item.sub_role) {
            if let Some(prose) = content_of(row.contents, role) {
                let is_scene = row.item.sub_role.carries_scene();
                if is_scene && last == Emitted::SceneProse {
                    push_scene_break(&mut out, &preset.scene_break);
                }
                push_prose(&mut out, prose, row_rtl);
                words += prose.split_whitespace().count();
                last = if is_scene { Emitted::SceneProse } else { Emitted::OtherProse };
                contributed = true;
            }
        }

        // 3. The synopsis, if the preset keeps it.
        if preset.include_synopses {
            if let Some(syn) = content_of(row.contents, ContentRole::SynopsisText) {
                push_prose(&mut out, syn, row_rtl);
                contributed = true;
            }
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

    Ok((doc, RenderStats { items: emitted_items, words }))
}

/// Flatten the frozen tree into the included rows, in document order, each with its
/// resolved language. `include` decides membership; document order decides sequence (so a
/// Choose… selection still exports top-to-bottom). `activated` is a defensive guard — a
/// trashed item never exports even if its id reaches the set.
fn flatten<'a>(req: &'a RenderRequest) -> Vec<Row<'a>> {
    let want: std::collections::HashSet<u64> = req.include.iter().copied().collect();
    let mut rows = Vec::new();
    for bwi in &req.gathered.binders {
        let items: Vec<BinderItem> = bwi.items.iter().map(|iwc| iwc.item.clone()).collect();
        let mut langs = std::collections::HashMap::new();
        language::tags_in_binder(req.work_lang, &items, &mut langs);
        for iwc in &bwi.items {
            if want.contains(&iwc.item.id) && iwc.item.activated {
                let lang = langs
                    .get(&iwc.item.id)
                    .cloned()
                    .unwrap_or_else(|| req.work_lang.to_string());
                rows.push(Row { item: &iwc.item, contents: &iwc.contents, lang });
            }
        }
    }
    rows
}

// ─────────────────────────────────────────────────────────────────────────────
// Djot builders
// ─────────────────────────────────────────────────────────────────────────────

/// A Djot block-attribute line that sets direction, or empty. `{direction=rtl}` on the line
/// before a block is what text-document's own Djot exporter writes and its importer reads.
fn dir_attr(rtl: bool) -> &'static str {
    if rtl { "{direction=rtl}\n" } else { "" }
}

fn push_heading(out: &mut String, level: u8, text: &str, rtl: bool) {
    out.push_str(dir_attr(rtl));
    for _ in 0..level.clamp(1, 6) {
        out.push('#');
    }
    out.push(' ');
    out.push_str(text.trim());
    out.push_str("\n\n");
}

fn push_para(out: &mut String, text: &str, rtl: bool) {
    out.push_str(dir_attr(rtl));
    out.push_str(text.trim());
    out.push_str("\n\n");
}

/// Append a scene's prose. It is already Djot, so it goes in verbatim (never touched — the
/// exporter generates furniture, it does not rewrite prose). For RTL each of the scene's
/// blank-line-separated blocks gets a `{direction=rtl}` attribute.
fn push_prose(out: &mut String, djot: &str, rtl: bool) {
    let trimmed = djot.trim();
    if !rtl {
        out.push_str(trimmed);
        out.push_str("\n\n");
        return;
    }
    for block in trimmed.split("\n\n") {
        let b = block.trim();
        if b.is_empty() {
            continue;
        }
        out.push_str("{direction=rtl}\n");
        out.push_str(b);
        out.push_str("\n\n");
    }
}

fn push_scene_break(out: &mut String, sep: &SceneBreak) {
    match sep {
        // The blank line between blocks is already the gap.
        SceneBreak::None | SceneBreak::BlankLine => {}
        // Escape a leading Djot block-marker so the glyph stays literal text — an
        // unescaped `#` is a heading, `* * *` / `---` a (dropped) thematic break.
        SceneBreak::Glyph(g) => {
            out.push_str(&escape_block_leading(g.trim()));
            out.push_str("\n\n");
        }
    }
}

/// Escape a leading Djot block-special character so a one-line glyph is a plain paragraph.
fn escape_block_leading(s: &str) -> String {
    match s.chars().next() {
        Some(c) if "#*-_+>~:|=`".contains(c) || c.is_ascii_digit() => format!("\\{s}"),
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

#[derive(PartialEq, Eq, Clone, Copy)]
enum Emitted {
    Nothing,
    Heading,
    SceneProse,
    OtherProse,
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
    for role in [ContentRole::ChapterTitle, ContentRole::PartTitle, ContentRole::BookTitle] {
        if let Some(t) = content_of(row.contents, role) {
            return Some(t);
        }
    }
    (!row.item.title.is_empty()).then(|| row.item.title.as_str())
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
        HeadingScheme::TitleOnly => title_of(row).map(str::to_string).or_else(|| Some(numbered())),
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
        Content { id, activated: true, role, data: data.to_string(), ..Default::default() }
    }

    fn iwc(id: u64, sub_role: SR, lang: &str, contents: Vec<Content>) -> ItemWithContents {
        ItemWithContents {
            item: BinderItem {
                id,
                role: BinderItemRole::Item,
                sub_role,
                dict_language: lang.to_string(),
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
                dict_language: work_lang.into(),
                ..Default::default()
            },
            tags: vec![],
            dict_words: vec![],
            trash_infos: vec![],
            paces: vec![],
            progress_snapshots: vec![],
            binders: vec![BinderWithItems { binder: Binder { id: 10, ..Default::default() }, items }],
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
                iwc(100, SR::BookBegin, "en", vec![c(1, ContentRole::BookTitle, "My Novel")]),
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

    fn req<'a>(g: &'a Gathered, include: &'a [u64], p: &'a Preset, f: ExportFormat) -> RenderRequest<'a> {
        RenderRequest { gathered: g, include, preset: p, format: f, work_lang: "en", explicit_selection: false }
    }

    #[test]
    fn renders_a_flat_book_to_html_with_headings_and_prose() {
        let g = flat_book();
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        assert!(html.contains("My Novel"), "book title: {html}");
        assert!(html.contains("Chapter 1 — Storms"), "chapter heading: {html}");
        assert!(html.contains("The wind rose over the hills."), "{html}");
        assert!(html.contains("She walked on into the dark."), "{html}");
        // The prose must NOT become a heading.
        assert!(!html.contains("<h2>The wind"), "prose leaked into a heading: {html}");
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
        assert!(html.contains("Chapter 1"), "content-language heading: {html}");
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
            vec![iwc(200, SR::Scene, "ar", vec![c(9, ContentRole::SceneText, "نص عربي هنا.")])],
            "ar",
        );
        let p = preset("neutral");
        let html = render_to_string(&req(&g, &[200], &p, ExportFormat::Html)).unwrap();
        assert!(html.contains("rtl"), "RTL direction should reach the HTML: {html}");
    }

    #[test]
    fn a_glyph_scene_break_separates_two_scenes() {
        let g = flat_book();
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("###"), "scene break glyph between the two scenes: {txt}");
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
        assert!(std::fs::metadata(&path).unwrap().len() > 0, "docx should be non-empty");
        assert!(stats.items >= 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_trashed_row_in_the_include_set_is_dropped_defensively() {
        let mut g = flat_book();
        g.binders[0].items[2].item.activated = false; // scene 102 trashed
        let p = preset("neutral");
        let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("The wind rose"));
        assert!(!txt.contains("She walked on"), "trashed scene must be excluded: {txt}");
    }

    fn titled(mut iwc: ItemWithContents, title: &str) -> ItemWithContents {
        iwc.item.title = title.to_string();
        iwc
    }

    /// A book with a chapter (one scene) and a research note swept in after it.
    fn book_with_note() -> Gathered {
        gathered(
            vec![
                iwc(100, SR::BookBegin, "en", vec![c(1, ContentRole::BookTitle, "My Novel")]),
                iwc(
                    101,
                    SR::ChapterScene,
                    "en",
                    vec![
                        c(2, ContentRole::ChapterTitle, "Storms"),
                        c(3, ContentRole::SceneText, "The wind rose over the hills."),
                    ],
                ),
                iwc(102, SR::Note, "en", vec![c(4, ContentRole::NoteText, "Research: local weather.")]),
            ],
            "en",
        )
    }

    #[test]
    fn a_swept_note_is_dropped_unless_the_preset_or_an_explicit_pick_keeps_it() {
        let g = book_with_note();
        let mut p = preset("neutral"); // include_notes = false
        // Swept into a whole-book export: the note is dropped by default.
        let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(txt.contains("The wind rose"), "{txt}");
        assert!(!txt.contains("Research"), "a swept note must be dropped by default: {txt}");
        // The preset keeps notes → included.
        p.include_notes = true;
        let with = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(with.contains("Research"), "include_notes should keep the note: {with}");
        // An explicit pick (Export Note / a checked item) overrides the toggle being off.
        p.include_notes = false;
        let ids = [102u64];
        let explicit =
            RenderRequest { explicit_selection: true, ..req(&g, &ids, &p, ExportFormat::PlainText) };
        let only = render_to_string(&explicit).unwrap();
        assert!(only.contains("Research"), "an explicit note pick overrides the toggle: {only}");
    }

    #[test]
    fn scene_titles_are_emitted_only_when_the_preset_keeps_them() {
        let g = gathered(
            vec![
                titled(
                    iwc(300, SR::Scene, "en", vec![c(1, ContentRole::SceneText, "Alpha prose.")]),
                    "Opening",
                ),
                titled(
                    iwc(301, SR::Scene, "en", vec![c(2, ContentRole::SceneText, "Closing prose.")]),
                    "Ending",
                ),
            ],
            "en",
        );
        let mut p = preset("neutral"); // include_scene_titles = false
        let without = render_to_string(&req(&g, &[300, 301], &p, ExportFormat::Html)).unwrap();
        assert!(!without.contains("Opening"), "scene titles must not leak when off: {without}");
        // Turned on: the titles head each scene (h1 here — no structural levels present).
        p.include_scene_titles = true;
        let with = render_to_string(&req(&g, &[300, 301], &p, ExportFormat::Html)).unwrap();
        assert!(with.contains("Opening") && with.contains("Ending"), "scene titles: {with}");
        assert!(with.contains("<h1"), "scene titles head at h1 with no structural levels: {with}");
    }
}
