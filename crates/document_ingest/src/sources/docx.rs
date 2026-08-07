// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Office Open XML scanner — `.docx`.
//!
//! `docx_rs::read_docx` does the container and the XML; this walks the typed tree
//! it returns and speaks [`rich`]'s vocabulary back. Everything past "here are the
//! paragraphs" is shared with the ODT scanner.
//!
//! ## Heading depth: outline level, then style id, then nothing
//!
//! **`w:outlineLvl` first**, because it is the only signal that means the same
//! thing in every locale. Word's style *names* are localized — "Heading 1",
//! "Titre 1", "Überschrift 1" — so matching them is a bug waiting for its first
//! French manuscript. The style **id** (`Heading1`) is stable and is asked second,
//! along with the outline level the style itself declares, following `w:basedOn`.
//! `w:outlineLvl w:val="9"` is OOXML for *body text*, not "level ten", and is read
//! as no heading at all.
//!
//! When a paragraph is plainly styled as a heading and none of that yields a depth,
//! it becomes prose and says so through [`ImportDiagnostic::UnknownStyleLevel`].
//! Guessing a depth reshapes the book silently; prose plus a warning does not.
//!
//! ## Tracked changes are accepted, and the writer is told
//!
//! `w:ins` and `w:moveTo` are the text as it stands, so they are kept; `w:del` and
//! `w:moveFrom` are text somebody removed, so they are dropped. That is what "the
//! final document" means, and it is what Word shows by default. But a manuscript
//! arriving mid-revision is worth one sentence
//! ([`ImportDiagnostic::TrackedChangesFlattened`]): a writer who finds a sentence
//! they remember rejecting sitting in their book should have been warned, not
//! surprised.
//!
//! ## Comments, including their threads
//!
//! `docx_rs` resolves `w15:commentsEx`'s `w15:paraIdParent` into
//! `Comment::parent_comment_id` while reading, so Word's one-level threading
//! arrives already threaded — and one level is exactly what Skribisto's card is
//! (one opening comment, a **flat** list of replies). The two shapes match with
//! nothing to reconcile.
//!
//! **A reply has no range of its own**, and `w:commentReference` is not part of the
//! typed tree, so a reply is never met while walking the body. Every comment the
//! walk did not reach is therefore swept up afterwards and attached to its parent's
//! thread. One that has no parent either is kept as a comment on its row, with
//! [`ImportDiagnostic::CommentUnanchored`] — an editor's note is the most valuable
//! thing in the file and is never dropped for want of an anchor.
//!
//! ## The supplementary pass, and why it is not paranoia
//!
//! Two constructs a manuscript genuinely uses are invisible to the typed reader, and
//! [`RawScan`] reads them straight out of `word/document.xml`:
//!
//! * **A comment anchored to a point rather than a range.** LibreOffice's `.docx`
//!   export writes these as a bare `w:commentReference` with no
//!   `w:commentRangeStart` at all — verified by converting a document with one. Left
//!   to the typed tree, every such comment would land on its row as a whole and
//!   report itself unanchored. Read here, it becomes a comment on the paragraph the
//!   reference sat in, which is what it is.
//! * **A horizontal rule** — an empty paragraph carrying a bottom border and nothing
//!   else. `ParagraphBorders` keeps every side private, so this cannot be asked of
//!   the typed tree; and refusing to read it would make DOCX unable to carry a break
//!   its writer can see, for the same reason the ODT scanner reads ODF's spelling.
//!
//! It is one extra read of one zip member, and it answers both questions from the
//! same parse. The alternative was two silent losses.

use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use docx_rs::{
    Bold, Comment, CommentChild, CommentRangeEnd, DeleteChild, DocumentChild, Docx, DrawingData,
    Insert, InsertChild, Italic, MoveToChild, Paragraph, ParagraphChild, ParagraphProperty,
    Run as DocxRun, RunChild, RunProperty, Style, Table, TableCellContent, TableChild,
    TableRowChild, Underline,
};

