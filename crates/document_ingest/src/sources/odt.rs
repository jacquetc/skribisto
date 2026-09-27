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
//! ## One deliberate simplification, stated rather than hidden
//!
//! **A comment spanning several paragraphs keeps its extent only within one stretch of
//! prose.** When its end lands in the same stored block as its start, the whole range
//! comes over; when a heading, a table or a scene break lies between them, it stops at
//! the end of the paragraph it started in. Its quote still starts on the words it
//! started on, which beats a range whose end nobody can locate.
//!
//! ## A `Quotations` paragraph is a quotation
//!
//! A paragraph in LibreOffice's `Quotations` style, or in a style built on it, arrives as a
//! quotation: the style is the application stating what the paragraph is, in its own
//! vocabulary, which is not a guess from an indent (see [`rich::styled_as`]).
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

use anyhow::{Result, anyhow};
use roxmltree::{Document, Node};
use skrib_format::xml_depth::{Dtd, XmlError};

use crate::block::SourceDocument;
use crate::diagnostics::ImportDiagnostic;
use crate::scanner::SourceScanner;
use crate::sources::rich;
use crate::sources::rich::{
    Alignment, AnnotationEnd, BlockProps, CommentMark, Direction, OpenMark, ParagraphKind,
    RichAnnotation, RichBlock, RichDocument, RichReply, RichRowMark, Run, RunStyle, assemble,
    attach_comment_marks,
};
use skribisto_model::round_trip;

const NS_OFFICE: &str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
const NS_TEXT: &str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
const NS_TABLE: &str = "urn:oasis:names:tc:opendocument:xmlns:table:1.0";
const NS_STYLE: &str = "urn:oasis:names:tc:opendocument:xmlns:style:1.0";
const NS_FO: &str = "urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0";
const NS_DRAW: &str = "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0";
const NS_DC: &str = "http://purl.org/dc/elements/1.1/";
const NS_XLINK: &str = "http://www.w3.org/1999/xlink";
const NS_LOEXT: &str = "urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0";
/// Skribisto's own extension namespace, carrying `skrb:uid` on `<office:annotation>` —
/// the exact same URI `text-document`'s `export_odt_uc` declares (`odt_render::NAMESPACES`)
/// under the same `skrb` prefix. Read here (M-S7) so a re-import of a file Skribisto
/// itself exported recognises its own comments instead of duplicating them — see
/// `RichAnnotation::uid`'s own doc. There is no ODT-side equivalent for
/// `author_initials`: ODF's `<office:annotation>` schema has no carrier for it (the
/// module doc above already states this format ceiling for the writer's side; it is
/// the same ceiling here, on the reader's).
const NS_SKRB: &str = "urn:ferntech:text-document:comment:1";

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

/// The most spaces one `<text:s text:c="…"/>` is read as.
///
/// ODF writes a run of spaces past the first as one element carrying its length, and the
/// length is the file's to choose. Taken as written, `text:c="4000000000"` asked for four
/// gigabytes of spaces, and a longer number for more than an allocation can hold, which
/// ends the process rather than the import. What a real document puts there is blank
/// space laid out by hand: a line of a manuscript page holds about a hundred and fifty
/// spaces of a twelve-point serif, so a thousand is more than six lines of nothing but
/// space. A longer run is cut to this many and reported
/// ([`ImportDiagnostic::SpacesShortened`]); the words around it are untouched.
pub(crate) const MAX_SPACE_RUN: usize = 1_000;

/// How many spaces a `<text:s>` stands for, at most [`MAX_SPACE_RUN`], and whether the
/// count it gave was cut to that.
///
/// `text:c` is a positive integer of any length, so a number too long for a `usize` is
/// still a count, and a long one. A missing or unreadable count is one space, as ODF
/// has it.
fn space_run(node: Node<'_, '_>) -> (usize, bool) {
    let Some(value) = node.attribute((NS_TEXT, "c")).map(str::trim) else {
        return (1, false);
    };
    match value.parse::<usize>() {
        Ok(count) if count <= MAX_SPACE_RUN => (count, false),
        Ok(_) => (MAX_SPACE_RUN, true),
        Err(_) => {
            let digits = value.strip_prefix('+').unwrap_or(value);
            if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
                (MAX_SPACE_RUN, true)
            } else {
                (1, false)
            }
        }
    }
}

pub struct OdtScanner;

impl SourceScanner for OdtScanner {
    fn extensions(&self) -> &[&str] {
        &["odt", "fodt"]
    }

    fn format_name(&self) -> &'static str {
        "opendocument-text"
    }

    /// The whole scan runs on `skrib_format`'s parser stack: the parse, the walk
    /// that follows it, and the conversion of what the walk collected. Each part is
    /// refused first if it nests past `MAX_XML_DEPTH` (see `parse_part`), and
    /// every recursion in the walk is bounded by the same ceiling, so a document
    /// that gets as far as the walk fits on that stack with room to spare.
    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        skrib_format::xml_depth::on_parser_stack(|| scan_document(bytes, display_name, origin))
            .map_err(anyhow::Error::new)?
    }
}

/// [`OdtScanner::scan`], on the parser stack.
fn scan_document(bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
    let parts = read_parts(bytes, origin)?;
    let content = parse_part(&parts.content_part, &parts.content)?;
    let styles_doc = match &parts.styles {
        Some(xml) => Some(parse_part("styles.xml", xml)?),
        None => None,
    };

    let mut styles = StyleTable::default();
    if let Some(d) = &styles_doc {
        styles.collect(d.root_element());
    }
    styles.collect(content.root_element());

    let mut doc = SourceDocument::new(display_name, origin);
    // The title is optional and a `meta.xml` that does not parse costs only the
    // title. One nested past the ceiling refuses the file like any other part.
    let meta = match &parts.meta {
        Some(xml) => match parse_part("meta.xml", xml) {
            Ok(meta) => Some(meta),
            Err(e) if skrib_format::xml_depth::too_deep(&e).is_some() => return Err(e),
            Err(_) => None,
        },
        None => None,
    };
    if let Some(m) = &meta
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
    walker.walk_container(body, element_depth(body));
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

/// Parse one part through `skrib_format::xml_depth`: refused with a typed
/// `XmlTooDeep` if it nests past the ceiling, otherwise parsed on the parser stack.
fn parse_part<'a>(part: &str, xml: &'a str) -> Result<Document<'a>> {
    skrib_format::xml_depth::parse(part, xml, Dtd::Refuse).map_err(|e| match e {
        XmlError::Malformed(e) => anyhow!("{part} is not well-formed XML: {e}"),
        other => anyhow::Error::new(other),
    })
}

