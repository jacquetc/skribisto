// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The OpenDocument Text scanner — `.odt` and its flat sibling `.fodt`.
//!
//! Zip plus XML, read member-by-member exactly the way the Plume importer reads a
//! `.plume`. Everything past "here are the paragraphs" is [`rich`]'s job, so this
//! module is only the spelling: which element is a heading, which attribute says
//! bold, where a comment starts and stops.
//!
//! ODT is the cleaner of the two container formats and therefore the one to read
//! first when something looks wrong in both. `<text:h text:outline-level="2">` says
//! the heading depth **explicitly**, so none of DOCX's style-name guesswork applies
//! and `UnknownStyleLevel` can never fire here.
//!
//! ## Comment threading: measured, not assumed
//!
//! ODF has no comment threading in the standard, so this was settled against
//! LibreOffice 25.8 rather than against the spec, in both directions:
//!
//! * A hand-written `office:annotation` carrying `loext:parent-name` **is**
//!   understood — converting such a file to `.docx` produced a matching
//!   `w15:paraIdParent`. So that attribute is the real spelling, and this scanner
//!   reads it.
//! * LibreOffice's own `.docx` → `.odt` export **does not write it**: a threaded
//!   Word comment comes out as two sibling annotations. Nothing in the file says a
//!   thread was lost, so nothing here can claim one was.
//!
//! [`ImportDiagnostic::CommentRepliesFlattened`] therefore reports the one case that
//! *is* detectable: a `loext:parent-name` naming an annotation this scanner never
//! saw. A reply whose producer already flattened it arrives as an ordinary comment,
//! which is what it now is.
//!
//! ## Two deliberate simplifications, both stated rather than hidden
//!
//! * **A comment spanning several paragraphs is clamped to the first.** Its quote
//!   still starts on the words it started on; it simply stops at the end of that
//!   paragraph instead of running on. Better than the alternative, which is a range
//!   whose end nobody can locate.
//! * **A `Quotations`-styled paragraph is imported as prose, not as a quote.** ODF
//!   expresses a block quote as an indent on a paragraph style, and matching style
//!   names to recover it is the kind of guess that goes wrong quietly. The words
//!   are all there; only the indent is not.
//!
//! ## A horizontal line *is* a scene break, and that is not the alignment rule
//!
//! `block.rs` refuses to read a scene break out of a centred paragraph, and this
//! does not contradict it. A centred paragraph is layout that a *human* reads as a
//! break; an empty paragraph whose style declares nothing but a bottom border is
//! the ODF encoding of a thematic break — the same construct CommonMark spells
//! `***` and the Markdown scanner already turns into a `SceneBreak` unconditionally.
//! Refusing it would lose a break the writer can see, and would make ODT the one
//! format that cannot round-trip its own thematic breaks. (Pandoc writes exactly
//! this for `* * *`, under the style name `Horizontal Line`.)

use std::collections::HashMap;
use std::io::Read;

use anyhow::{Result, anyhow};
use roxmltree::{Document, Node};