use crate::block::{SourceAnnotationReply, SourceBlock, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::scanner::SourceScanner;
use crate::sources::rich::{
    ParagraphKind, RichAnnotation, RichBlock, RichDocument, Run, RunStyle, assemble,
};

pub struct DocxScanner;

impl SourceScanner for DocxScanner {
    fn extensions(&self) -> &[&str] {
        &["docx"]
    }

    fn format_name(&self) -> &'static str {
        "office-open-xml"
    }

    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        let docx = docx_rs::read_docx(bytes)
            .map_err(|e| anyhow!("not a readable Word document: {e:?}"))?;

        // No document title is read. `docx_rs` keeps `CoreProps` private with no
        // accessor, and opening the container a second time to reach
        // `docProps/core.xml` would be a lot of machinery for a field Word leaves
        // empty in almost every manuscript. `SourceDocument::effective_title` falls
        // back to the first heading, which is the better answer for a book anyway.
        // (The ODT scanner *can* read `dc:title`, so the two differ here.)
        let mut doc = SourceDocument::new(display_name, origin);

        let styles = StyleTable::new(&docx);
        let comments = CommentTable::new(&docx);
        let raw = RawScan::read(bytes).unwrap_or_default();
        let mut walker = Walker::new(&styles, &comments, &raw, origin);
        walker.walk(&docx.document.children);
        let rich = walker.finish();

        doc.diagnostics.extend(walker.diagnostics);
        assemble(&rich, &mut doc)?;

        if doc.blocks.is_empty() {
            doc.diagnostics.push(ImportDiagnostic::EmptyFile {
                path: origin.to_string(),
            });
        } else if !doc
            .blocks
            .iter()
            .any(|b| matches!(b, SourceBlock::Heading { .. }))
        {
            doc.diagnostics.push(ImportDiagnostic::NoHeadings {
                path: origin.to_string(),
            });
        }
        Ok(doc)
    }
}

// ---------------------------------------------------------------------------
// Styles
// ---------------------------------------------------------------------------

struct StyleTable {
    by_id: HashMap<String, Style>,
}

impl StyleTable {
    fn new(docx: &Docx) -> Self {
        StyleTable {
            by_id: docx
                .styles
                .styles
                .iter()
                .map(|s| (s.style_id.clone(), s.clone()))
                .collect(),
        }
    }

    /// The heading depth this paragraph is at, or `None` for body text.
    ///
    /// Returns `Err(style_id)` when the paragraph is plainly styled as a heading but
    /// no depth could be derived — the caller reports that rather than guessing.
    fn heading_level(&self, property: &ParagraphProperty) -> HeadingVerdict {
        if let Some(level) = outline_level(property) {
            return HeadingVerdict::Heading(level);
        }
        let Some(style_id) = property.style.as_ref().map(|s| s.val.clone()) else {
            return HeadingVerdict::Body;
        };

        // The style's own outline level, following `w:basedOn`.
        let mut current = Some(style_id.clone());
        let mut guard = 0;
        while let Some(id) = current {
            let Some(style) = self.by_id.get(&id) else {
                break;
            };
            if let Some(level) = outline_level(&style.paragraph_property) {
                return HeadingVerdict::Heading(level);
            }
            current = self.based_on(style);
            guard += 1;
            if guard > 32 {
                break;
            }
        }

        // The style id itself. Stable across locales, unlike the style *name*.
        match level_from_style_id(&style_id) {
            Some(level) => HeadingVerdict::Heading(level),
            None if looks_like_a_heading(&style_id) => HeadingVerdict::Unknown(style_id),
            None => HeadingVerdict::Body,
        }
    }

    /// Character formatting for a run: the paragraph style's, then the paragraph's
    /// own, then the run's character style, then the run's direct formatting.
    fn run_style(&self, paragraph: &ParagraphProperty, run: &RunProperty) -> RunStyle {
        let mut style = RunStyle::default();
        if let Some(id) = paragraph.style.as_ref().map(|s| &s.val) {
            self.apply_style_chain(id, &mut style);
        }
        apply_run_property(&paragraph.run_property, &mut style);
        if let Some(id) = run.style.as_ref().map(|s| &s.val) {
            self.apply_style_chain(id, &mut style);
        }
        apply_run_property(run, &mut style);
        style
    }

    fn apply_style_chain(&self, id: &str, style: &mut RunStyle) {
        // Root-first so a derived style overrides what it is based on.
        let mut chain: Vec<&Style> = Vec::new();
        let mut current = Some(id.to_string());
        let mut guard = 0;
        while let Some(id) = current {
            let Some(found) = self.by_id.get(&id) else {
                break;
            };
            chain.push(found);
            current = self.based_on(found);
            guard += 1;
            if guard > 32 {
                break;
            }
        }
        for found in chain.iter().rev() {
            apply_run_property(&found.run_property, style);
        }
    }

    /// The style id a style is based on.
    ///
    /// `BasedOn` keeps its value private with no accessor, so the id is recovered by
    /// comparing against a constructed twin over the ids this document actually
    /// defines. The candidate set is closed and small, and the comparison can only
    /// fail to find a match — which ends the chain — never match the wrong style.
    fn based_on(&self, style: &Style) -> Option<String> {
        let based_on = style.based_on.as_ref()?;
        self.by_id
            .keys()
            .find(|id| *based_on == docx_rs::BasedOn::new(id.as_str()))
            .cloned()
    }
}

enum HeadingVerdict {
    Heading(u8),
    Body,
    /// Styled as a heading, but at no depth this scanner could read.
    Unknown(String),
}

/// `w:outlineLvl` is 0-based, and **9 means body text** rather than level ten.
fn outline_level(property: &ParagraphProperty) -> Option<u8> {
    let v = property.outline_lvl.as_ref()?.v;
    if v >= 9 { None } else { Some(v as u8 + 1) }
}