/// How deep `node` sits, the root element counting as one. Asked of the body the
/// walk starts from, which carries the count down from there, and of each table
/// cell's paragraph, which the table reaches through `descendants` instead.
fn element_depth(node: Node<'_, '_>) -> usize {
    node.ancestors().filter(Node::is_element).count()
}

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

struct Parts {
    /// What a refusal calls `content`: `content.xml` in a zip, the file itself
    /// when it is a flat `.fodt`.
    content_part: String,
    content: String,
    styles: Option<String>,
    meta: Option<String>,
}

/// Read the three XML parts, from either a zip `.odt` or a flat `.fodt`.
///
/// Flat ODF is one XML file holding what the zip splits in three, so it is read as
/// `content` and left to answer for all of them — every element this scanner looks
/// for is in the same document.
fn read_parts(bytes: &[u8], origin: &str) -> Result<Parts> {
    if !bytes.starts_with(b"PK") {
        let text = String::from_utf8_lossy(bytes).into_owned();
        let file_name = std::path::Path::new(origin)
            .file_name()
            .map_or_else(|| origin.to_string(), |n| n.to_string_lossy().into_owned());
        return Ok(Parts {
            content_part: file_name,
            content: text,
            styles: None,
            meta: None,
        });
    }
    // Every member inflates under the shared guard, so a crafted `.odt` (a bomb, or a
    // zip64 header declaring an enormous member) is refused before it fills memory —
    // the refusal reaching the review as `ImportDiagnostic::ArchiveTooLarge`.
    let (mut zip, mut guard) = skrib_format::zip_guard::ZipGuard::open(
        crate::sources::DOCUMENT_ZIP_LIMITS,
        std::io::Cursor::new(bytes),
    )
    .map_err(|e| anyhow!("not a readable OpenDocument container: {e}"))?;
    // The three parts this scanner reads, weighed before any inflates.
    guard.check_directory(&mut zip, |name| {
        matches!(name, "content.xml" | "styles.xml" | "meta.xml")
    })?;
    let content = read_member(&mut guard, &mut zip, "content.xml")?
        .ok_or_else(|| anyhow!("no content.xml in the container"))?;
    let styles = read_member(&mut guard, &mut zip, "styles.xml")?;
    let meta = read_member(&mut guard, &mut zip, "meta.xml")?;
    Ok(Parts {
        content_part: "content.xml".to_string(),
        content,
        styles,
        meta,
    })
}

fn read_member<R: std::io::Read + std::io::Seek>(
    guard: &mut skrib_format::zip_guard::ZipGuard,
    zip: &mut zip::ZipArchive<R>,
    name: &str,
) -> Result<Option<String>> {
    Ok(guard
        .read_named(zip, name)?
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned()))
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
    /// `style:text-position`: raised, lowered, or explicitly on the baseline.
    position: Option<TextPosition>,
}

/// Where `style:text-position` puts a run relative to the baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextPosition {
    Baseline,
    Raised,
    Lowered,
}

/// Read `style:text-position`: `super` or `sub`, or a percentage whose sign says which,
/// each optionally followed by the font scale.
fn text_position(value: &str) -> Option<TextPosition> {
    let first = value.split_whitespace().next()?;
    match first {
        "super" => return Some(TextPosition::Raised),
        "sub" => return Some(TextPosition::Lowered),
        _ => {}
    }
    let percent: f32 = first.strip_suffix('%')?.trim().parse().ok()?;
    Some(if percent > 0.0 {
        TextPosition::Raised
    } else if percent < 0.0 {
        TextPosition::Lowered
    } else {
        TextPosition::Baseline
    })
}

/// What a paragraph style's `style:paragraph-properties` says about alignment, direction
/// and page breaks, as written. `None` is silence, which defers to the parent style.
#[derive(Default, Clone)]
struct RawParagraphProps {
    /// `fo:text-align`.
    align: Option<String>,
    /// `style:writing-mode`.
    writing_mode: Option<String>,
    /// `fo:break-before`.
    break_before: Option<String>,
    /// `fo:break-after`.
    break_after: Option<String>,
}

#[derive(Default, Clone)]
struct RawParaStyle {
    parent: Option<String>,
    /// A bottom border with nothing else — the ODF spelling of a horizontal rule.
    rule: Option<bool>,
    /// `style:default-outline-level` — the heading depth this style *declares*.
    ///
    /// ODF lets a paragraph style say it is a heading, which is how a document can carry a
    /// full chapter structure without a single `<text:h>` in it. See [`StyleTable::outline_level`].
    outline_level: Option<u8>,
    /// The text properties a paragraph style carries in its own right; a run with
    /// no span of its own inherits them.
    text: RawTextStyle,
    paragraph: RawParagraphProps,
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
                let properties = node
                    .children()
                    .find(|c| is(c, NS_STYLE, "paragraph-properties"));
                let paragraph = properties
                    .map(|p| RawParagraphProps {
                        align: p.attribute((NS_FO, "text-align")).map(str::to_string),
                        writing_mode: p.attribute((NS_STYLE, "writing-mode")).map(str::to_string),
                        break_before: p.attribute((NS_FO, "break-before")).map(str::to_string),
                        break_after: p.attribute((NS_FO, "break-after")).map(str::to_string),
                    })
                    .unwrap_or_default();
                let rule = properties.map(|p| {
                    let bottom = p.attribute((NS_FO, "border-bottom"));
                    let all = p.attribute((NS_FO, "border"));
                    let has_bottom =
                        bottom.is_some_and(|v| v != "none") || all.is_some_and(|v| v != "none");
                    let sides_clear = ["border-top", "border-left", "border-right"]
                        .iter()
                        .all(|s| p.attribute((NS_FO, *s)).is_none_or(|v| v == "none"));
                    has_bottom && sides_clear
                });
                let outline_level = node
                    .attribute((NS_STYLE, "default-outline-level"))
                    .and_then(|v| v.trim().parse::<u8>().ok())
                    .filter(|l| (1..=10).contains(l));
                self.para.insert(
                    name.to_string(),
                    RawParaStyle {
                        parent: parent.clone(),
                        rule,
                        text,
                        outline_level,
                        paragraph,
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
            if let Some(position) = raw.position {
                style.superscript = position == TextPosition::Raised;
                style.subscript = position == TextPosition::Lowered;
            }
        }
        style
    }

    /// The first value `pick` finds in the paragraph style `name` or the ones it inherits
    /// from, the same walk and 32-step guard as [`Self::outline_level`].
    fn paragraph_value(
        &self,
        name: &str,
        pick: impl Fn(&RawParagraphProps) -> Option<&String>,
    ) -> Option<String> {
        let mut current = Some(name.to_string());
        let mut guard = 0;
        while let Some(n) = current {
            let raw = self.para.get(&n)?;
            if let Some(value) = pick(&raw.paragraph) {
                return Some(value.clone());
            }
            current = raw.parent.clone();
            guard += 1;
            if guard > 32 {
                return None;
            }
        }
        None
    }