use crate::block::{SourceAnnotationReply, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::scanner::SourceScanner;
use crate::sources::rich::{
    ParagraphKind, RichAnnotation, RichBlock, RichDocument, Run, RunStyle, assemble,
};

const NS_OFFICE: &str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
const NS_TEXT: &str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
const NS_TABLE: &str = "urn:oasis:names:tc:opendocument:xmlns:table:1.0";
const NS_STYLE: &str = "urn:oasis:names:tc:opendocument:xmlns:style:1.0";
const NS_FO: &str = "urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0";
const NS_DRAW: &str = "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0";
const NS_DC: &str = "http://purl.org/dc/elements/1.1/";
const NS_XLINK: &str = "http://www.w3.org/1999/xlink";
const NS_LOEXT: &str = "urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0";

/// Field elements whose text is the value they were last showing.
///
/// A curated list rather than "anything unrecognised": an unknown element still has
/// its text collected, so the prose survives either way — this is only about which
/// ones are worth *telling the writer* went static.
const FIELD_ELEMENTS: &[&str] = &[
    "page-number",
    "page-count",
    "date",
    "time",
    "chapter",
    "title",
    "subject",
    "author-name",
    "author-initials",
    "initial-creator",
    "file-name",
    "sequence",
    "bookmark-ref",
    "reference-ref",
    "variable-get",
    "user-defined",
    "creation-date",
    "modification-date",
    "editing-duration",
    "text-input",
    "placeholder",
    "expression",
    "conditional-text",
    "hidden-text",
];

pub struct OdtScanner;

impl SourceScanner for OdtScanner {
    fn extensions(&self) -> &[&str] {
        &["odt", "fodt"]
    }

    fn format_name(&self) -> &'static str {
        "opendocument-text"
    }

    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        let parts = read_parts(bytes)?;
        let content = Document::parse(&parts.content)
            .map_err(|e| anyhow!("content.xml is not well-formed XML: {e}"))?;
        let styles_doc = match &parts.styles {
            Some(xml) => Some(
                Document::parse(xml)
                    .map_err(|e| anyhow!("styles.xml is not well-formed XML: {e}"))?,
            ),
            None => None,
        };

        let mut styles = StyleTable::default();
        if let Some(d) = &styles_doc {
            styles.collect(d.root_element());
        }
        styles.collect(content.root_element());

        let mut doc = SourceDocument::new(display_name, origin);
        if let Some(meta) = &parts.meta
            && let Ok(m) = Document::parse(meta)
            && let Some(title) = find_descendant(m.root_element(), NS_DC, "title")
        {
            let title = element_text(title);
            if !title.trim().is_empty() {
                doc.metadata.title = Some(title.trim().to_string());
            }
        }

        let body = find_descendant(content.root_element(), NS_OFFICE, "text")
            .ok_or_else(|| anyhow!("no <office:text> body"))?;

        let mut walker = Walker::new(&styles, origin);
        walker.walk_container(body);
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
            .any(|b| matches!(b, crate::block::SourceBlock::Heading { .. }))
        {
            doc.diagnostics.push(ImportDiagnostic::NoHeadings {
                path: origin.to_string(),
            });
        }
        Ok(doc)
    }
}

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

struct Parts {
    content: String,
    styles: Option<String>,
    meta: Option<String>,
}

/// Read the three XML parts, from either a zip `.odt` or a flat `.fodt`.
///
/// Flat ODF is one XML file holding what the zip splits in three, so it is read as
/// `content` and left to answer for all of them — every element this scanner looks
/// for is in the same document.
fn read_parts(bytes: &[u8]) -> Result<Parts> {
    if !bytes.starts_with(b"PK") {
        let text = String::from_utf8_lossy(bytes).into_owned();
        return Ok(Parts {
            content: text,
            styles: None,
            meta: None,
        });
    }
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))
        .map_err(|e| anyhow!("not a readable OpenDocument container: {e}"))?;
    let content = read_member(&mut zip, "content.xml")?
        .ok_or_else(|| anyhow!("no content.xml in the container"))?;
    let styles = read_member(&mut zip, "styles.xml")?;
    let meta = read_member(&mut zip, "meta.xml")?;
    Ok(Parts {
        content,
        styles,
        meta,
    })
}

fn read_member<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    name: &str,
) -> Result<Option<String>> {
    let Ok(mut file) = zip.by_name(name) else {
        return Ok(None);
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| anyhow!("{name} could not be read: {e}"))?;
    Ok(Some(String::from_utf8_lossy(&buffer).into_owned()))
}

// ---------------------------------------------------------------------------
// Styles
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
struct RawTextStyle {
    parent: Option<String>,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    strikethrough: Option<bool>,
}

#[derive(Default, Clone)]
struct RawParaStyle {
    parent: Option<String>,
    /// A bottom border with nothing else — the ODF spelling of a horizontal rule.
    rule: Option<bool>,
    /// The text properties a paragraph style carries in its own right; a run with
    /// no span of its own inherits them.
    text: RawTextStyle,
}

#[derive(Default)]
struct StyleTable {
    text: HashMap<String, RawTextStyle>,
    para: HashMap<String, RawParaStyle>,
    /// List style name → whether each 1-based level is numbered.
    list: HashMap<String, Vec<bool>>,
}