/// `Heading1` … `Heading9`, and the spaced spelling some producers write.
fn level_from_style_id(id: &str) -> Option<u8> {
    let squashed: String = id
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .collect::<String>()
        .to_ascii_lowercase();
    let digits = squashed.strip_prefix("heading")?;
    digits.parse::<u8>().ok().filter(|n| (1..=9).contains(n))
}

fn looks_like_a_heading(id: &str) -> bool {
    let lower = id.to_ascii_lowercase();
    lower.starts_with("heading") || lower.starts_with("title") || lower.starts_with("subtitle")
}

/// `Bold`, `Italic` and `Underline` keep their value private, so they are compared
/// against a constructed twin rather than read. That is the whole public API for
/// them, and it is exact: `Bold::new()` is the "on" value and `.disable()` the "off"
/// one, so a run that explicitly turns bold *off* is distinguishable from one that
/// says nothing — which matters, because a heading style that is bold can be
/// overridden by a run inside it.
fn apply_run_property(property: &RunProperty, style: &mut RunStyle) {
    if let Some(bold) = &property.bold {
        style.bold = *bold == Bold::new();
    }
    if let Some(italic) = &property.italic {
        style.italic = *italic == Italic::new();
    }
    if let Some(strike) = &property.strike {
        style.strikethrough = strike.val;
    }
    if let Some(underline) = &property.underline {
        style.underline = *underline != Underline::new("none");
    }
}

// ---------------------------------------------------------------------------
// Comments
// ---------------------------------------------------------------------------

struct CommentMeta {
    author: String,
    created: Option<chrono::DateTime<chrono::Utc>>,
    body: String,
    resolved: bool,
    parent: Option<usize>,
}

struct CommentTable {
    /// In document order, as `comments.xml` lists them.
    order: Vec<usize>,
    by_id: HashMap<usize, CommentMeta>,
}

impl CommentTable {
    fn new(docx: &Docx) -> Self {
        // `w15:done` lives in commentsExtended, keyed by the *paragraph* id of the
        // comment's first paragraph — the same join `docx_rs` uses internally for
        // threading, and the only key the two parts share.
        let mut done_by_paragraph: HashMap<&str, bool> = HashMap::new();
        for extended in &docx.comments_extended.children {
            done_by_paragraph.insert(extended.paragraph_id.as_str(), extended.done);
        }

        let mut order = Vec::new();
        let mut by_id = HashMap::new();
        for comment in docx.comments.inner() {
            order.push(comment.id);
            by_id.insert(comment.id, meta_of(comment, &done_by_paragraph));
        }
        CommentTable { order, by_id }
    }

    fn get(&self, id: usize) -> Option<&CommentMeta> {
        self.by_id.get(&id)
    }
}

fn meta_of(comment: &Comment, done_by_paragraph: &HashMap<&str, bool>) -> CommentMeta {
    let mut body_parts: Vec<String> = Vec::new();
    let mut resolved = false;
    for child in &comment.children {
        if let CommentChild::Paragraph(paragraph) = child {
            if let Some(done) = done_by_paragraph.get(paragraph.id.as_str()) {
                resolved |= *done;
            }
            body_parts.push(paragraph_text(paragraph));
        }
    }
    CommentMeta {
        author: comment.author.clone(),
        created: parse_date(&comment.date),
        body: body_parts.join("\n").trim().to_string(),
        resolved,
        parent: comment.parent_comment_id,
    }
}

/// The plain text of a comment's own paragraph — its body, never prose.
fn paragraph_text(paragraph: &Paragraph) -> String {
    let mut out = String::new();
    collect_paragraph_text(&paragraph.children, &mut out);
    out
}

fn collect_paragraph_text(children: &[ParagraphChild], out: &mut String) {
    for child in children {
        match child {
            ParagraphChild::Run(run) => collect_run_text(run, out),
            ParagraphChild::Insert(insert) => {
                for child in &insert.children {
                    if let InsertChild::Run(run) = child {
                        collect_run_text(run, out);
                    }
                }
            }
            ParagraphChild::MoveTo(move_to) => {
                for child in &move_to.children {
                    if let MoveToChild::Run(run) = child {
                        collect_run_text(run, out);
                    }
                }
            }
            ParagraphChild::Hyperlink(link) => collect_paragraph_text(&link.children, out),
            _ => {}
        }
    }
}

fn collect_run_text(run: &DocxRun, out: &mut String) {
    for child in &run.children {
        match child {
            RunChild::Text(text) => out.push_str(&text.text),
            RunChild::Tab(_) | RunChild::PTab(_) => out.push(' '),
            _ => {}
        }
    }
}