    /// What the paragraph style `name` says about alignment, direction and a page break
    /// before the paragraph, its inheritance resolved.
    ///
    /// `fo:text-align` has two logical values, `start` and `end`, which name an edge only
    /// once the direction is known, and two physical ones, `left` and `right`.
    fn paragraph_props(&self, name: Option<&str>) -> BlockProps {
        let Some(name) = name else {
            return BlockProps::default();
        };
        let direction = self
            .paragraph_value(name, |p| p.writing_mode.as_ref())
            .and_then(|mode| match mode.as_str() {
                "rl-tb" | "rl" => Some(Direction::RightToLeft),
                "lr-tb" | "lr" => Some(Direction::LeftToRight),
                // `page` defers to the page, and the vertical modes have no counterpart.
                _ => None,
            });
        let right_to_left = direction == Some(Direction::RightToLeft);
        let (start, end) = if right_to_left {
            (Alignment::Right, Alignment::Left)
        } else {
            (Alignment::Left, Alignment::Right)
        };
        let alignment = self
            .paragraph_value(name, |p| p.align.as_ref())
            .and_then(|align| match align.as_str() {
                "center" => Some(Alignment::Center),
                "justify" => Some(Alignment::Justify),
                "start" => Some(start),
                "end" => Some(end),
                "left" => Some(Alignment::Left),
                "right" => Some(Alignment::Right),
                _ => None,
            });
        BlockProps {
            alignment,
            direction,
            page_break_before: self
                .paragraph_value(name, |p| p.break_before.as_ref())
                .is_some_and(|b| b == "page"),
        }
    }

    /// Whether the paragraph style `name` ends its page after the paragraph.
    fn breaks_page_after(&self, name: Option<&str>) -> bool {
        name.and_then(|n| self.paragraph_value(n, |p| p.break_after.as_ref()))
            .is_some_and(|b| b == "page")
    }

    /// Whether a paragraph style is the ODF spelling of a horizontal rule.
    /// The heading depth a paragraph style declares, walking the inheritance chain.
    ///
    /// # Why a `<text:p>` can be a heading
    ///
    /// This module's own doc says ODF states heading depth explicitly on `<text:h>`, so none of
    /// DOCX's style-name guesswork is needed. True, and incomplete: ODF lets a *paragraph
    /// style* declare the same thing with `style:default-outline-level`, and a document written
    /// from a novel template does exactly that — its chapters are `<text:p>` in a "Chapter
    /// title" style whose definition says `default-outline-level="2"`, with no `<text:h>`
    /// anywhere in the file. Read only as `<text:h>`, a 90 000-word manuscript with
    /// twenty-seven chapters arrives as one scene.
    ///
    /// This is still not name guessing — nothing here looks at what a style is *called*. The
    /// level is a value the document itself states, in the vocabulary's own attribute; it is
    /// simply stated in a second place, which the reader now looks at.
    ///
    /// The chain matters as much as the attribute: a real document applies an **automatic**
    /// style (`P12`) to each chapter, and only its `style:parent-style-name` reaches the named
    /// style carrying the level. Same walk, and same 32-step guard, as [`Self::is_rule`].
    fn outline_level(&self, name: &str) -> Option<u8> {
        let mut current = Some(name.to_string());
        let mut guard = 0;
        while let Some(n) = current {
            let raw = self.para.get(&n)?;
            if let Some(level) = raw.outline_level {
                return Some(level);
            }
            current = raw.parent.clone();
            guard += 1;
            if guard > 32 {
                return None;
            }
        }
        None
    }

    /// What a paragraph style claims this paragraph is — an epigraph, a quotation, or
    /// nothing — walking the inheritance chain the same way [`Self::outline_level`] does.
    ///
    /// The chain is the whole of it. Every paragraph this workspace's own ODT writer emits
    /// with an override carries an **automatic** style (`P1`) whose only link to the named
    /// `Epigraph` is its `style:parent-style-name`; checking the applied name alone would
    /// recognise an epigraph in a default-options export (where `paragraph_style` shortcuts
    /// to the parent) and miss it in every other, which is the worst of both.
    ///
    /// Matching on `style:name` rather than a style id is correct *for ODF specifically*:
    /// unlike OOXML it does not localize style names, so the stored name is the stable
    /// identifier here. See [`rich::styled_as`], which owns the vocabulary both scanners
    /// share.
    fn quoted_as(&self, name: &str) -> Option<rich::StyledAs> {
        let mut current = Some(name.to_string());
        let mut guard = 0;
        while let Some(n) = current {
            if let Some(styled) = rich::styled_as(&n) {
                return Some(styled);
            }
            current = self.para.get(&n)?.parent.clone();
            guard += 1;
            if guard > 32 {
                return None;
            }
        }
        None
    }

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
    if let Some(position) = props.attribute((NS_STYLE, "text-position")) {
        raw.position = text_position(position);
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
    row_marks: Vec<RichRowMark>,
    /// Comment marks closed so far, matched to their annotations in `finish`.
    comment_marks: Vec<CommentMark>,
    /// Bookmark name → the comment mark it opened.
    open_marks: HashMap<String, OpenMark>,
    /// `office:name` → index into `annotations`, for `annotation-end` and for
    /// `loext:parent-name`.
    by_name: HashMap<String, usize>,
    open: HashMap<String, OpenRange>,
    diagnostics: Vec<ImportDiagnostic>,
    tracked_changes: usize,
    /// Who made them, first-seen order — see the DOCX scanner's own
    /// `Walker::tracked_authors`. ODF puts these in one place, `office:change-info`
    /// inside each `text:changed-region`, so they are read where the regions are
    /// counted rather than gathered as the prose is walked.
    tracked_authors: Vec<String>,
    text_boxes: usize,
    embedded_objects: usize,
    fields: usize,
    /// Notes the walk could not carry — see [`Walker::note`]. Not a count of the
    /// document's footnotes: the ones it did carry are in [`Self::footnotes`].
    footnotes_dropped: usize,
    /// One entry per note whose reference reached the prose, deduplicated by label.
    footnotes: Vec<rich::RichFootnote>,
    orphan_replies: usize,
    /// `<text:s>` runs whose count was cut to [`MAX_SPACE_RUN`], in prose, comments and
    /// notes alike.
    spaces_shortened: usize,
    /// A page ended after the last block (`fo:break-after="page"`), so the next block the
    /// walk produces starts a new one.
    page_break_pending: bool,
}

/// The paragraph currently being built.
struct ParaBuild {
    kind: ParagraphKind,
    runs: Vec<Run>,
    len: usize,
    props: BlockProps,
    /// Whether a white-space character read now is dropped: at the start of the paragraph,
    /// and right after another one. See [`collapse_space`].
    after_space: bool,
    /// Whether this is a table cell's text, where a line break is a space: a cell is one
    /// block of its table, never a paragraph of its own.
    in_cell: bool,
}

impl ParaBuild {
    fn new(kind: ParagraphKind, props: BlockProps) -> Self {
        ParaBuild {
            kind,
            runs: Vec::new(),
            len: 0,
            props,
            after_space: true,
            in_cell: false,
        }
    }