impl StyleTable {
    /// Walk a whole document collecting every `style:style` and `text:list-style`,
    /// wherever it lives — `office:styles`, `office:automatic-styles` and
    /// `office:master-styles` all hold them, and a flat `.fodt` holds all three.
    fn collect(&mut self, root: Node<'_, '_>) {
        for node in root.descendants().filter(Node::is_element) {
            match (node.tag_name().namespace(), node.tag_name().name()) {
                (Some(NS_STYLE), "style") => self.collect_style(node),
                (Some(NS_TEXT), "list-style") => self.collect_list_style(node),
                _ => {}
            }
        }
    }

    fn collect_style(&mut self, node: Node<'_, '_>) {
        let Some(name) = node.attribute((NS_STYLE, "name")) else {
            return;
        };
        let parent = node
            .attribute((NS_STYLE, "parent-style-name"))
            .map(str::to_string);
        let text = text_properties(node, parent.clone());

        match node.attribute((NS_STYLE, "family")) {
            Some("text") => {
                self.text.insert(name.to_string(), text);
            }
            Some("paragraph") => {
                let rule = node
                    .children()
                    .find(|c| is(c, NS_STYLE, "paragraph-properties"))
                    .map(|p| {
                        let bottom = p.attribute((NS_FO, "border-bottom"));
                        let all = p.attribute((NS_FO, "border"));
                        let has_bottom =
                            bottom.is_some_and(|v| v != "none") || all.is_some_and(|v| v != "none");
                        let sides_clear = ["border-top", "border-left", "border-right"]
                            .iter()
                            .all(|s| p.attribute((NS_FO, *s)).is_none_or(|v| v == "none"));
                        has_bottom && sides_clear
                    });
                self.para.insert(
                    name.to_string(),
                    RawParaStyle {
                        parent: parent.clone(),
                        rule,
                        text,
                    },
                );
            }
            _ => {}
        }
    }

    fn collect_list_style(&mut self, node: Node<'_, '_>) {
        let Some(name) = node.attribute((NS_STYLE, "name")) else {
            return;
        };
        let mut levels: Vec<bool> = Vec::new();
        for child in node.children().filter(Node::is_element) {
            let ordered = match child.tag_name().name() {
                "list-level-style-number" => true,
                "list-level-style-bullet" | "list-level-style-image" => false,
                _ => continue,
            };
            let level: usize = child
                .attribute((NS_TEXT, "level"))
                .and_then(|v| v.parse().ok())
                .unwrap_or(levels.len() + 1);
            if levels.len() < level {
                levels.resize(level, false);
            }
            levels[level - 1] = ordered;
        }
        self.list.insert(name.to_string(), levels);
    }

    /// The character formatting a named text style resolves to, following parents.
    fn text_style(&self, name: &str) -> RunStyle {
        let mut style = RunStyle::default();
        let mut chain: Vec<&RawTextStyle> = Vec::new();
        let mut current = Some(name.to_string());
        // Root-first, so a child overrides its parent.
        let mut guard = 0;
        while let Some(n) = current {
            let Some(raw) = self.text.get(&n).or(self.para.get(&n).map(|p| &p.text)) else {
                break;
            };
            chain.push(raw);
            current = raw.parent.clone();
            guard += 1;
            if guard > 32 {
                break; // a cycle in the style graph; take what we have
            }
        }
        for raw in chain.iter().rev() {
            if let Some(v) = raw.bold {
                style.bold = v;
            }
            if let Some(v) = raw.italic {
                style.italic = v;
            }
            if let Some(v) = raw.underline {
                style.underline = v;
            }
            if let Some(v) = raw.strikethrough {
                style.strikethrough = v;
            }
        }
        style
    }

    /// Whether a paragraph style is the ODF spelling of a horizontal rule.
    fn is_rule(&self, name: &str) -> bool {
        let mut current = Some(name.to_string());
        let mut guard = 0;
        while let Some(n) = current {
            let Some(raw) = self.para.get(&n) else {
                return false;
            };
            if let Some(rule) = raw.rule {
                return rule;
            }
            current = raw.parent.clone();
            guard += 1;
            if guard > 32 {
                return false;
            }
        }
        false
    }