/// `w:date` is ISO 8601 with an offset, but producers vary. Missing or unreadable
/// is not an error: the comment simply carries no date.
fn parse_date(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&chrono::Utc));
    }
    for format in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M", "%Y-%m-%d"] {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(value, format) {
            return Some(naive.and_utc());
        }
    }
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc())
}

// ---------------------------------------------------------------------------
// The supplementary pass over word/document.xml
// ---------------------------------------------------------------------------

const NS_W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

/// What the typed reader does not surface — see the module note.
///
/// Everything is keyed by **top-level paragraph ordinal**: the *n*-th `w:p` that is
/// a direct child of `w:body` is the *n*-th `DocumentChild::Paragraph`, in the same
/// order, so the two passes agree without either knowing about the other. A
/// paragraph inside a table or an `w:sdt` is deliberately not counted, on either
/// side.
#[derive(Default)]
struct RawScan {
    /// Paragraph ordinal → the point comments in it, as `(comment id, char offset)`.
    references: HashMap<usize, Vec<(usize, usize)>>,
    /// Paragraph ordinals that are a horizontal rule.
    rules: HashSet<usize>,
}

impl RawScan {
    /// Returns `None` when the member cannot be read or parsed. That is not a
    /// failure worth stopping an import for: without it the scanner behaves as it
    /// would have without this pass at all.
    fn read(bytes: &[u8]) -> Option<RawScan> {
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).ok()?;
        let xml = {
            let mut file = zip.by_name("word/document.xml").ok()?;
            let mut buffer = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut buffer).ok()?;
            String::from_utf8_lossy(&buffer).into_owned()
        };
        let document = roxmltree::Document::parse(&xml).ok()?;
        let body = document
            .descendants()
            .find(|n| n.is_element() && n.tag_name().name() == "body")?;

        let mut scan = RawScan::default();
        // Ids that carry a real range anywhere in the document. Word writes a
        // `w:commentReference` for *those* too, right after the range end, and
        // treating it as a second, point-anchored comment would double every
        // commented passage.
        let ranged: HashSet<usize> = body
            .descendants()
            .filter(|n| n.is_element() && n.tag_name().name() == "commentRangeStart")
            .filter_map(|n| n.attribute((NS_W, "id")))
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();

        for (ordinal, paragraph) in body
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "p")
            .enumerate()
        {
            if is_horizontal_rule(paragraph) {
                scan.rules.insert(ordinal);
            }
            let mut offset = 0usize;
            let mut found: Vec<(usize, usize)> = Vec::new();
            walk_raw_paragraph(paragraph, &mut offset, &mut found, &ranged);
            if !found.is_empty() {
                scan.references.insert(ordinal, found);
            }
        }
        Some(scan)
    }
}

/// An empty paragraph whose only border is at the bottom — Word's horizontal rule,
/// and what its autoformat produces from a typed `---`.
fn is_horizontal_rule(paragraph: roxmltree::Node<'_, '_>) -> bool {
    let has_text = paragraph
        .descendants()
        .any(|n| n.is_element() && n.tag_name().name() == "t" && !text_of(n).trim().is_empty());
    if has_text {
        return false;
    }
    let Some(borders) = paragraph
        .descendants()
        .find(|n| n.is_element() && n.tag_name().name() == "pBdr")
    else {
        return false;
    };
    let side = |name: &str| {
        borders
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == name)
            .and_then(|n| n.attribute((NS_W, "val")))
            .filter(|v| *v != "none" && *v != "nil")
    };
    side("bottom").is_some() && ["top", "left", "right"].iter().all(|s| side(s).is_none())
}

/// Walk one paragraph's XML counting characters exactly as the typed walk does, and
/// note where each bare `w:commentReference` sits.
///
/// `w:delText` and anything under `w:moveFrom` are skipped, because the typed walk
/// drops that text too — if the two counted differently, every comment after a
/// tracked change would be placed by an offset nobody could reproduce.
fn walk_raw_paragraph(
    node: roxmltree::Node<'_, '_>,
    offset: &mut usize,
    found: &mut Vec<(usize, usize)>,
    ranged: &HashSet<usize>,
) {
    for child in node.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "t" => *offset += text_of(child).chars().count(),
            "tab" | "ptab" => *offset += 1,
            "delText" | "moveFrom" | "instrText" | "delInstrText" => {}
            "commentReference" => {
                if let Some(id) = child
                    .attribute((NS_W, "id"))
                    .and_then(|v| v.parse::<usize>().ok())
                    && !ranged.contains(&id)
                {
                    found.push((id, *offset));
                }
            }
            _ => walk_raw_paragraph(child, offset, found, ranged),
        }
    }
}

fn text_of(node: roxmltree::Node<'_, '_>) -> String {
    node.children()
        .filter(roxmltree::Node::is_text)
        .filter_map(|n| n.text())
        .collect()
}

// ---------------------------------------------------------------------------
// The walk
// ---------------------------------------------------------------------------