    /// The rest of the same paragraph after a line break: same kind, same alignment and
    /// direction, and not the start of a page.
    fn continuation(&self) -> ParaBuild {
        ParaBuild::new(
            self.kind,
            BlockProps {
                page_break_before: false,
                ..self.props
            },
        )
    }
}

/// Character data under ODF's white-space rule (ODF 1.2 part 1, 6.1.2): a tab, a line feed
/// or a carriage return is a space, a run of spaces is one space, and a space at the start
/// of a paragraph or right after another one is dropped, across element boundaries.
///
/// That is how every consumer reads it, LibreOffice included; a producer writes the
/// spaces it means as `<text:s/>` and its tabs as `<text:tab/>`, which the walk takes
/// literally. `after_space` carries the state from one piece of character data to the
/// next.
fn collapse_space(text: &str, after_space: &mut bool) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, ' ' | '\t' | '\n' | '\r') {
            if !*after_space {
                out.push(' ');
                *after_space = true;
            }
        } else {
            out.push(c);
            *after_space = false;
        }
    }
    out
}

impl<'a> Walker<'a> {
    fn new(styles: &'a StyleTable, origin: &str) -> Self {
        Walker {
            styles,
            origin: origin.to_string(),
            blocks: Vec::new(),
            annotations: Vec::new(),
            row_marks: Vec::new(),
            comment_marks: Vec::new(),
            open_marks: HashMap::new(),
            by_name: HashMap::new(),
            open: HashMap::new(),
            diagnostics: Vec::new(),
            tracked_changes: 0,
            tracked_authors: Vec::new(),
            text_boxes: 0,
            embedded_objects: 0,
            fields: 0,
            footnotes_dropped: 0,
            footnotes: Vec::new(),
            orphan_replies: 0,
            spaces_shortened: 0,
            page_break_pending: false,
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
                    authors: self.tracked_authors.clone(),
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
                self.footnotes_dropped,
                ImportDiagnostic::FootnoteNotCarried {
                    path: self.origin.clone(),
                    count: self.footnotes_dropped,
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
        if self.spaces_shortened > 0 {
            self.diagnostics.push(ImportDiagnostic::SpacesShortened {
                path: self.origin.clone(),
                count: self.spaces_shortened,
                limit: MAX_SPACE_RUN,
            });
        }

        attach_comment_marks(&mut self.annotations, &self.comment_marks);

        RichDocument {
            blocks: std::mem::take(&mut self.blocks),
            annotations: std::mem::take(&mut self.annotations),
            row_marks: std::mem::take(&mut self.row_marks),
            footnotes: std::mem::take(&mut self.footnotes),
        }
    }

    /// Walk anything that holds block-level content.
    ///
    /// `depth` is `node`'s own nesting, the root counting as one, and every walk
    /// below carries it the same way. The parse refused anything nested past
    /// `MAX_XML_DEPTH`, so [`past_the_ceiling`] never stops a walk over a document
    /// read here; it keeps each recursion bounded by that ceiling on its own terms.
    fn walk_container(&mut self, node: Node<'_, '_>, depth: usize) {
        if past_the_ceiling(depth) {
            return;
        }
        for child in node.children().filter(Node::is_element) {
            let name = child.tag_name().name();
            match (child.tag_name().namespace(), name) {
                (Some(NS_TEXT), "h") => {
                    let level = child
                        .attribute((NS_TEXT, "outline-level"))
                        .and_then(|v| v.parse::<u8>().ok())
                        .unwrap_or(1)
                        .max(1);
                    self.paragraph(child, ParagraphKind::Heading { level }, depth + 1);
                }
                (Some(NS_TEXT), "p") => {
                    // A rule-styled paragraph is a thematic break — see the module
                    // note. It is emitted as its glyph so the vocabulary in
                    // `skribisto_model` stays the one authority on what a break is.
                    let styled_as_rule = child
                        .attribute((NS_TEXT, "style-name"))
                        .is_some_and(|s| self.styles.is_rule(s));
                    if styled_as_rule && element_text(child).trim().is_empty() {
                        // A scene break carries no paragraph formatting, so a pending
                        // page break ends here.
                        self.page_break_pending = false;
                        self.push_block(RichBlock::body(vec![Run::plain(
                            skribisto_model::scene_break::CANONICAL_MINOR,
                        )]));
                        continue;
                    }
                    // A paragraph whose style declares an outline level *is* a heading — the
                    // shape every novel-template document uses, where chapters are styled
                    // `<text:p>` and the file contains no `<text:h>` at all. See
                    // `StyleTable::outline_level`. An empty one is not: a blank paragraph
                    // left in a heading style is spacing, and promoting it would open a
                    // titleless chapter.
                    let declared = child
                        .attribute((NS_TEXT, "style-name"))
                        .and_then(|s| self.styles.outline_level(s));
                    match declared {
                        Some(level) if !element_text(child).trim().is_empty() => self.paragraph(
                            child,
                            ParagraphKind::Heading {
                                level: level.max(1),
                            },
                            depth + 1,
                        ),
                        // …and a paragraph whose style names it an epigraph or a quotation is
                        // that, on the same terms: a value the document states about itself,
                        // asked *after* the outline level so a style that somehow declared
                        // both is still read as the heading it says it is.
                        _ => {
                            let styled = child
                                .attribute((NS_TEXT, "style-name"))
                                .and_then(|s| self.styles.quoted_as(s));
                            self.paragraph(
                                child,
                                styled.map_or(ParagraphKind::Body, rich::kind_for_style),
                                depth + 1,
                            )
                        }
                    }
                }
                (Some(NS_TEXT), "list") => self.walk_list(child, 0, depth + 1),
                (Some(NS_TEXT), "section") => self.walk_container(child, depth + 1),
                (Some(NS_TEXT), "tracked-changes") => {
                    self.tracked_changes += child
                        .children()
                        .filter(|c| is(c, NS_TEXT, "changed-region"))
                        .count();
                    // `<office:change-info><dc:creator>` inside each region. Read
                    // by descent rather than by an exact path: a region wraps its
                    // change-info in `<text:insertion>` or `<text:deletion>`, and
                    // ODF allows further nesting for a format change.
                    for creator in child
                        .descendants()
                        .filter(|c| is(c, NS_DC, "creator"))
                        .map(element_text)
                    {
                        let creator = creator.trim();
                        if !creator.is_empty() && !self.tracked_authors.iter().any(|a| a == creator)
                        {
                            self.tracked_authors.push(creator.to_string());
                        }
                    }
                }
                (Some(NS_TABLE), "table") => self.walk_table(child),
                (Some(NS_TEXT), "soft-page-break") | (Some(NS_TEXT), "sequence-decls") => {}
                // Anything else that could hold paragraphs (index bodies, frames at
                // body level, change marks). Recursing beats ignoring: an unread
                // container is prose the writer wrote and never saw again.
                _ => self.walk_container(child, depth + 1),
            }
        }
    }

    /// `list_depth` is how many lists this one sits inside; `depth` is its element
    /// nesting, as for [`Self::walk_container`]. An item's content sits two levels
    /// under the list, one for the `<text:list-item>` holding it.
    fn walk_list(&mut self, node: Node<'_, '_>, list_depth: u8, depth: usize) {
        if past_the_ceiling(depth) {
            return;
        }
        let ordered = self
            .styles
            .list_is_ordered(node.attribute((NS_TEXT, "style-name")), list_depth);
        for item in node.children().filter(Node::is_element) {
            if !matches!(item.tag_name().name(), "list-item" | "list-header") {
                continue;
            }
            for child in item.children().filter(Node::is_element) {
                match (child.tag_name().namespace(), child.tag_name().name()) {
                    (Some(NS_TEXT), "p") | (Some(NS_TEXT), "h") => self.paragraph(
                        child,
                        ParagraphKind::ListItem {
                            ordered,
                            depth: list_depth,
                        },
                        depth + 2,
                    ),
                    (Some(NS_TEXT), "list") => {
                        self.walk_list(child, list_depth.saturating_add(1), depth + 2)
                    }
                    _ => {}
                }
            }
        }
    }

    /// A table, every cell read with the same inline walk a paragraph gets.
    ///
    /// One buffer runs through the whole table, so its offsets are the table's own plain
    /// text (cells joined by a line break, a cell's paragraphs by a space): the space a
    /// comment or a round-trip mark inside a cell is measured in, which is what lets
    /// `rich::assemble` place it on its words. A cell's text is its paragraphs' text and
    /// nothing else; a comment's author, date and body, or a note's text, are not the
    /// cell's words.
    fn walk_table(&mut self, node: Node<'_, '_>) {
        let mut build = ParaBuild::new(ParagraphKind::Body, BlockProps::default());
        build.in_cell = true;
        let mut rows: Vec<Vec<Vec<Run>>> = Vec::new();
        let mut first_cell = true;
        for row in node.descendants().filter(|n| is(n, NS_TABLE, "table-row")) {
            let mut cells: Vec<Vec<Run>> = Vec::new();
            for cell in row.children().filter(|c| is(c, NS_TABLE, "table-cell")) {
                if !first_cell {
                    build.len += 1;
                }
                first_cell = false;
                let start = build.runs.len();
                for (i, paragraph) in cell_paragraphs(cell).into_iter().enumerate() {
                    if i > 0 {
                        build.len += 1;
                        build.runs.push(Run::plain(" "));
                    }
                    build.after_space = true;
                    let base = paragraph
                        .attribute((NS_TEXT, "style-name"))
                        .map(|s| self.styles.text_style(s))
                        .unwrap_or_default();
                    // Cells are found through `descendants`, not by a walk that
                    // carries the count down, so each paragraph measures its own.
                    self.inline(paragraph, &mut build, base, None, element_depth(paragraph));
                }
                cells.push(build.runs.split_off(start));
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
        if !rows.is_empty() {
            // A table carries no paragraph formatting, so a page break before it is not
            // carried either.
            self.page_break_pending = false;
            self.push_block(RichBlock::Table { rows });
        }
    }

    fn paragraph(&mut self, node: Node<'_, '_>, kind: ParagraphKind, depth: usize) {
        let style_name = node.attribute((NS_TEXT, "style-name"));
        let base = style_name
            .map(|s| self.styles.text_style(s))
            .unwrap_or_default();
        let mut build = ParaBuild::new(kind, self.styles.paragraph_props(style_name));
        self.inline(node, &mut build, base, None, depth);
        self.flush_paragraph(build);
        if self.styles.breaks_page_after(style_name) {
            self.page_break_pending = true;
        }
    }

    /// Finish the paragraph under construction. Also what a deliberate line break does,
    /// the rest of the line starting a paragraph of its own: Djot has no line break inside
    /// a paragraph that the editor keeps.
    fn flush_paragraph(&mut self, build: ParaBuild) {
        if build.runs.is_empty() {
            return;
        }
        let mut props = build.props;
        props.page_break_before |= std::mem::take(&mut self.page_break_pending);
        self.push_block(RichBlock::Paragraph {
            kind: build.kind,
            runs: build.runs,
            props,
        });
    }

    fn push_block(&mut self, block: RichBlock) {
        self.blocks.push(block);
    }

    /// Walk one paragraph's inline content, accumulating runs and tracking where
    /// each comment range opens and closes. `depth` as for [`Self::walk_container`].
    fn inline(
        &mut self,
        node: Node<'_, '_>,
        build: &mut ParaBuild,
        style: RunStyle,
        link: Option<&str>,
        depth: usize,
    ) {
        if past_the_ceiling(depth) {
            return;
        }
        for child in node.children() {
            if child.is_text() {
                let text = collapse_space(child.text().unwrap_or_default(), &mut build.after_space);
                if !text.is_empty() {
                    build.len += text.chars().count();
                    build.runs.push(Run {
                        text,
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
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
                    self.inline(child, build, inner, link, depth + 1);
                }
                (Some(NS_TEXT), "a") => {
                    let url = child.attribute((NS_XLINK, "href")).unwrap_or_default();
                    self.inline(child, build, style, Some(url), depth + 1);
                }
                (Some(NS_TEXT), "s") => {
                    let (count, shortened) = space_run(child);
                    self.spaces_shortened += usize::from(shortened);
                    let spaces = " ".repeat(count);
                    build.len += count;
                    // Written spaces are the producer's own, never collapsed, and a raw
                    // space after them is kept.
                    build.after_space = false;
                    build.runs.push(Run {
                        text: spaces,
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                (Some(NS_TEXT), "tab") => {
                    build.len += 1;
                    build.after_space = false;
                    build.runs.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                (Some(NS_TEXT), "line-break") if build.in_cell => {
                    build.len += 1;
                    build.after_space = true;
                    build.runs.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                (Some(NS_TEXT), "line-break") => {
                    let next = build.continuation();
                    let finished = std::mem::replace(build, next);
                    self.flush_paragraph(finished);
                }
                (Some(NS_OFFICE), "annotation") => self.open_annotation(child, build, depth + 1),
                (Some(NS_OFFICE), "annotation-end") => {
                    if let Some(name) = child.attribute((NS_OFFICE, "name")) {
                        self.close_annotation(name, build);
                    }
                }
                (Some(NS_TEXT), "note") => self.note(child, build, depth + 1),
                (Some(NS_DRAW), "frame") | (Some(NS_DRAW), "g") => self.frame(child, build, style),
                // A bookmark is ordinarily nothing to a manuscript importer — a
                // cross-reference target, a table-of-contents entry, LibreOffice's own
                // `__Fieldmark__`. Three names are not: the round-trip marks this app writes
                // into its own exports (`skrb_r…`, `skrb_c…`). Everything else still falls
                // through to being ignored, and `round_trip::parse_mark_name` is strict about
                // the shape precisely so a foreign bookmark cannot be mistaken for identity.
                (Some(NS_TEXT), "bookmark") => self.point_mark(child, build),
                (Some(NS_TEXT), "bookmark-start") => self.open_mark(child, build),
                (Some(NS_TEXT), "bookmark-end") => self.close_mark(child, build),
                (Some(NS_TEXT), "change-start")
                | (Some(NS_TEXT), "change-end")
                | (Some(NS_TEXT), "change")
                | (Some(NS_TEXT), "soft-page-break") => {}
                (Some(NS_TEXT), field) if FIELD_ELEMENTS.contains(&field) => {
                    self.fields += 1;
                    self.inline(child, build, style, link, depth + 1);
                }
                // Unknown inline element: take its text rather than drop it.
                _ => self.inline(child, build, style, link, depth + 1),
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
                    // The frame, not the image, states how large the picture is shown.
                    let size = |name: &str| {
                        node.attribute((NS_SVG, name))
                            .and_then(length_in_pixels)
                            .unwrap_or(0)
                    };
                    build.len += 1;
                    build.after_space = false;
                    build
                        .runs
                        .push(Run::sized_image(alt, src, size("width"), size("height")));
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

    /// `<text:bookmark>` — a zero-length mark. A row mark is written this way; a comment mark
    /// never is (it always brackets characters), so one arriving here is a degenerate write we
    /// have no use for and ignore rather than guess at.
    fn point_mark(&mut self, node: Node<'_, '_>, build: &ParaBuild) {
        let Some(name) = node.attribute((NS_TEXT, "name")) else {
            return;
        };
        if let Some(round_trip::MarkName::Row { uid_tag, digest }) =
            round_trip::parse_mark_name(name)
        {
            self.row_marks.push(RichRowMark {
                block_index: self.blocks.len(),
                uid_tag,
                digest,
            });
        }
        let _ = build;
    }

    /// `<text:bookmark-start>` — opens a comment mark's range. A *row* mark arriving as a range
    /// is accepted at its start: ODF permits it, and a row mark's extent has never meant
    /// anything (it names a position), so there is nothing to lose by taking the position and
    /// letting the matching end tag fall through.
    fn open_mark(&mut self, node: Node<'_, '_>, build: &ParaBuild) {
        let Some(name) = node.attribute((NS_TEXT, "name")) else {
            return;
        };
        match round_trip::parse_mark_name(name) {
            Some(round_trip::MarkName::Row { uid_tag, digest }) => {
                self.row_marks.push(RichRowMark {
                    block_index: self.blocks.len(),
                    uid_tag,
                    digest,
                });
            }
            Some(round_trip::MarkName::Comment { uid_tag }) => {
                self.open_marks.insert(
                    name.to_string(),
                    OpenMark {
                        uid_tag,
                        block: self.blocks.len(),
                        start: build.len,
                    },
                );
            }
            None => {}
        }
    }

    /// `<text:bookmark-end>` — closes a comment mark, giving the exact character range the
    /// comment covered when it was written. Recorded for `finish` to match against the
    /// annotations, which cannot be done here: on a file an editor has saved, the annotation
    /// and its mark are two independent elements whose order is the editor's to choose.
    fn close_mark(&mut self, node: Node<'_, '_>, build: &ParaBuild) {
        let Some(name) = node.attribute((NS_TEXT, "name")) else {
            return;
        };
        if let Some(open) = self.open_marks.remove(name) {
            self.comment_marks.push(CommentMark {
                uid_tag: open.uid_tag,
                block: open.block,
                start: open.start,
                // A mark that opened in an earlier block has no meaningful length here; the
                // start is what identifies it, and `finish` matches on that.
                length: if open.block == self.blocks.len() {
                    build.len.saturating_sub(open.start)
                } else {
                    0
                },
            });
        }
    }

    /// `depth` is the annotation's own nesting, as for [`Self::walk_container`].
    fn open_annotation(&mut self, node: Node<'_, '_>, build: &ParaBuild, depth: usize) {
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
        // One `Vec<Run>` per `<text:p>` — the comment's own text, styled but not
        // yet Djot. `rich::assemble` converts it, alongside manuscript prose,
        // rather than this scanner reimplementing that pipeline for a comment's
        // paragraphs. `element_text`'s flat concatenation used to be used here,
        // which is why a bold or italic word inside a comment used to vanish on
        // import: it reads every text node under the annotation regardless of
        // the `text:span` carrying it, so the run's own formatting was never
        // even looked at.
        // One builder across every `<text:p>`, not one buffer per paragraph: a line break
        // *inside* a paragraph has to be able to end it, which a per-paragraph buffer
        // cannot express. Each `<text:p>` closes its own paragraph on the way out, so
        // Enter (separate elements) and Shift+Enter (`<text:line-break/>`) both arrive as
        // real paragraph boundaries and read back identically.
        let mut body = AnnotationBody::default();
        for p in node.children().filter(|c| is(c, NS_TEXT, "p")) {
            let base = p
                .attribute((NS_TEXT, "style-name"))
                .map(|s| self.styles.text_style(s))
                .unwrap_or_default();
            self.annotation_inline(p, base, None, &mut body, depth + 1);
            body.break_paragraph();
        }
        self.spaces_shortened += body.spaces_shortened;
        let paragraphs: Vec<Vec<Run>> = body.finish();
        let resolved = node
            .attribute((NS_LOEXT, "resolved"))
            .is_some_and(|v| v == "true");
        let name = node.attribute((NS_OFFICE, "name"));
        // Only Skribisto's own writer puts this attribute on `<office:annotation>` —
        // an unparsable or absent value (a plain LibreOffice comment) is `None`, not
        // an error: the comment is still imported, just as one an editor authored
        // fresh. See `NS_SKRB`'s own doc.
        let uid = node
            .attribute((NS_SKRB, "uid"))
            .and_then(|v| uuid::Uuid::parse_str(v).ok());

        // A reply joins its parent's thread rather than becoming a comment.
        if let Some(parent) = node.attribute((NS_LOEXT, "parent-name")) {
            match self.by_name.get(parent).copied() {
                Some(index) => {
                    self.annotations[index].replies.push(RichReply {
                        uid,
                        author,
                        // ODF has no carrier for this at all — see `NS_SKRB`'s doc.
                        author_initials: String::new(),
                        created,
                        paragraphs,
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
            end: None,
            uid,
            // Filled by `attach_comment_marks` once the whole document is walked — the
            // bookmark carrying it may close after this point, and on a file an editor has
            // saved it may even precede the annotation.
            uid_tag: None,
            author,
            author_initials: String::new(),
            created,
            paragraphs,
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

    /// A `<text:note>` — a footnote or an endnote, which ODF spells the same way.
    ///
    /// Easier than OOXML's, and in one specific way: the note's body sits **inline**,
    /// right where its reference does, so there is no second part to read and no
    /// offset to reconstruct. The reference becomes a run at the point the walk has
    /// already reached, which is by construction the point it belongs at.
    ///
    /// `<text:note-citation>` — the number LibreOffice printed — is deliberately not
    /// read. The number is derived from position by `skribisto_model::footnote_numbering`
    /// and never stored, so carrying the source's would be a second answer to the
    /// same question, and a wrong one the moment the note lands anywhere else in the
    /// book.
    ///
    /// `depth` is the note's own nesting, as for [`Self::walk_container`].
    fn note(&mut self, node: Node<'_, '_>, build: &mut ParaBuild, depth: usize) {
        let Some(id) = node.attribute((NS_TEXT, "id")).map(sanitise_note_id) else {
            self.footnotes_dropped += 1;
            return;
        };
        let Some(body_node) = node.children().find(|c| is(c, NS_TEXT, "note-body")) else {
            self.footnotes_dropped += 1;
            return;
        };
        let mut body = AnnotationBody::default();
        for p in body_node.children().filter(|c| is(c, NS_TEXT, "p")) {
            let base = p
                .attribute((NS_TEXT, "style-name"))
                .map(|s| self.styles.text_style(s))
                .unwrap_or_default();
            self.annotation_inline(p, base, None, &mut body, depth + 2);
            body.break_paragraph();
        }
        self.spaces_shortened += body.spaces_shortened;
        let paragraphs = body.finish();
        if paragraphs.is_empty() {
            // A note with an empty body: the marker would cite nothing.
            self.footnotes_dropped += 1;
            return;
        }
        let label = format!("{}{id}", crate::sources::docx::FOOTNOTE_LABEL_PREFIX);
        // **Advances `build.len` by one, and it has to.** A footnote reference carries
        // no text, but it is one object-replacement character in the plain text every
        // comment offset in this paragraph is measured against — see
        // `rich::Run::plain_push`. Counting it as nothing would put every comment after
        // a note in the same paragraph one character early, silently: the quote would
        // be captured a character off, on the wrong words.
        //
        // The DOCX scanner reaches the same place by a different road (`shift_offsets`,
        // after the fact) only because its references come from a raw pass that runs
        // after the typed walk. Here the note *is* the walk, so the counter can simply
        // be right the first time.
        build.len += 1;
        build.after_space = false;
        build.runs.push(Run {
            footnote: Some(label.clone()),
            ..Default::default()
        });
        if !self.footnotes.iter().any(|f| f.label == label) {
            self.footnotes
                .push(rich::RichFootnote { label, paragraphs });
        }
    }

    /// Character-styled runs for one paragraph of a comment or reply's own
    /// text — the same `text:span`/`text:a`/`text:s`/`text:tab` vocabulary
    /// [`Self::inline`] reads for manuscript prose, narrowed to what an
    /// annotation's own `<text:p>` can actually hold.
    ///
    /// A separate method rather than a call into [`Self::inline`] itself: that
    /// one also handles footnotes, frames, tables and nested annotations, all of
    /// which touch `self`'s diagnostic counters (`self.footnotes`,
    /// `self.text_boxes`, …) and none of which a comment's own body can contain
    /// — LibreOffice offers no UI to put a footnote or a frame *inside* a
    /// comment, and no UI to comment on a comment either. Reusing `inline`
    /// directly would silently start counting those against the manuscript's
    /// own diagnostics for constructs that live in a margin note instead.
    ///
    /// `depth` as for [`Self::walk_container`].
    fn annotation_inline(
        &self,
        node: Node<'_, '_>,
        style: RunStyle,
        link: Option<&str>,
        out: &mut AnnotationBody,
        depth: usize,
    ) {
        if past_the_ceiling(depth) {
            return;
        }
        for child in node.children() {
            if child.is_text() {
                let text = collapse_space(child.text().unwrap_or_default(), &mut out.after_space);
                if !text.is_empty() {
                    out.push(Run {
                        text,
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                continue;
            }
            if !child.is_element() {
                continue;
            }
            match (child.tag_name().namespace(), child.tag_name().name()) {
                (Some(NS_TEXT), "span") => {
                    let inner = child
                        .attribute((NS_TEXT, "style-name"))
                        .map(|s| merge(style, self.styles.text_style(s)))
                        .unwrap_or(style);
                    self.annotation_inline(child, inner, link, out, depth + 1);
                }
                (Some(NS_TEXT), "a") => {
                    let url = child.attribute((NS_XLINK, "href")).unwrap_or_default();
                    self.annotation_inline(child, style, Some(url), out, depth + 1);
                }
                (Some(NS_TEXT), "s") => {
                    let (count, shortened) = space_run(child);
                    out.spaces_shortened += usize::from(shortened);
                    out.after_space = false;
                    out.push(Run {
                        text: " ".repeat(count),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                // A tab collapses to a single space: a comment box has no tab stops of
                // its own to honour, so the words either side still read correctly and
                // only the exact whitespace differs.
                (Some(NS_TEXT), "tab") => {
                    out.after_space = false;
                    out.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                // A deliberate line break starts a new paragraph, exactly as it does for
                // manuscript prose in [`Self::inline`] and for a `.docx` comment in
                // `docx::CommentBodyBuilder`. An editor who writes a two-line note with
                // Shift+Enter meant two lines; collapsing them to a space silently
                // reflows their remark into one run-on sentence, and — worse — makes the
                // same note import differently depending on whether it was written in
                // LibreOffice or in Word.
                (Some(NS_TEXT), "line-break") => out.break_paragraph(),
                // Unknown inline element: take its text rather than drop it,
                // matching `Self::inline`'s own fallback.
                _ => self.annotation_inline(child, style, link, out, depth + 1),
            }
        }
    }

    /// Close a comment range.
    ///
    /// A range that ends in a *later* paragraph keeps its length to the end of the one
    /// it started in, and where it really ends beside it. `rich::assemble` keeps the
    /// whole extent when both ends land in the same stretch of stored prose, and
    /// otherwise stops it at the end of its first paragraph: the quote still begins on
    /// the right words, and a range whose end nobody can place is worse than a short one.
    fn close_annotation(&mut self, name: &str, build: &ParaBuild) {
        let Some(open) = self.open.remove(name) else {
            return;
        };
        let current = self.blocks.len();
        let annotation = &mut self.annotations[open.annotation];
        if open.block == current {
            annotation.length = build.len.saturating_sub(open.start);
            return;
        }
        let start_len = self
            .blocks
            .get(open.block)
            .map(|b| b.plain_text().chars().count())
            .unwrap_or(open.start);
        annotation.length = start_len.saturating_sub(open.start);
        annotation.end = (current > open.block).then_some(AnnotationEnd {
            block_index: current,
            offset: build.len,
        });
    }
}

/// A comment or reply body under construction: finished paragraphs, plus the one still
/// being filled.
///
/// The ODF counterpart of `docx::CommentBodyBuilder`, and it exists for the same reason.
/// A body is `Vec<Vec<Run>>` — one inner vector per paragraph — but the runs arrive from a
/// recursive walk that cannot see paragraph boundaries, and a `<text:line-break/>` can end
/// a paragraph from *inside* one. Threading a single builder through the walk is what lets
/// the break reach the outer vector; a plain `&mut Vec<Run>` per paragraph structurally
/// cannot, which is exactly how a Shift+Enter in a LibreOffice comment used to arrive as a
/// space.
struct AnnotationBody {
    paragraphs: Vec<Vec<Run>>,
    current: Vec<Run>,
    /// The [`collapse_space`] state of the paragraph in progress.
    after_space: bool,
    /// `<text:s>` runs cut to [`MAX_SPACE_RUN`] in this body. Counted here because
    /// [`Walker::annotation_inline`] reads the walker without changing it; whoever
    /// finishes the body adds this to the walker's own count.
    spaces_shortened: usize,
}

impl Default for AnnotationBody {
    fn default() -> Self {
        AnnotationBody {
            paragraphs: Vec::new(),
            current: Vec::new(),
            after_space: true,
            spaces_shortened: 0,
        }
    }
}

impl AnnotationBody {
    fn push(&mut self, run: Run) {
        self.current.push(run);
    }

    /// End the paragraph in progress.
    ///
    /// A no-op when nothing has been collected, so consecutive breaks, and the closing
    /// break every `<text:p>` performs, cannot manufacture empty paragraphs. White space
    /// at the start of the next paragraph is dropped again.
    fn break_paragraph(&mut self) {
        self.after_space = true;
        if !self.current.is_empty() {
            self.paragraphs.push(std::mem::take(&mut self.current));
        }
    }

    fn finish(mut self) -> Vec<Vec<Run>> {
        self.break_paragraph();
        self.paragraphs
    }
}

/// An ODF `text:id` narrowed to what a Djot footnote label may hold.
///
/// ODF ids are NCNames in practice (`ftn1`, `endnote-3`), but the attribute is not
/// schema-constrained in the wild and a label carrying `]`, `^` or a space would
/// either break the `[^label]` reference or, worse, match a *different* one. Anything
/// outside `[A-Za-z0-9_-]` becomes `_`, which cannot collide across two ids that
/// differ only in a character both map to — because the id also has to be unique in
/// the document for the reference to have found it in the first place, and two
/// distinct ids mapping to one label is exactly what the deduplication in
/// [`Walker::note`] would then treat as one note.
///
/// So collisions are made impossible rather than merely unlikely: the sanitised id
/// is suffixed with the original's length whenever sanitising changed anything.
fn sanitise_note_id(id: &str) -> String {
    let clean: String = id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if clean == id {
        clean
    } else {
        format!("{clean}_{:x}", fnv1a(id))
    }
}

/// FNV-1a over the raw id, so a sanitised label still names exactly one source id.
fn fnv1a(value: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

const NS_SVG: &str = "urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0";

/// An ODF length (`3.5cm`, `2in`, `120pt`) in pixels at 96 to the inch, the unit the
/// editor measures a picture in. `None` for a unit it does not know, or a length that
/// rounds to nothing.
fn length_in_pixels(value: &str) -> Option<u32> {
    let value = value.trim();
    let split = value.find(|c: char| c.is_ascii_alphabetic())?;
    let (number, unit) = value.split_at(split);
    let number: f64 = number.trim().parse().ok()?;
    let per_unit = match unit {
        "in" => 96.0,
        "cm" => 96.0 / 2.54,
        "mm" => 96.0 / 25.4,
        "pt" => 96.0 / 72.0,
        "pc" => 16.0,
        "px" => 1.0,
        _ => return None,
    };
    let pixels = (number * per_unit).round();
    (pixels.is_finite() && pixels >= 1.0 && pixels <= f64::from(u32::MAX)).then_some(pixels as u32)
}

/// Character formatting from an inner span applied over what it inherits.
///
/// ODF spans do not carry "off" switches for what a parent turned on, so an inner
/// span can only add — which is what a reader of the document sees.
fn merge(outer: RunStyle, inner: RunStyle) -> RunStyle {
    // A span that raises or lowers its text decides the position; one that says nothing
    // about it keeps the outer one.
    let (superscript, subscript) = if inner.superscript || inner.subscript {
        (inner.superscript, inner.subscript)
    } else {
        (outer.superscript, outer.subscript)
    };
    RunStyle {
        bold: outer.bold || inner.bold,
        italic: outer.italic || inner.italic,
        underline: outer.underline || inner.underline,
        strikethrough: outer.strikethrough || inner.strikethrough,
        code: outer.code || inner.code,
        superscript,
        subscript,
    }
}

/// Whether an element `depth` levels deep lies past the ceiling the parse holds
/// every document to, and so must not be walked.
fn past_the_ceiling(depth: usize) -> bool {
    depth > skrib_format::MAX_XML_DEPTH
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

/// The paragraphs of one table cell, in order: its own, whether directly inside it or
/// inside a list or a section it holds, and not those of a table nested in it (whose rows
/// the table walk meets on their own) nor the text of a comment or a note sitting in it.
fn cell_paragraphs<'a, 'input>(cell: Node<'a, 'input>) -> Vec<Node<'a, 'input>> {
    cell.descendants()
        .filter(|n| is(n, NS_TEXT, "p") || is(n, NS_TEXT, "h"))
        .filter(|paragraph| {
            for ancestor in paragraph.ancestors().skip(1) {
                if ancestor == cell {
                    return true;
                }
                if is(&ancestor, NS_TABLE, "table-cell")
                    || is(&ancestor, NS_OFFICE, "annotation")
                    || is(&ancestor, NS_TEXT, "note")
                {
                    return false;
                }
            }
            false
        })
        .collect()
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