    fn list_is_ordered(&self, name: Option<&str>, depth: u8) -> bool {
        name.and_then(|n| self.list.get(n))
            .and_then(|levels| levels.get(depth as usize))
            .copied()
            .unwrap_or(false)
    }
}

fn text_properties(node: Node<'_, '_>, parent: Option<String>) -> RawTextStyle {
    let mut raw = RawTextStyle {
        parent,
        ..Default::default()
    };
    let Some(props) = node.children().find(|c| is(c, NS_STYLE, "text-properties")) else {
        return raw;
    };
    if let Some(weight) = props.attribute((NS_FO, "font-weight")) {
        raw.bold = Some(is_bold(weight));
    }
    if let Some(style) = props.attribute((NS_FO, "font-style")) {
        raw.italic = Some(matches!(style, "italic" | "oblique"));
    }
    if let Some(u) = props.attribute((NS_STYLE, "text-underline-style")) {
        raw.underline = Some(u != "none");
    }
    if let Some(s) = props.attribute((NS_STYLE, "text-line-through-style")) {
        raw.strikethrough = Some(s != "none");
    }
    raw
}

/// `bold`, or any numeric weight from 600 up — `fo:font-weight` allows both.
fn is_bold(value: &str) -> bool {
    value == "bold" || value.parse::<u32>().is_ok_and(|w| w >= 600)
}

// ---------------------------------------------------------------------------
// Body
// ---------------------------------------------------------------------------

/// A comment whose range has been opened and not yet closed.
struct OpenRange {
    /// Index into `Walker::annotations`.
    annotation: usize,
    block: usize,
    start: usize,
}

struct Walker<'a> {
    styles: &'a StyleTable,
    origin: String,
    blocks: Vec<RichBlock>,
    annotations: Vec<RichAnnotation>,
    /// `office:name` → index into `annotations`, for `annotation-end` and for
    /// `loext:parent-name`.
    by_name: HashMap<String, usize>,
    open: HashMap<String, OpenRange>,
    diagnostics: Vec<ImportDiagnostic>,
    tracked_changes: usize,
    text_boxes: usize,
    embedded_objects: usize,
    fields: usize,
    footnotes: usize,
    orphan_replies: usize,
}

/// The paragraph currently being built.
struct ParaBuild {
    kind: ParagraphKind,
    runs: Vec<Run>,
    len: usize,
}

impl<'a> Walker<'a> {
    fn new(styles: &'a StyleTable, origin: &str) -> Self {
        Walker {
            styles,
            origin: origin.to_string(),
            blocks: Vec::new(),
            annotations: Vec::new(),
            by_name: HashMap::new(),
            open: HashMap::new(),
            diagnostics: Vec::new(),
            tracked_changes: 0,
            text_boxes: 0,
            embedded_objects: 0,
            fields: 0,
            footnotes: 0,
            orphan_replies: 0,
        }
    }

    fn finish(&mut self) -> RichDocument {
        // Anything still open never met its `annotation-end`: it is a comment on the
        // paragraph, which is exactly what a zero length means downstream.
        self.open.clear();

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
            (
                self.orphan_replies,
                ImportDiagnostic::CommentRepliesFlattened {
                    path: self.origin.clone(),
                    count: self.orphan_replies,
                },
            ),
        ];
        for (count, diagnostic) in counted {
            if count > 0 {
                self.diagnostics.push(diagnostic);
            }
        }