struct OpenRange {
    annotation: usize,
    block: usize,
    start: usize,
}

struct Walker<'a> {
    styles: &'a StyleTable,
    comments: &'a CommentTable,
    raw: &'a RawScan,
    /// How many top-level paragraphs have been processed — the key `RawScan` uses.
    paragraph_ordinal: usize,
    origin: String,
    blocks: Vec<RichBlock>,
    annotations: Vec<RichAnnotation>,
    /// Comment id → index into `annotations`, so a reply can find its thread.
    annotation_of: HashMap<usize, usize>,
    open: HashMap<usize, OpenRange>,
    seen: HashSet<usize>,
    diagnostics: Vec<ImportDiagnostic>,
    tracked_changes: usize,
    text_boxes: usize,
    embedded_objects: usize,
    fields: usize,
    footnotes: usize,
    unknown_styles: HashSet<String>,
}

struct ParaBuild {
    kind: ParagraphKind,
    runs: Vec<Run>,
    len: usize,
}

impl<'a> Walker<'a> {
    fn new(
        styles: &'a StyleTable,
        comments: &'a CommentTable,
        raw: &'a RawScan,
        origin: &str,
    ) -> Self {
        Walker {
            styles,
            comments,
            raw,
            paragraph_ordinal: 0,
            origin: origin.to_string(),
            blocks: Vec::new(),
            annotations: Vec::new(),
            annotation_of: HashMap::new(),
            open: HashMap::new(),
            seen: HashSet::new(),
            diagnostics: Vec::new(),
            tracked_changes: 0,
            text_boxes: 0,
            embedded_objects: 0,
            fields: 0,
            footnotes: 0,
            unknown_styles: HashSet::new(),
        }
    }