        RichDocument {
            blocks: std::mem::take(&mut self.blocks),
            annotations: std::mem::take(&mut self.annotations),
        }
    }

    /// Walk anything that holds block-level content.
    fn walk_container(&mut self, node: Node<'_, '_>) {
        for child in node.children().filter(Node::is_element) {
            let name = child.tag_name().name();
            match (child.tag_name().namespace(), name) {
                (Some(NS_TEXT), "h") => {
                    let level = child
                        .attribute((NS_TEXT, "outline-level"))
                        .and_then(|v| v.parse::<u8>().ok())
                        .unwrap_or(1)
                        .max(1);
                    self.paragraph(child, ParagraphKind::Heading { level });
                }
                (Some(NS_TEXT), "p") => {
                    // A rule-styled paragraph is a thematic break — see the module
                    // note. It is emitted as its glyph so the vocabulary in
                    // `skribisto_model` stays the one authority on what a break is.
                    let styled_as_rule = child
                        .attribute((NS_TEXT, "style-name"))
                        .is_some_and(|s| self.styles.is_rule(s));
                    if styled_as_rule && element_text(child).trim().is_empty() {
                        self.push_block(RichBlock::body(vec![Run::plain(
                            skribisto_model::scene_break::CANONICAL_MINOR,
                        )]));
                        continue;
                    }
                    self.paragraph(child, ParagraphKind::Body);
                }
                (Some(NS_TEXT), "list") => self.walk_list(child, 0),
                (Some(NS_TEXT), "section") => self.walk_container(child),
                (Some(NS_TEXT), "tracked-changes") => {
                    self.tracked_changes += child
                        .children()
                        .filter(|c| is(c, NS_TEXT, "changed-region"))
                        .count();
                }
                (Some(NS_TABLE), "table") => self.walk_table(child),
                (Some(NS_TEXT), "soft-page-break") | (Some(NS_TEXT), "sequence-decls") => {}
                // Anything else that could hold paragraphs (index bodies, frames at
                // body level, change marks). Recursing beats ignoring: an unread
                // container is prose the writer wrote and never saw again.
                _ => self.walk_container(child),
            }
        }
    }

    fn walk_list(&mut self, node: Node<'_, '_>, depth: u8) {
        let ordered = self
            .styles
            .list_is_ordered(node.attribute((NS_TEXT, "style-name")), depth);
        for item in node.children().filter(Node::is_element) {
            if !matches!(item.tag_name().name(), "list-item" | "list-header") {
                continue;
            }
            for child in item.children().filter(Node::is_element) {
                match (child.tag_name().namespace(), child.tag_name().name()) {
                    (Some(NS_TEXT), "p") | (Some(NS_TEXT), "h") => {
                        self.paragraph(child, ParagraphKind::ListItem { ordered, depth })
                    }
                    (Some(NS_TEXT), "list") => self.walk_list(child, depth.saturating_add(1)),
                    _ => {}
                }
            }
        }
    }

    fn walk_table(&mut self, node: Node<'_, '_>) {
        let mut rows: Vec<Vec<Vec<Run>>> = Vec::new();
        for row in node.descendants().filter(|n| is(n, NS_TABLE, "table-row")) {
            let mut cells: Vec<Vec<Run>> = Vec::new();
            for cell in row.children().filter(|c| is(c, NS_TABLE, "table-cell")) {
                // A cell's paragraphs are joined with a space: a table cell is one
                // Djot cell whatever it holds, and a newline inside one would break
                // the plain-text arithmetic the annotations rest on.
                let text = element_text(cell).replace(['\n', '\r'], " ");
                cells.push(vec![Run::plain(text.trim())]);
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
        if !rows.is_empty() {
            self.push_block(RichBlock::Table { rows });
        }
    }

    fn paragraph(&mut self, node: Node<'_, '_>, kind: ParagraphKind) {
        let base = node
            .attribute((NS_TEXT, "style-name"))
            .map(|s| self.styles.text_style(s))
            .unwrap_or_default();
        let mut build = ParaBuild {
            kind,
            runs: Vec::new(),
            len: 0,
        };
        self.inline(node, &mut build, base, None);
        self.flush_paragraph(build);
    }

    /// Finish the paragraph under construction and start a fresh one — what a
    /// deliberate line break becomes, since the conversion drops `<br>` outright
    /// and would otherwise glue the two lines into one word.
    fn flush_paragraph(&mut self, build: ParaBuild) {
        if build.runs.is_empty() {
            return;
        }
        self.push_block(RichBlock::Paragraph {
            kind: build.kind,
            runs: build.runs,
        });
    }

    fn push_block(&mut self, block: RichBlock) {
        self.blocks.push(block);
    }

    /// Walk one paragraph's inline content, accumulating runs and tracking where
    /// each comment range opens and closes.
    fn inline(
        &mut self,
        node: Node<'_, '_>,
        build: &mut ParaBuild,
        style: RunStyle,
        link: Option<&str>,
    ) {
        for child in node.children() {
            if child.is_text() {
                let text = child.text().unwrap_or_default();
                if !text.is_empty() {
                    build.len += text.chars().count();
                    build.runs.push(Run {
                        text: text.to_string(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                    });
                }
                continue;
            }
            if !child.is_element() {
                continue;
            }
            let ns = child.tag_name().namespace();
            let name = child.tag_name().name();
            match (ns, name) {
                (Some(NS_TEXT), "span") => {
                    let inner = child
                        .attribute((NS_TEXT, "style-name"))
                        .map(|s| merge(style, self.styles.text_style(s)))
                        .unwrap_or(style);
                    self.inline(child, build, inner, link);
                }
                (Some(NS_TEXT), "a") => {
                    let url = child.attribute((NS_XLINK, "href")).unwrap_or_default();
                    self.inline(child, build, style, Some(url));
                }
                (Some(NS_TEXT), "s") => {
                    let count: usize = child
                        .attribute((NS_TEXT, "c"))
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1);
                    let spaces = " ".repeat(count);
                    build.len += count;
                    build.runs.push(Run {
                        text: spaces,
                        style,
                        link: link.map(str::to_string),
                        image: None,
                    });
                }
                (Some(NS_TEXT), "tab") => {
                    build.len += 1;
                    build.runs.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                    });
                }
                (Some(NS_TEXT), "line-break") => {
                    let finished = std::mem::replace(
                        build,
                        ParaBuild {
                            kind: build.kind,
                            runs: Vec::new(),
                            len: 0,
                        },
                    );
                    self.flush_paragraph(finished);
                }
                (Some(NS_OFFICE), "annotation") => self.open_annotation(child, build),
                (Some(NS_OFFICE), "annotation-end") => {
                    if let Some(name) = child.attribute((NS_OFFICE, "name")) {
                        self.close_annotation(name, build);
                    }
                }
                (Some(NS_TEXT), "note") => {
                    // Footnotes and endnotes have no carrier in the block model, and
                    // their body is not part of the sentence it hangs off.
                    self.footnotes += 1;
                }
                (Some(NS_DRAW), "frame") | (Some(NS_DRAW), "g") => self.frame(child, build, style),
                (Some(NS_TEXT), "change-start")
                | (Some(NS_TEXT), "change-end")
                | (Some(NS_TEXT), "change")
                | (Some(NS_TEXT), "bookmark")
                | (Some(NS_TEXT), "bookmark-start")
                | (Some(NS_TEXT), "bookmark-end")
                | (Some(NS_TEXT), "soft-page-break") => {}
                (Some(NS_TEXT), field) if FIELD_ELEMENTS.contains(&field) => {
                    self.fields += 1;
                    self.inline(child, build, style, link);
                }
                // Unknown inline element: take its text rather than drop it.
                _ => self.inline(child, build, style, link),
            }
        }
    }

    fn frame(&mut self, node: Node<'_, '_>, build: &mut ParaBuild, style: RunStyle) {
        for child in node.children().filter(Node::is_element) {
            match child.tag_name().name() {
                "image" => {
                    let src = child.attribute((NS_XLINK, "href")).unwrap_or_default();
                    let alt = node
                        .children()
                        .find(|c| is(c, NS_DRAW, "image") || is(c, NS_SVG, "title"))
                        .and_then(|c| c.text())
                        .unwrap_or("")
                        .to_string();
                    self.diagnostics.push(ImportDiagnostic::ImageNotIngested {
                        path: self.origin.clone(),
                        target: src.to_string(),
                    });
                    build.len += 1;
                    build.runs.push(Run::image(alt, src));
                }
                "text-box" => self.text_boxes += 1,
                "object" | "object-ole" | "applet" | "plugin" | "floating-frame" => {
                    self.embedded_objects += 1
                }
                _ => {
                    let _ = style;
                }
            }
        }
    }

    fn open_annotation(&mut self, node: Node<'_, '_>, build: &ParaBuild) {
        let author = node
            .children()
            .find(|c| is(c, NS_DC, "creator"))
            .map(element_text)
            .unwrap_or_default();
        let created = node
            .children()
            .find(|c| is(c, NS_DC, "date"))
            .map(element_text)
            .and_then(|d| parse_date(&d));
        let body = node
            .children()
            .filter(|c| is(c, NS_TEXT, "p"))
            .map(element_text)
            .collect::<Vec<_>>()
            .join("\n");
        let resolved = node
            .attribute((NS_LOEXT, "resolved"))
            .is_some_and(|v| v == "true");
        let name = node.attribute((NS_OFFICE, "name"));

        // A reply joins its parent's thread rather than becoming a comment.
        if let Some(parent) = node.attribute((NS_LOEXT, "parent-name")) {
            match self.by_name.get(parent).copied() {
                Some(index) => {
                    self.annotations[index].replies.push(SourceAnnotationReply {
                        author,
                        created,
                        body,
                    });
                    return;
                }
                // The parent is not in this document — the only case a file can
                // actually tell us a thread was broken.
                None => self.orphan_replies += 1,
            }
        }

        let index = self.annotations.len();
        self.annotations.push(RichAnnotation {
            block_index: self.blocks.len(),
            start: build.len,
            length: 0,
            author,
            created,
            body,
            resolved,
            replies: Vec::new(),
        });
        if let Some(name) = name {
            self.by_name.insert(name.to_string(), index);
            self.open.insert(
                name.to_string(),
                OpenRange {
                    annotation: index,
                    block: self.blocks.len(),
                    start: build.len,
                },
            );
        }
    }

    /// Close a comment range.
    ///
    /// A range that ends in a *later* paragraph is clamped to the end of the one it
    /// started in — the quote still begins on the right words, and a range whose end
    /// nobody can place is worse than a short one.
    fn close_annotation(&mut self, name: &str, build: &ParaBuild) {
        let Some(open) = self.open.remove(name) else {
            return;
        };
        let end = if open.block == self.blocks.len() {
            build.len
        } else {
            self.blocks
                .get(open.block)
                .map(|b| b.plain_text().chars().count())
                .unwrap_or(open.start)
        };
        self.annotations[open.annotation].length = end.saturating_sub(open.start);
    }
}