    fn walk(&mut self, children: &[DocumentChild]) {
        for child in children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    let ordinal = self.paragraph_ordinal;
                    self.paragraph_ordinal += 1;
                    self.top_level_paragraph(paragraph, ordinal);
                }
                DocumentChild::Table(table) => self.table(table),
                DocumentChild::StructuredDataTag(tag) => {
                    for child in &tag.children {
                        if let docx_rs::StructuredDataTagChild::Paragraph(p) = child {
                            self.paragraph(p);
                        }
                    }
                }
                DocumentChild::CommentStart(start) => {
                    // A range opened between paragraphs rather than inside one: it
                    // belongs to whatever comes next.
                    let block = self.blocks.len();
                    self.open_comment(start.id, block, 0);
                }
                DocumentChild::CommentEnd(end) => self.close_comment(end, None),
                DocumentChild::BookmarkStart(_)
                | DocumentChild::BookmarkEnd(_)
                | DocumentChild::TableOfContents(_)
                | DocumentChild::Section(_) => {}
            }
        }
    }

    fn finish(&mut self) -> RichDocument {
        // Every comment the walk never met — every reply, since `w:commentReference`
        // is not part of the typed tree, plus any comment whose range the producer
        // omitted. Swept in `comments.xml` order so a thread keeps its chronology.
        let unseen: Vec<usize> = self
            .comments
            .order
            .iter()
            .copied()
            .filter(|id| !self.seen.contains(id))
            .collect();
        for id in unseen {
            let Some(meta) = self.comments.get(id) else {
                continue;
            };
            match meta
                .parent
                .and_then(|p| self.annotation_of.get(&p).copied())
            {
                Some(index) => {
                    self.annotations[index].replies.push(SourceAnnotationReply {
                        author: meta.author.clone(),
                        created: meta.created,
                        body: meta.body.clone(),
                    });
                }
                None => {
                    // No range and no thread. `assemble` turns an annotation whose
                    // block does not exist into a comment on the row plus a
                    // diagnostic, which is what this is.
                    self.annotations.push(RichAnnotation {
                        block_index: usize::MAX,
                        start: 0,
                        length: 0,
                        author: meta.author.clone(),
                        created: meta.created,
                        body: meta.body.clone(),
                        resolved: meta.resolved,
                        replies: Vec::new(),
                    });
                }
            }
            self.seen.insert(id);
        }

        let counted = [
            (
                self.tracked_changes,
                ImportDiagnostic::TrackedChangesFlattened {
                    path: self.origin.clone(),
                    count: self.tracked_changes,
                },
            ),
            (
                self.text_boxes,
                ImportDiagnostic::TextBoxDropped {
                    path: self.origin.clone(),
                    count: self.text_boxes,
                },
            ),
            (
                self.embedded_objects,
                ImportDiagnostic::EmbeddedObjectDropped {
                    path: self.origin.clone(),
                    count: self.embedded_objects,
                },
            ),
            (
                self.fields,
                ImportDiagnostic::FieldFlattened {
                    path: self.origin.clone(),
                    count: self.fields,
                },
            ),
            (
                self.footnotes,
                ImportDiagnostic::FootnotesDegraded {
                    path: self.origin.clone(),
                    count: self.footnotes,
                },
            ),
        ];
        for (count, diagnostic) in counted {
            if count > 0 {
                self.diagnostics.push(diagnostic);
            }
        }
        for style in std::mem::take(&mut self.unknown_styles) {
            self.diagnostics.push(ImportDiagnostic::UnknownStyleLevel {
                path: self.origin.clone(),
                style,
            });
        }

        RichDocument {
            blocks: std::mem::take(&mut self.blocks),
            annotations: std::mem::take(&mut self.annotations),
        }
    }

    /// A paragraph that is a direct child of `w:body`, and so has an ordinal the
    /// supplementary pass can key on.
    fn top_level_paragraph(&mut self, paragraph: &Paragraph, ordinal: usize) {
        if self.raw.rules.contains(&ordinal) {
            // A horizontal rule. Emitted as its glyph so `skribisto_model` stays the
            // one authority on what a break is — the same treatment the ODT scanner
            // gives ODF's spelling of the same construct.
            self.blocks.push(RichBlock::body(vec![Run::plain(
                skribisto_model::scene_break::CANONICAL_MINOR,
            )]));
            return;
        }
        let first_block = self.blocks.len();
        self.paragraph(paragraph);
        let produced = self.blocks.len() > first_block;

        for (id, offset) in self.raw.references.get(&ordinal).into_iter().flatten() {
            self.point_comment(
                *id,
                if produced { first_block } else { usize::MAX },
                *offset,
            );
        }
    }

    /// A comment anchored to a point rather than a range: it belongs to the
    /// paragraph its reference sat in, which a zero length says downstream.
    fn point_comment(&mut self, id: usize, block: usize, offset: usize) {
        if block == usize::MAX {
            // The paragraph produced nothing — `assemble` reports this and keeps the
            // comment on its row rather than losing it.
            self.open_comment(id, usize::MAX, 0);
            return;
        }
        self.open_comment(id, block, offset);
    }

    fn paragraph(&mut self, paragraph: &Paragraph) {
        let property = &paragraph.property;
        let kind = match self.styles.heading_level(property) {
            HeadingVerdict::Heading(level) => ParagraphKind::Heading { level },
            HeadingVerdict::Unknown(style) => {
                self.unknown_styles.insert(style);
                ParagraphKind::Body
            }
            HeadingVerdict::Body => match &property.numbering_property {
                Some(numbering) => ParagraphKind::ListItem {
                    // Bullet or number is a property of the numbering definition,
                    // which Word stores separately and producers fill in
                    // inconsistently. An unordered list is the safe default: a
                    // bulleted list rendered as numbered invents an order the
                    // writer did not write.
                    ordered: false,
                    depth: numbering
                        .level
                        .as_ref()
                        .map(|l| l.val.min(8) as u8)
                        .unwrap_or(0),
                },
                None => ParagraphKind::Body,
            },
        };

        let mut build = ParaBuild {
            kind,
            runs: Vec::new(),
            len: 0,
        };
        self.paragraph_children(&paragraph.children, property, &mut build, None);
        self.flush(build);
    }

    fn paragraph_children(
        &mut self,
        children: &[ParagraphChild],
        property: &ParagraphProperty,
        build: &mut ParaBuild,
        link: Option<&str>,
    ) {
        for child in children {
            match child {
                ParagraphChild::Run(run) => self.run(run, property, build, link),
                // Accepted: this is the text as it stands.
                ParagraphChild::Insert(insert) => {
                    self.tracked_changes += 1;
                    self.insert_children(insert, property, build, link);
                }
                ParagraphChild::MoveTo(move_to) => {
                    self.tracked_changes += 1;
                    for child in &move_to.children {
                        match child {
                            MoveToChild::Run(run) => self.run(run, property, build, link),
                            MoveToChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), build.len)
                            }
                            MoveToChild::CommentEnd(end) => {
                                self.close_comment(end, Some(build.len))
                            }
                            MoveToChild::Delete(_) => self.tracked_changes += 1,
                        }
                    }
                }
                // Dropped: this is text somebody removed.
                ParagraphChild::Delete(delete) => {
                    self.tracked_changes += 1;
                    for child in &delete.children {
                        match child {
                            DeleteChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), build.len)
                            }
                            DeleteChild::CommentEnd(end) => {
                                self.close_comment(end, Some(build.len))
                            }
                            DeleteChild::Run(_) => {}
                        }
                    }
                }
                ParagraphChild::MoveFrom(_) => self.tracked_changes += 1,
                ParagraphChild::Hyperlink(hyperlink) => {
                    let url = match &hyperlink.link {
                        docx_rs::HyperlinkData::External { rid: _, path } => Some(path.clone()),
                        docx_rs::HyperlinkData::Anchor { anchor } => Some(format!("#{anchor}")),
                    };
                    self.paragraph_children(
                        &hyperlink.children,
                        property,
                        build,
                        url.as_deref().or(link),
                    );
                }
                ParagraphChild::CommentStart(start) => {
                    self.open_comment(start.id, self.blocks.len(), build.len)
                }
                ParagraphChild::CommentEnd(end) => self.close_comment(end, Some(build.len)),
                ParagraphChild::StructuredDataTag(tag) => {
                    for child in &tag.children {
                        if let docx_rs::StructuredDataTagChild::Run(run) = child {
                            self.run(run, property, build, link);
                        }
                    }
                }
                ParagraphChild::PageNum(_) | ParagraphChild::NumPages(_) => self.fields += 1,
                ParagraphChild::BookmarkStart(_) | ParagraphChild::BookmarkEnd(_) => {}
            }
        }
    }

    fn insert_children(
        &mut self,
        insert: &Insert,
        property: &ParagraphProperty,
        build: &mut ParaBuild,
        link: Option<&str>,
    ) {
        for child in &insert.children {
            match child {
                InsertChild::Run(run) => self.run(run, property, build, link),
                InsertChild::CommentStart(start) => {
                    self.open_comment(start.id, self.blocks.len(), build.len)
                }
                InsertChild::CommentEnd(end) => self.close_comment(end, Some(build.len)),
                // An insertion of a deletion: the text is gone either way.
                InsertChild::Delete(_) => {}
            }
        }
    }

    fn run(
        &mut self,
        run: &DocxRun,
        property: &ParagraphProperty,
        build: &mut ParaBuild,
        link: Option<&str>,
    ) {
        let style = self.styles.run_style(property, &run.run_property);
        for child in &run.children {
            match child {
                RunChild::Text(text) => {
                    build.len += text.text.chars().count();
                    build.runs.push(Run {
                        text: text.text.clone(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                    });
                }
                RunChild::Tab(_) | RunChild::PTab(_) => {
                    build.len += 1;
                    build.runs.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                    });
                }
                RunChild::Break(_) | RunChild::CarriageReturn(_) => {
                    // The conversion drops `<br>` outright, gluing the words either
                    // side together, so a break becomes a paragraph of its own.
                    let finished = std::mem::replace(
                        build,
                        ParaBuild {
                            kind: build.kind,
                            runs: Vec::new(),
                            len: 0,
                        },
                    );
                    self.flush(finished);
                }
                RunChild::Drawing(drawing) => match &drawing.data {
                    Some(DrawingData::Pic(pic)) => {
                        self.diagnostics.push(ImportDiagnostic::ImageNotIngested {
                            path: self.origin.clone(),
                            target: pic.id.clone(),
                        });
                        build.len += 1;
                        build.runs.push(Run::image("", pic.id.clone()));
                    }
                    Some(DrawingData::TextBox(_)) => self.text_boxes += 1,
                    None => self.embedded_objects += 1,
                },
                RunChild::Shape(_) => self.embedded_objects += 1,
                RunChild::FootnoteReference(_) => self.footnotes += 1,
                RunChild::FieldChar(field) => {
                    if matches!(field.field_char_type, docx_rs::FieldCharType::Begin) {
                        self.fields += 1;
                    }
                }
                RunChild::CommentStart(start) => {
                    self.open_comment(start.id, self.blocks.len(), build.len)
                }
                RunChild::CommentEnd(end) => self.close_comment(end, Some(build.len)),
                // A tracked deletion's text, an instruction's source, a symbol with
                // no Unicode meaning: none of them are prose.
                RunChild::DeleteText(_)
                | RunChild::DeleteInstrText(_)
                | RunChild::InstrText(_)
                | RunChild::InstrTextString(_)
                | RunChild::Sym(_)
                | RunChild::Shading(_) => {}
            }
        }
    }

    fn table(&mut self, table: &Table) {
        let mut rows: Vec<Vec<Vec<Run>>> = Vec::new();
        for TableChild::TableRow(row) in &table.rows {
            let mut cells: Vec<Vec<Run>> = Vec::new();
            for TableRowChild::TableCell(cell) in &row.cells {
                let mut text = String::new();
                for content in &cell.children {
                    if let TableCellContent::Paragraph(paragraph) = content {
                        if !text.is_empty() {
                            text.push(' ');
                        }
                        collect_paragraph_text(&paragraph.children, &mut text);
                    }
                }
                // One cell is one Djot cell whatever it held: a newline inside one
                // would break the plain-text arithmetic the annotations rest on.
                cells.push(vec![Run::plain(text.replace(['\n', '\r'], " ").trim())]);
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
        if !rows.is_empty() {
            self.blocks.push(RichBlock::Table { rows });
        }
    }

    fn flush(&mut self, build: ParaBuild) {
        if build.runs.is_empty() {
            return;
        }
        self.blocks.push(RichBlock::Paragraph {
            kind: build.kind,
            runs: build.runs,
        });
    }

    fn open_comment(&mut self, id: usize, block: usize, start: usize) {
        if self.seen.contains(&id) {
            return;
        }
        let Some(meta) = self.comments.get(id) else {
            return;
        };
        self.seen.insert(id);

        // A reply that *does* carry a range still belongs to its parent's thread.
        if let Some(index) = meta
            .parent
            .and_then(|p| self.annotation_of.get(&p).copied())
        {
            self.annotations[index].replies.push(SourceAnnotationReply {
                author: meta.author.clone(),
                created: meta.created,
                body: meta.body.clone(),
            });
            return;
        }

        let index = self.annotations.len();
        self.annotations.push(RichAnnotation {
            block_index: block,
            start,
            length: 0,
            author: meta.author.clone(),
            created: meta.created,
            body: meta.body.clone(),
            resolved: meta.resolved,
            replies: Vec::new(),
        });
        self.annotation_of.insert(id, index);
        self.open.insert(
            index,
            OpenRange {
                annotation: index,
                block,
                start,
            },
        );
    }

    /// Close whichever open range this end belongs to.
    ///
    /// `CommentRangeEnd` keeps its id private and offers no accessor, so the id is
    /// recovered by comparing against a constructed twin — the type derives
    /// `PartialEq`, and the set of open ranges is never more than a handful. Exact,
    /// and using nothing but the public API.
    fn close_comment(&mut self, end: &CommentRangeEnd, at: Option<usize>) {
        let ids: Vec<usize> = self.open.keys().copied().collect();
        let Some(index) = ids
            .into_iter()
            .find(|index| self.annotation_of_matches(*index, end))
        else {
            return;
        };
        let Some(open) = self.open.remove(&index) else {
            return;
        };
        let end_offset = match at {
            Some(offset) if open.block == self.blocks.len() => offset,
            _ => self
                .blocks
                .get(open.block)
                .map(|b| b.plain_text().chars().count())
                .unwrap_or(open.start),
        };
        self.annotations[open.annotation].length = end_offset.saturating_sub(open.start);
    }

    fn annotation_of_matches(&self, index: usize, end: &CommentRangeEnd) -> bool {
        self.annotation_of
            .iter()
            .any(|(id, at)| *at == index && *end == CommentRangeEnd::new(*id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_outline_level_of_nine_is_body_text_not_level_ten() {
        let mut property = ParagraphProperty::new();
        property.outline_lvl = Some(docx_rs::OutlineLvl::new(9));
        assert_eq!(outline_level(&property), None);
        property.outline_lvl = Some(docx_rs::OutlineLvl::new(0));
        assert_eq!(outline_level(&property), Some(1));
        property.outline_lvl = Some(docx_rs::OutlineLvl::new(2));
        assert_eq!(outline_level(&property), Some(3));
    }

    /// The style *id* is stable across locales; the style *name* is not, which is
    /// why nothing here ever reads one.
    #[test]
    fn a_heading_style_id_is_read_however_a_producer_spells_it() {
        for (id, expected) in [
            ("Heading1", Some(1)),
            ("heading 3", Some(3)),
            ("Heading-2", Some(2)),
            ("Heading9", Some(9)),
            ("Heading10", None),
            ("Heading0", None),
            ("Normal", None),
            ("Titre1", None),
        ] {
            assert_eq!(level_from_style_id(id), expected, "for {id:?}");
        }
    }

    #[test]
    fn a_style_that_is_plainly_a_heading_but_names_no_level_is_reported() {
        assert!(looks_like_a_heading("HeadingChapter"));
        assert!(looks_like_a_heading("TitleMain"));
        assert!(!looks_like_a_heading("BodyText"));
    }

    #[test]
    fn a_run_that_turns_bold_off_is_not_bold() {
        let mut style = RunStyle {
            bold: true,
            ..Default::default()
        };
        let mut property = RunProperty::new();
        property.bold = Some(Bold::new().disable());
        apply_run_property(&property, &mut style);
        assert!(!style.bold, "an explicit off must override an inherited on");
    }

    #[test]
    fn an_underline_of_none_is_not_an_underline() {
        let mut style = RunStyle::default();
        let mut property = RunProperty::new();
        property.underline = Some(Underline::new("none"));
        apply_run_property(&property, &mut style);
        assert!(!style.underline);
        property.underline = Some(Underline::new("single"));
        apply_run_property(&property, &mut style);
        assert!(style.underline);
    }

    #[test]
    fn a_comment_date_is_read_in_the_shapes_producers_write() {
        assert!(parse_date("2026-01-02T03:04:05Z").is_some());
        assert!(parse_date("2026-01-02T03:04:05").is_some());
        assert!(parse_date("2026-01-02").is_some());
        assert!(parse_date("").is_none());
        assert!(parse_date("not a date").is_none());
    }
}