const NS_SVG: &str = "urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0";

/// Character formatting from an inner span applied over what it inherits.
///
/// ODF spans do not carry "off" switches for what a parent turned on, so an inner
/// span can only add — which is what a reader of the document sees.
fn merge(outer: RunStyle, inner: RunStyle) -> RunStyle {
    RunStyle {
        bold: outer.bold || inner.bold,
        italic: outer.italic || inner.italic,
        underline: outer.underline || inner.underline,
        strikethrough: outer.strikethrough || inner.strikethrough,
        code: outer.code || inner.code,
    }
}

fn is(node: &Node<'_, '_>, ns: &str, name: &str) -> bool {
    node.is_element() && node.tag_name().namespace() == Some(ns) && node.tag_name().name() == name
}

fn find_descendant<'a, 'input>(
    root: Node<'a, 'input>,
    ns: &str,
    name: &str,
) -> Option<Node<'a, 'input>> {
    root.descendants().find(|n| is(n, ns, name))
}

/// Every text node under `node`, concatenated.
fn element_text(node: Node<'_, '_>) -> String {
    node.descendants()
        .filter(Node::is_text)
        .filter_map(|n| n.text())
        .collect()
}

/// `dc:date` is ISO 8601, with or without an offset. LibreOffice writes it without.
fn parse_date(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let value = value.trim();
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&chrono::Utc));
    }
    for format in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M", "%Y-%m-%d"] {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(value, format) {
            return Some(naive.and_utc());
        }
        if let Ok(date) = chrono::NaiveDate::parse_from_str(value, format) {
            return Some(date.and_hms_opt(0, 0, 0)?.and_utc());
        }
    }
    None
}
