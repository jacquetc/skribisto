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
//! Three constructs a manuscript genuinely uses are invisible to the typed reader, and
//! `RawScan` reads them straight out of the container's own XML — two from
//! `word/document.xml`, one from `word/comments.xml`:
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
//! * **A comment's `skrb:uid` and `w:initials` (M-S7).** `docx_rs::Comment` (the
//!   typed reader's own comment type) carries neither field at all — verified
//!   against its actual source, not assumed (see `RawScan`'s own doc). Only
//!   Skribisto's own DOCX writer (`text-document`'s `export_docx_uc::patch_comment_extras`)
//!   ever puts a `skrb:uid` on a `<w:comment>`, so reading it back here is what lets
//!   `apply_document_import_uc` recognise a comment it already created on a previous
//!   export, instead of duplicating it on every round trip.
//!
//! It is a small number of extra reads over the same zip, and each answers a question
//! the typed tree cannot. The alternative was silent loss on every one of them.

use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use docx_rs::{
    Bold, Break, BreakType, Comment, CommentChild, CommentRangeEnd, DeleteChild, DocumentChild,
    Docx, DrawingData, Insert, InsertChild, Italic, MoveToChild, Paragraph, ParagraphChild,
    ParagraphProperty, Run as DocxRun, RunChild, RunProperty, Style, Table, TableCellContent,
    TableChild, TableRowChild, Underline, VertAlign, VertAlignType,
};

use crate::block::{SourceBlock, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::scanner::SourceScanner;
use crate::sources::rich;
use crate::sources::rich::{
    Alignment, AnnotationEnd, BlockProps, CommentMark, Direction, OpenMark, ParagraphKind,
    RichAnnotation, RichBlock, RichDocument, RichReply, RichRowMark, Run, RunStyle, assemble,
    attach_comment_marks,
};
use skribisto_model::round_trip;

pub struct DocxScanner;

impl SourceScanner for DocxScanner {
    fn extensions(&self) -> &[&str] {
        &["docx"]
    }

    fn format_name(&self) -> &'static str {
        "office-open-xml"
    }

    /// The whole scan runs on `skrib_format`'s parser stack, after every part that
    /// will be parsed has been checked against `MAX_XML_DEPTH` (see
    /// `refuse_deep_parts`). `docx-rs` descends once for every table nested in a
    /// table cell, about 50 KB of stack each in a debug build, and cannot be
    /// stopped once it has started; the check is what bounds it, and the stack is
    /// what gives the bound room.
    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        skrib_format::xml_depth::on_parser_stack(|| scan_document(bytes, display_name, origin))
            .map_err(anyhow::Error::new)?
    }
}

/// [`DocxScanner::scan`], on the parser stack.
fn scan_document(bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
    refuse_deep_parts(bytes)?;
    let docx =
        docx_rs::read_docx(bytes).map_err(|e| anyhow!("not a readable Word document: {e:?}"))?;

    // No document title is read. `docx_rs` keeps `CoreProps` private with no
    // accessor, and opening the container a second time to reach
    // `docProps/core.xml` would be a lot of machinery for a field Word leaves
    // empty in almost every manuscript. `SourceDocument::effective_title` falls
    // back to the first heading, which is the better answer for a book anyway.
    // (The ODT scanner *can* read `dc:title`, so the two differ here.)
    let mut doc = SourceDocument::new(display_name, origin);

    let styles = StyleTable::new(&docx);
    let raw = RawScan::read(bytes).unwrap_or_default();
    let comments = CommentTable::new(&docx, &styles, &raw.comment_attrs);
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

// ---------------------------------------------------------------------------
// Depth
// ---------------------------------------------------------------------------

/// The package-level relationship naming the main document part, and the one
/// naming the custom properties: the two `_rels/.rels` targets `docx-rs` follows.
const OFFICE_DOCUMENT: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const CUSTOM_PROPERTIES: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties";

/// The document-level relationships whose targets `docx-rs` parses as XML: every
/// type `read_docx` follows out of the document's own `.rels` except `image` and
/// `hyperlink`, spelled and matched exactly as its `reader/namespace.rs` spells
/// them. `docx-rs` is pinned to one release (`=0.4.22`, see Cargo.toml), and this
/// list is part of what moving off it has to re-read.
const XML_RELATIONSHIPS: [&str; 9] = [
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
];

/// Refuse the file if any part this scan will parse as XML nests past
/// `MAX_XML_DEPTH`.
///
/// **Which parts is the whole question**, because a `.docx` also carries images and
/// embedded objects, and a binary checked as if it were XML reads as a random
/// depth that a large enough picture always exceeds. So the set is exactly what
/// the two readers will parse, found the way they find it: the fixed names, the
/// main part wherever `_rels/.rels` points, and every target of an XML-bearing
/// type in that part's own relationships, which `docx-rs` follows whatever the
/// target is called. The relationship files themselves are read with `docx-rs`'s
/// own readers (loops, not recursions, so safe to run on anything), so a
/// relationship this check cannot see is one `docx-rs` cannot follow either.
///
/// **Each part is measured once for each parser that reads it**, because the two
/// do not agree on where a document's markup is. The raw pass parses with
/// `roxmltree`, which `skrib_format::xml_depth::check` follows token for token.
/// `docx-rs` parses with `quick-xml`, which [`check_as_docx_rs_reads`] runs
/// itself: it ends a DOCTYPE, an end tag or a processing instruction at
/// different places from `roxmltree`, takes a lowercase `<!doctype`, parses a
/// part with a UTF-16 byte-order mark as the bytes it is, and reads on past an
/// ill-formed end tag where `roxmltree` stops. Measured only `roxmltree`'s way, a
/// short preamble was enough to hide any nesting behind it from the check and
/// hand the whole of it to `docx-rs`.
fn refuse_deep_parts(bytes: &[u8]) -> Result<()> {
    let Ok(mut archive) = zip::ZipArchive::new(std::io::Cursor::new(bytes)) else {
        // Not a zip at all: `read_docx` says so, and parses nothing.
        return Ok(());
    };
    for part in xml_parts(&mut archive) {
        let Ok(mut member) = archive.by_name(&part) else {
            continue;
        };
        let mut data = Vec::new();
        if std::io::Read::read_to_end(&mut member, &mut data).is_err() {
            continue;
        }
        skrib_format::xml_depth::check(&part, &data)?;
        check_as_docx_rs_reads(&part, &data)?;
    }
    Ok(())
}

/// Refuse `xml` if `docx-rs` would read it nested past `MAX_XML_DEPTH`.
///
/// Measured with the parser `docx-rs` reads with, configured as its
/// `EventReader::new` configures it and fed through the same `BufReader`, so this
/// sees exactly the events its readers see. Those readers are entered on a start
/// tag and leave by its end tag at the latest, or on the first error, so at any
/// point they are never deeper than the start tags this counts minus the end tags
/// it counts. An
/// error is read past, not stopped at. After a syntax error `quick-xml` reports
/// the end of the input and nothing follows; after an ill-formed one, an end tag
/// that does not match, it goes on, and several of `docx-rs`'s readers catch a
/// child's error (`if let Ok(table) = Table::read(..)`) and go on reading the
/// same stream, so the elements after it are parsed and have to be counted.
///
/// One kind of element is left out of the count: one whose local name holds a NUL
/// byte. `docx-rs` recognises an element by comparing its local name with names
/// that hold none, so such an element can never enter one of its readers. It is
/// what every tag of a part genuinely written in UTF-16 looks like to a parser
/// reading it byte by byte, which `quick-xml` does, and counting them would refuse
/// such a file as nested past the ceiling when `docx-rs` reads nothing from it.
fn check_as_docx_rs_reads(part: &str, xml: &[u8]) -> Result<(), skrib_format::XmlTooDeep> {
    use quick_xml::events::Event;

    let mut reader = quick_xml::Reader::from_reader(std::io::BufReader::new(xml));
    let config = reader.config_mut();
    config.trim_text(false);
    config.check_end_names = true;
    config.expand_empty_elements = false;

    let refuse = |depth: usize, read_to: u64| {
        let read_to = usize::try_from(read_to).unwrap_or(xml.len()).min(xml.len());
        skrib_format::XmlTooDeep {
            part: part.to_string(),
            depth,
            line: 1 + xml[..read_to].iter().filter(|&&b| b == b'\n').count(),
        }
    };
    let mut buf = Vec::new();
    let mut depth = 0usize;
    let mut errors_in_a_row = 0usize;
    loop {
        buf.clear();
        let event = reader.read_event_into(&mut buf);
        if event.is_ok() {
            errors_in_a_row = 0;
        }
        match event {
            Ok(Event::Start(tag)) if names_an_element(tag.name().as_ref()) => {
                depth += 1;
                if depth > skrib_format::MAX_XML_DEPTH {
                    return Err(refuse(depth, reader.buffer_position()));
                }
            }
            Ok(Event::Empty(tag)) if names_an_element(tag.name().as_ref()) => {
                if depth + 1 > skrib_format::MAX_XML_DEPTH {
                    return Err(refuse(depth + 1, reader.buffer_position()));
                }
            }
            Ok(Event::End(tag)) if names_an_element(tag.name().as_ref()) => {
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Eof) => return Ok(()),
            Ok(_) => {}
            Err(_) => {
                // An ill-formed error consumes what it rejected, a syntax error
                // ends the stream, and reading from memory has no I/O error, so
                // the stream reaches its end long before this. The count only
                // guarantees that the loop does.
                errors_in_a_row += 1;
                if errors_in_a_row > xml.len() {
                    return Ok(());
                }
            }
        }
    }
}

/// Whether `docx-rs` could recognise an element by this qualified name: whether
/// its local part, everything after the first `:` as `docx-rs` splits it, is free
/// of NUL bytes. See [`check_as_docx_rs_reads`].
fn names_an_element(name: &[u8]) -> bool {
    let local = name
        .iter()
        .position(|&b| b == b':')
        .map_or(name, |colon| &name[colon + 1..]);
    !local.contains(&0)
}

/// The names of every part [`refuse_deep_parts`] has to check, normalised the way
/// `docx-rs`'s `read_zip` normalises a name before looking it up.
fn xml_parts(archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>) -> Vec<String> {
    use docx_rs::FromXML;

    let mut parts: Vec<String> = [
        "[Content_Types].xml",
        "_rels/.rels",
        // The raw pass's three, read by these names whatever the relationships say.
        "word/document.xml",
        "word/comments.xml",
        "word/footnotes.xml",
    ]
    .map(str::to_string)
    .to_vec();

    let package = docx_rs::read_zip(archive, "_rels/.rels")
        .ok()
        .and_then(|data| docx_rs::Rels::from_xml(&data[..]).ok());
    // `read_docx`'s own fallback when the package names no main part.
    let main = package
        .as_ref()
        .and_then(|rels| rels.find_target(OFFICE_DOCUMENT))
        .map_or_else(|| "word/document.xml".to_string(), |rel| rel.2.clone());
    if let Some(custom) = package
        .as_ref()
        .and_then(|rels| rels.find_target(CUSTOM_PROPERTIES))
    {
        parts.push(custom.2.clone());
    }
    parts.push(main.clone());
    if let Ok(rels) = docx_rs::read_document_rels(archive, &main) {
        for kind in XML_RELATIONSHIPS {
            for (_, path, _) in rels.find_target_path(kind).unwrap_or_default() {
                parts.push(path.to_string_lossy().into_owned());
            }
        }
    }

    let mut parts: Vec<String> = parts
        .iter()
        .map(|name| name.replace('\\', "/").trim_start_matches('/').to_string())
        .collect();
    parts.sort_unstable();
    parts.dedup();
    parts
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

    /// What a paragraph style claims this paragraph is — an epigraph, a quotation, or
    /// nothing — following `w:basedOn` exactly as [`Self::apply_style_chain`] does.
    ///
    /// **Style ids, never style names.** This module's own heading rule says why: Word
    /// localizes a style's display name ("Quote" is "Citation" in French Word, "Zitat" in
    /// German) and does not localize its `w:styleId`. Matching the name would work on
    /// every English manuscript and quietly fail on the first one that is not.
    ///
    /// The chain matters because a returning file's paragraph may reference a style
    /// derived from ours rather than ours itself — the same reason the run-formatting
    /// walk above follows it — and because Word's own `IntenseQuote` is `basedOn`
    /// `Quote` in many templates.
    ///
    /// The vocabulary itself lives in [`rich::styled_as`], shared with the ODT scanner so
    /// the two containers cannot disagree about the same manuscript.
    fn quoted_as(&self, property: &ParagraphProperty) -> Option<rich::StyledAs> {
        let mut current = property.style.as_ref().map(|s| s.val.clone());
        let mut guard = 0;
        while let Some(id) = current {
            if let Some(styled) = rich::styled_as(&id) {
                return Some(styled);
            }
            current = self.based_on(self.by_id.get(&id)?);
            guard += 1;
            if guard > 32 {
                return None;
            }
        }
        None
    }

    /// The `w:jc` value this paragraph is aligned by: its own, else its style's, following
    /// `w:basedOn`. `default_style` is the style a paragraph naming none is in.
    fn justification(
        &self,
        property: &ParagraphProperty,
        default_style: Option<&str>,
    ) -> Option<String> {
        if let Some(jc) = &property.alignment {
            return Some(jc.val.clone());
        }
        let mut current = property
            .style
            .as_ref()
            .map(|s| s.val.clone())
            .or_else(|| default_style.map(str::to_string));
        let mut guard = 0;
        while let Some(id) = current {
            let style = self.by_id.get(&id)?;
            if let Some(jc) = &style.paragraph_property.alignment {
                return Some(jc.val.clone());
            }
            current = self.based_on(style);
            guard += 1;
            if guard > 32 {
                return None;
            }
        }
        None
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

/// A DrawingML length in pixels at 96 to the inch, the unit the editor measures a picture
/// in. An English Metric Unit is 1/914,400 of an inch, so a pixel is 9,525 of them.
fn emu_to_pixels(emu: u32) -> u32 {
    (emu + 9_525 / 2) / 9_525
}

/// A `w:jc` value as the edge the paragraph is laid against.
///
/// OOXML's `left` and `right` name the paragraph's **start** and **end**, the same as the
/// `start` and `end` its strict schema spells out: in a right-to-left paragraph, `left` is
/// the right-hand edge. That is how Word writes them and how LibreOffice reads them back,
/// so a right-to-left paragraph's edges are swapped here, where the direction is known.
/// `docx-rs` reads an unrecognised value (the kashida justifications) as `left`, which is
/// the start edge, the one alignment the stored Djot never needs to state.
fn alignment_of(value: &str, right_to_left: bool) -> Option<Alignment> {
    let (start, end) = if right_to_left {
        (Alignment::Right, Alignment::Left)
    } else {
        (Alignment::Left, Alignment::Right)
    };
    match value {
        "center" => Some(Alignment::Center),
        "left" | "start" => Some(start),
        "right" | "end" => Some(end),
        "both" | "distribute" | "justified" => Some(Alignment::Justify),
        _ => None,
    }
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
    // `w:vertAlign`. `baseline` is how a run inside a raised style says it is not raised,
    // so it clears both.
    if let Some(position) = &property.vert_align {
        style.superscript = *position == VertAlign::new(VertAlignType::SuperScript);
        style.subscript = *position == VertAlign::new(VertAlignType::SubScript);
    }
}

// ---------------------------------------------------------------------------
// Comments
// ---------------------------------------------------------------------------

struct CommentMeta {
    /// Only Skribisto's own writer puts these two on a `<w:comment>` — see
    /// `RawScan`'s own doc on why they need a raw pass at all, and
    /// [`crate::sources::rich::RichAnnotation::uid`] for what recognising one lets
    /// `apply_document_import_uc` do. `initials` is empty (never `None`) for the
    /// ordinary case of a comment with no `w:initials`, mirroring
    /// `RichAnnotation::author_initials`'s own convention.
    uid: Option<uuid::Uuid>,
    initials: String,
    author: String,
    created: Option<chrono::DateTime<chrono::Utc>>,
    /// One `Vec` of [`Run`]s per paragraph — not yet Djot. See
    /// [`crate::sources::rich::RichAnnotation::paragraphs`]: the conversion is
    /// centralised in `rich::assemble`, not duplicated here.
    paragraphs: Vec<Vec<Run>>,
    resolved: bool,
    parent: Option<usize>,
}

struct CommentTable {
    /// In document order, as `comments.xml` lists them.
    order: Vec<usize>,
    by_id: HashMap<usize, CommentMeta>,
}

impl CommentTable {
    fn new(docx: &Docx, styles: &StyleTable, comment_attrs: &HashMap<usize, CommentAttrs>) -> Self {
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
            let attrs = comment_attrs.get(&comment.id);
            by_id.insert(
                comment.id,
                meta_of(comment, &done_by_paragraph, styles, attrs),
            );
        }
        CommentTable { order, by_id }
    }

    fn get(&self, id: usize) -> Option<&CommentMeta> {
        self.by_id.get(&id)
    }
}

fn meta_of(
    comment: &Comment,
    done_by_paragraph: &HashMap<&str, bool>,
    styles: &StyleTable,
    attrs: Option<&CommentAttrs>,
) -> CommentMeta {
    let mut builder = CommentBodyBuilder::new(styles);
    let mut resolved = false;
    for child in &comment.children {
        if let CommentChild::Paragraph(paragraph) = child {
            if let Some(done) = done_by_paragraph.get(paragraph.id.as_str()) {
                resolved |= *done;
            }
            builder.paragraph(paragraph);
        }
    }
    CommentMeta {
        uid: attrs.and_then(|a| a.uid),
        initials: attrs.map(|a| a.initials.clone()).unwrap_or_default(),
        author: comment.author.clone(),
        created: parse_date(&comment.date),
        paragraphs: builder.finish(),
        resolved,
        parent: comment.parent_comment_id,
    }
}

/// Builds the styled paragraphs of one comment or reply's own text — the same
/// bold/italic/underline/strikethrough machinery [`Walker::run`] applies to
/// manuscript prose ([`StyleTable::run_style`]), narrowed to what a `w:comment`
/// can actually contain.
///
/// Deliberately narrower than [`Walker::paragraph_children`]: a comment carries
/// no tracked change of its *own* review pass (`w:ins`/`w:del`/`w:moveFrom` track
/// edits to the *manuscript*, never to a margin note — though a comment can sit
/// inside an accepted `w:ins`/`w:moveTo` span, which is why those two are still
/// read here), no nested comment (Word offers no UI to comment on a comment), no
/// field, footnote or embedded object. What is left is `w:r`, `w:hyperlink` and a tab,
/// each keeping its run's own formatting.
struct CommentBodyBuilder<'a> {
    styles: &'a StyleTable,
    paragraphs: Vec<Vec<Run>>,
    current: Vec<Run>,
}

impl<'a> CommentBodyBuilder<'a> {
    fn new(styles: &'a StyleTable) -> Self {
        CommentBodyBuilder {
            styles,
            paragraphs: Vec::new(),
            current: Vec::new(),
        }
    }

    /// Consume one `w:comment`/reply paragraph, then close it off — a `w:comment`
    /// with several `<w:p>` children is an editor who pressed Enter inside the
    /// comment box, and each becomes its own Djot paragraph.
    fn paragraph(&mut self, paragraph: &Paragraph) {
        self.children(&paragraph.children, &paragraph.property, None);
        self.paragraphs.push(std::mem::take(&mut self.current));
    }

    fn children(
        &mut self,
        children: &[ParagraphChild],
        property: &ParagraphProperty,
        link: Option<&str>,
    ) {
        for child in children {
            match child {
                ParagraphChild::Run(run) => self.run(run, property, link),
                // Accepted: this is the text as it stands, same as manuscript prose.
                ParagraphChild::Insert(insert) => {
                    for c in &insert.children {
                        if let InsertChild::Run(run) = c {
                            self.run(run, property, link);
                        }
                    }
                }
                ParagraphChild::MoveTo(move_to) => {
                    for c in &move_to.children {
                        if let MoveToChild::Run(run) = c {
                            self.run(run, property, link);
                        }
                    }
                }
                ParagraphChild::Hyperlink(hyperlink) => {
                    let url = match &hyperlink.link {
                        docx_rs::HyperlinkData::External { rid: _, path } => Some(path.clone()),
                        docx_rs::HyperlinkData::Anchor { anchor } => Some(format!("#{anchor}")),
                    };
                    self.children(&hyperlink.children, property, url.as_deref().or(link));
                }
                _ => {}
            }
        }
    }

    fn run(&mut self, run: &DocxRun, property: &ParagraphProperty, link: Option<&str>) {
        let style = self.styles.run_style(property, &run.run_property);
        for rc in &run.children {
            match rc {
                RunChild::Text(text) => self.current.push(Run {
                    text: text.text.clone(),
                    style,
                    link: link.map(str::to_string),
                    image: None,
                    footnote: None,
                }),
                RunChild::Tab(_) | RunChild::PTab(_) => self.current.push(Run {
                    text: " ".into(),
                    style,
                    link: link.map(str::to_string),
                    image: None,
                    footnote: None,
                }),
                // A line break starts a new paragraph, as `Walker::run` does for
                // manuscript prose: Djot has no line break inside a paragraph that the
                // editor keeps.
                RunChild::Break(_) | RunChild::CarriageReturn(_) => {
                    self.paragraphs.push(std::mem::take(&mut self.current));
                }
                _ => {}
            }
        }
    }

    fn finish(self) -> Vec<Vec<Run>> {
        self.paragraphs
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
/// Skribisto's own extension namespace, carrying `skrb:uid` on `<w:comment>` — the
/// exact same URI `text-document`'s `export_docx_uc` declares
/// (`export_docx_uc::SKRB_NAMESPACE_URI`) under the same `skrb` prefix. See the
/// module doc's "supplementary pass" note for why reading it needs a raw pass at
/// all: `docx_rs::Comment` has no field for it or for `w:initials`.
const NS_SKRB: &str = "urn:ferntech:text-document:comment:1";

/// What only Skribisto's own writer puts on a `<w:comment>` — read once from
/// `word/comments.xml`, keyed by `w:id` (the same plain `usize` `docx_rs::Comment::id`
/// already is, so no join table is needed to match the two up).
#[derive(Debug, Clone, Default)]
struct CommentAttrs {
    uid: Option<uuid::Uuid>,
    /// Empty (never absent) when `w:initials=""` or the attribute is missing —
    /// the same "empty means none" convention `RichAnnotation::author_initials`
    /// documents.
    initials: String,
}

/// What the typed reader does not surface — see the module note.
///
/// The paragraph-keyed fields (`references`, `rules`) are keyed by **top-level
/// paragraph ordinal**: the *n*-th `w:p` that is a direct child of `w:body` is the
/// *n*-th `DocumentChild::Paragraph`, in the same order, so the two passes agree
/// without either knowing about the other. A paragraph inside a table or an
/// `w:sdt` is deliberately not counted, on either side. `comment_attrs` needs no
/// such join: it is keyed by the comment's own `w:id`, which both this pass and
/// `docx_rs::Comment::id` read off the identical attribute.
#[derive(Default)]
struct RawScan {
    /// Paragraph ordinal → the point comments in it, as `(comment id, char offset)`.
    references: HashMap<usize, Vec<(usize, usize)>>,
    /// Paragraph ordinals that are a horizontal rule.
    rules: HashSet<usize>,
    /// `w:id` → the `skrb:uid`/`w:initials` attributes only Skribisto's own writer
    /// puts on that comment's `<w:comment>` — see [`CommentAttrs`].
    comment_attrs: HashMap<usize, CommentAttrs>,
    /// Paragraph ordinal → the footnote references in it, as `(w:id, char offset)`.
    ///
    /// Found here, in the raw pass, because **`docx-rs`'s reader never produces
    /// one**: nothing in its `src/reader/` constructs `RunChild::FootnoteReference`,
    /// so the typed walk's arm for it is unreachable and a scanner that trusted it
    /// would see a document with no notes in it. That is what made this the one
    /// silent loss in the whole import — a `.docx` coming back from an editor lost
    /// its footnotes and said nothing.
    ///
    /// The same shape and the same coordinate space as [`Self::references`], so
    /// a reference lands where the typed walk's text says it does.
    footnotes: HashMap<usize, Vec<(usize, usize)>>,
    /// `w:id` → that footnote's own styled paragraphs, from `word/footnotes.xml`.
    footnote_bodies: HashMap<usize, Vec<Vec<Run>>>,
    /// Paragraph ordinal → the direction and page break its own `w:pPr` states.
    ///
    /// Read here because `docx-rs` never reads `w:bidi` at all, and reads
    /// `w:pageBreakBefore` only when it is on, so a paragraph turning off the break its
    /// style asks for would be indistinguishable from one saying nothing.
    paragraph_props: HashMap<usize, RawParagraphProps>,
    /// The same two properties for each paragraph style of `word/styles.xml`, and the
    /// document's defaults, for a paragraph that states neither itself.
    styles: RawStyles,
}

/// What a `w:pPr` states about a paragraph's direction and page break. `None` is silence,
/// which defers to the style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct RawParagraphProps {
    bidi: Option<bool>,
    page_break_before: Option<bool>,
}

impl RawParagraphProps {
    fn read(paragraph_or_style: roxmltree::Node<'_, '_>) -> Self {
        let Some(ppr) = paragraph_or_style
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "pPr")
        else {
            return Self::default();
        };
        RawParagraphProps {
            bidi: raw_toggle_value(ppr, "bidi"),
            page_break_before: raw_toggle_value(ppr, "pageBreakBefore"),
        }
    }
}

/// The paragraph styles of `word/styles.xml`, reduced to what [`RawParagraphProps`] holds.
#[derive(Debug, Clone, Default)]
struct RawStyles {
    /// Style id → its own properties and the id it is based on.
    by_id: HashMap<String, (RawParagraphProps, Option<String>)>,
    /// The style a paragraph naming none is in: the one marked `w:default`.
    default_style: Option<String>,
    /// `w:docDefaults`, under every style.
    defaults: RawParagraphProps,
}

impl RawStyles {
    fn read<R: std::io::Read + std::io::Seek>(zip: &mut zip::ZipArchive<R>) -> Option<Self> {
        let xml = {
            let mut file = zip.by_name("word/styles.xml").ok()?;
            let mut buffer = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut buffer).ok()?;
            String::from_utf8_lossy(&buffer).into_owned()
        };
        let document = roxmltree::Document::parse(&xml).ok()?;
        let mut styles = RawStyles::default();
        for node in document
            .root_element()
            .children()
            .filter(|n| n.is_element())
        {
            match node.tag_name().name() {
                "docDefaults" => {
                    if let Some(default) = node
                        .children()
                        .find(|n| n.is_element() && n.tag_name().name() == "pPrDefault")
                    {
                        styles.defaults = RawParagraphProps::read(default);
                    }
                }
                "style" if node.attribute((NS_W, "type")) == Some("paragraph") => {
                    let Some(id) = node.attribute((NS_W, "styleId")) else {
                        continue;
                    };
                    if matches!(node.attribute((NS_W, "default")), Some("1" | "true" | "on")) {
                        styles.default_style = Some(id.to_string());
                    }
                    let based_on = node
                        .children()
                        .find(|n| n.is_element() && n.tag_name().name() == "basedOn")
                        .and_then(|n| n.attribute((NS_W, "val")))
                        .map(str::to_string);
                    styles
                        .by_id
                        .insert(id.to_string(), (RawParagraphProps::read(node), based_on));
                }
                _ => {}
            }
        }
        Some(styles)
    }

    /// The first value `pick` finds in the style `id` names or the ones it is based on, and
    /// then in the document's defaults.
    fn resolve(
        &self,
        id: Option<&str>,
        pick: impl Fn(&RawParagraphProps) -> Option<bool>,
    ) -> Option<bool> {
        let mut current = id
            .map(str::to_string)
            .or_else(|| self.default_style.clone());
        let mut guard = 0;
        while let Some(style) = current {
            let Some((props, based_on)) = self.by_id.get(&style) else {
                break;
            };
            if let Some(value) = pick(props) {
                return Some(value);
            }
            current = based_on.clone();
            guard += 1;
            if guard > 32 {
                break;
            }
        }
        pick(&self.defaults)
    }
}

impl RawScan {
    /// Returns `None` when `word/document.xml` cannot be read or parsed. That is
    /// not a failure worth stopping an import for: without it the scanner behaves
    /// as it would have without this pass at all. `word/comments.xml` is read
    /// best-effort within the same zip open — a document with no comments has no
    /// such member, and that is not an error either, just an empty `comment_attrs`.
    fn read(bytes: &[u8]) -> Option<RawScan> {
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).ok()?;
        let xml = {
            let mut file = zip.by_name("word/document.xml").ok()?;
            let mut buffer = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut buffer).ok()?;
            String::from_utf8_lossy(&buffer).into_owned()
        };
        let document = parse_part("word/document.xml", &xml).ok()?;
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
            let mut footnotes: Vec<(usize, usize)> = Vec::new();
            walk_raw_paragraph(
                paragraph,
                &mut offset,
                &mut found,
                &ranged,
                &mut footnotes,
                0,
            );
            if !footnotes.is_empty() {
                scan.footnotes.insert(ordinal, footnotes);
            }
            if !found.is_empty() {
                scan.references.insert(ordinal, found);
            }
            let props = RawParagraphProps::read(paragraph);
            if props != RawParagraphProps::default() {
                scan.paragraph_props.insert(ordinal, props);
            }
        }

        scan.comment_attrs = read_comment_attrs(&mut zip).unwrap_or_default();
        scan.footnote_bodies = read_footnote_bodies(&mut zip).unwrap_or_default();
        scan.styles = RawStyles::read(&mut zip).unwrap_or_default();
        Some(scan)
    }
}

/// Read `word/comments.xml` for the two attributes only Skribisto's own writer sets
/// — see [`CommentAttrs`]. `None` when the member is absent (no comments at all) or
/// not well-formed; either way the caller falls back to an empty map, which is
/// exactly what a plain Word/LibreOffice `.docx` already looks like from here.
fn read_comment_attrs<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
) -> Option<HashMap<usize, CommentAttrs>> {
    let xml = {
        let mut file = zip.by_name("word/comments.xml").ok()?;
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut buffer).ok()?;
        String::from_utf8_lossy(&buffer).into_owned()
    };
    let document = parse_part("word/comments.xml", &xml).ok()?;
    let mut out = HashMap::new();
    for node in document
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "comment")
    {
        let Some(id) = node
            .attribute((NS_W, "id"))
            .and_then(|v| v.parse::<usize>().ok())
        else {
            continue;
        };
        let uid = node
            .attribute((NS_SKRB, "uid"))
            .and_then(|v| uuid::Uuid::parse_str(v).ok());
        let initials = node
            .attribute((NS_W, "initials"))
            .unwrap_or_default()
            .to_string();
        out.insert(id, CommentAttrs { uid, initials });
    }
    Some(out)
}

/// Read `word/footnotes.xml` for each footnote's own text.
///
/// Mirrors [`read_comment_attrs`], and for the same reason: `docx-rs`'s reader
/// does not surface footnotes at all — not the part, not the `w:footnoteReference`
/// that names one — so the only way to either is the raw member.
///
/// Word puts two synthetic notes at the top of every file, the separator rule and
/// its continuation; both are chrome rather than content and are skipped by their
/// `w:type`. What comes back is styled paragraphs on exactly the terms
/// [`CommentBodyBuilder`] produces for a comment, so a note goes through
/// `rich::assemble`'s single Djot conversion rather than acquiring a second one
/// here — a footnote is prose the writer wrote, and the four marks
/// [`rich::RunStyle`] carries are the four they could have applied to it.
fn read_footnote_bodies<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
) -> Option<HashMap<usize, Vec<Vec<Run>>>> {
    let xml = {
        let mut file = zip.by_name("word/footnotes.xml").ok()?;
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut buffer).ok()?;
        String::from_utf8_lossy(&buffer).into_owned()
    };
    let document = parse_part("word/footnotes.xml", &xml).ok()?;
    let mut out = HashMap::new();
    for node in document
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "footnote")
    {
        // `separator` / `continuationSeparator`: the horizontal rules Word draws
        // above a page's notes, present in every document and never authored.
        if node.attribute((NS_W, "type")).is_some() {
            continue;
        }
        let Some(id) = node
            .attribute((NS_W, "id"))
            .and_then(|v| v.parse::<usize>().ok())
        else {
            continue;
        };
        let body = raw_note_paragraphs(node);
        if !body.is_empty() {
            out.insert(id, body);
        }
    }
    Some(out)
}

/// The styled paragraphs of one `<w:footnote>`, from the raw tree.
///
/// The note opens with a run holding `<w:footnoteRef/>` — Word's own printed
/// number — usually followed by a tab or a space. That run contributes no text, so
/// it disappears on its own; the separator it left behind would arrive as a
/// leading indent on the note's first line, so the first paragraph is trimmed at
/// the front. Nothing else is trimmed: a writer's own spacing further in is theirs.
fn raw_note_paragraphs(note: roxmltree::Node<'_, '_>) -> Vec<Vec<Run>> {
    let mut paragraphs: Vec<Vec<Run>> = Vec::new();
    for p in note
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "p")
    {
        let mut runs: Vec<Run> = Vec::new();
        raw_note_runs(p, &mut runs, 0);
        paragraphs.push(runs);
    }
    if let Some(first) = paragraphs.iter_mut().find(|p| !p.is_empty())
        && let Some(run) = first.first_mut()
    {
        run.text = run.text.trim_start().to_string();
    }
    while paragraphs
        .last()
        .is_some_and(|p| p.iter().all(|r| r.text.trim().is_empty()))
    {
        paragraphs.pop();
    }
    paragraphs
}

/// Collect one raw paragraph's text runs, carrying the four marks that survive.
///
/// Recursion is bounded by [`MAX_RUN_NESTING`] on the same terms and for the same
/// reason as [`walk_raw_paragraph`]: a hostile file's nesting is not a manuscript's,
/// and following it aborts the process rather than unwinding.
fn raw_note_runs(node: roxmltree::Node<'_, '_>, out: &mut Vec<Run>, depth: u32) {
    if depth >= MAX_RUN_NESTING {
        return;
    }
    for child in node.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "r" => {
                let style = raw_run_style(child);
                for grandchild in child.children().filter(roxmltree::Node::is_element) {
                    match grandchild.tag_name().name() {
                        "t" => out.push(Run::styled(text_of(grandchild), style)),
                        "tab" | "ptab" => out.push(Run::styled(" ", style)),
                        // A tracked deletion inside a note is not the note's text,
                        // on the same terms as in the manuscript.
                        _ => {}
                    }
                }
            }
            // A nested note is not a thing Word can author, but a paragraph inside
            // a table inside a note is — so the walk recurses rather than assuming
            // runs are always direct children.
            "p" => {}
            _ => raw_note_runs(child, out, depth + 1),
        }
    }
}

/// The marks [`rich::RunStyle`] carries, read off a raw `<w:rPr>`.
///
/// `code` is deliberately never set: OOXML expresses monospace as a *font*, and a
/// font is not a mark this model carries — reading one as `code` would turn a note
/// somebody typed in Courier into a literal code span.
fn raw_run_style(run: roxmltree::Node<'_, '_>) -> RunStyle {
    let Some(rpr) = run
        .children()
        .filter(roxmltree::Node::is_element)
        .find(|n| n.tag_name().name() == "rPr")
    else {
        return RunStyle::default();
    };
    RunStyle {
        bold: raw_toggle(rpr, "b"),
        italic: raw_toggle(rpr, "i"),
        // `w:u` is not a toggle: it names the *kind* of line, and `none` is how
        // OOXML spells "no underline". Absent and `val="none"` mean the same thing.
        underline: rpr
            .children()
            .filter(roxmltree::Node::is_element)
            .find(|n| n.tag_name().name() == "u")
            .is_some_and(|n| n.attribute((NS_W, "val")) != Some("none")),
        strikethrough: raw_toggle(rpr, "strike") || raw_toggle(rpr, "dstrike"),
        code: false,
        superscript: raw_vertical_position(rpr) == Some("superscript"),
        subscript: raw_vertical_position(rpr) == Some("subscript"),
    }
}

/// The `w:vertAlign` value of a raw `<w:rPr>`, if it has one.
fn raw_vertical_position<'a>(rpr: roxmltree::Node<'a, '_>) -> Option<&'a str> {
    rpr.children()
        .filter(roxmltree::Node::is_element)
        .find(|n| n.tag_name().name() == "vertAlign")
        .and_then(|n| n.attribute((NS_W, "val")))
}

/// What an OOXML toggle property under `parent` says: `Some(true)` when present and on,
/// `Some(false)` when present and off, `None` when absent. See [`raw_toggle`] for the
/// spellings of off.
fn raw_toggle_value(parent: roxmltree::Node<'_, '_>, name: &str) -> Option<bool> {
    parent
        .children()
        .filter(roxmltree::Node::is_element)
        .find(|n| n.tag_name().name() == name)
        .map(|n| !matches!(n.attribute((NS_W, "val")), Some("0" | "false" | "off")))
}

/// One OOXML toggle property: present means on, unless it says otherwise.
///
/// `<w:b/>` and `<w:b w:val="1"/>` are both on; `0`, `false` and `off` are the
/// three spellings of off that ECMA-376 allows for a toggle. Getting this wrong is
/// silent — a note explicitly un-bolded inside a bold paragraph would arrive bold.
fn raw_toggle(rpr: roxmltree::Node<'_, '_>, name: &str) -> bool {
    rpr.children()
        .filter(roxmltree::Node::is_element)
        .find(|n| n.tag_name().name() == name)
        .is_some_and(|n| !matches!(n.attribute((NS_W, "val")), Some("0" | "false" | "off")))
}

/// What a scanner's placeholder footnote label starts with.
///
/// Shared with the ODT scanner so both mint the same shape, and so
/// `apply_document_import` has one prefix to recognise. The label itself is
/// never shown to a writer and never stored — see
/// [`crate::block::SourceFootnote::label`].
pub const FOOTNOTE_LABEL_PREFIX: &str = "srcfn-";

/// Insert a footnote-reference run `within` characters into `runs`, splitting the
/// run that straddles that point.
///
/// The offset is in characters of the runs' plain text, so an image (one
/// `IMAGE_PLACEHOLDER`) counts as one and a run already carrying a footnote counts
/// as none — which is what makes two notes in one sentence land in the right order
/// rather than both at the same seam.
fn insert_footnote_run(runs: &mut Vec<Run>, within: usize, label: &str) {
    let reference = Run {
        footnote: Some(label.to_string()),
        ..Default::default()
    };
    let mut seen = 0usize;
    for index in 0..runs.len() {
        let len = if runs[index].image.is_some() || runs[index].footnote.is_some() {
            1
        } else {
            runs[index].text.chars().count()
        };
        if within < seen + len {
            let split = within - seen;
            if split == 0 {
                runs.insert(index, reference);
            } else if runs[index].image.is_some() {
                // An image is one indivisible character: a note anchored "inside"
                // it goes after it, never between its alt text's letters.
                runs.insert(index + 1, reference);
            } else {
                let byte = runs[index]
                    .text
                    .char_indices()
                    .nth(split)
                    .map_or(runs[index].text.len(), |(b, _)| b);
                let tail = runs[index].text.split_off(byte);
                let mut second = runs[index].clone();
                second.text = tail;
                runs.insert(index + 1, reference);
                runs.insert(index + 2, second);
            }
            return;
        }
        seen += len;
    }
    runs.push(reference);
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

/// How deep this walk will follow a paragraph's element tree.
///
/// The parse itself is bounded: every part the raw pass reads goes through
/// [`parse_part`], which refuses a part nested past
/// `skrib_format::MAX_XML_DEPTH` before `roxmltree` recurses into it. This walk
/// recurses once per level as well, and stops far below that ceiling on its own
/// terms. Word nests a handful deep (a hyperlink inside a smart tag inside a
/// content control), so a paragraph claiming more than this is not a manuscript.
/// Stopping is the same trade `skrib_format::djot_depth` makes on the prose side:
/// content below the cap is not read.
const MAX_RUN_NESTING: u32 = 64;

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
    footnotes: &mut Vec<(usize, usize)>,
    depth: u32,
) {
    if depth >= MAX_RUN_NESTING {
        return;
    }
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
            // **Does not advance the offset.** Word draws a superscript number
            // here, but these offsets index the text the *typed* walk produces,
            // and that walk contributes no character for a reference — so
            // counting one would put every comment after a footnote one
            // character late. (An earlier revision of this arm did advance, on
            // the reasoning that the reference occupies a rendered position.
            // It does; it just is not in this coordinate space.)
            "footnoteReference" | "endnoteReference" => {
                let id = child
                    .attribute((NS_W, "id"))
                    .and_then(|v| v.parse::<usize>().ok());
                if let Some(id) = id {
                    footnotes.push((id, *offset));
                }
            }
            _ => walk_raw_paragraph(child, offset, found, ranged, footnotes, depth + 1),
        }
    }
}

/// Parse one part of the raw pass through `skrib_format::xml_depth`, so it is
/// bounded like everything else. [`refuse_deep_parts`] has already checked these
/// three by name, so a refusal cannot reach here from [`DocxScanner::scan`].
fn parse_part<'a>(part: &str, xml: &'a str) -> Result<roxmltree::Document<'a>> {
    skrib_format::xml_depth::parse(part, xml, skrib_format::xml_depth::Dtd::Refuse)
        .map_err(anyhow::Error::new)
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
    row_marks: Vec<RichRowMark>,
    /// Comment marks closed so far, matched to their annotations in `finish`.
    comment_marks: Vec<CommentMark>,
    /// Bookmark **id** → the comment mark it opened. Keyed by id and not by name because
    /// OOXML's `w:bookmarkEnd` carries the id alone — see `close_mark`.
    open_marks: HashMap<usize, OpenMark>,
    seen: HashSet<usize>,
    diagnostics: Vec<ImportDiagnostic>,
    tracked_changes: usize,
    /// Who made them, first-seen order. A `.docx` an editor and a proofreader have
    /// both been through names two people, and the accepted text names neither.
    tracked_authors: Vec<String>,
    text_boxes: usize,
    embedded_objects: usize,
    fields: usize,
    /// References the walk could not put back into any block — see
    /// [`Walker::splice_footnotes`]. Not a count of the document's footnotes: the
    /// ones that *were* placed are in [`Self::footnotes`] and are carried.
    footnotes_dropped: usize,
    /// One entry per note whose reference reached the prose, in the order the
    /// references were met. Deduplicated by label: a note cited twice is one note.
    footnotes: Vec<rich::RichFootnote>,
    unknown_styles: HashSet<String>,
    /// A page break was met and nothing has started on the new page yet. The next block
    /// the walk produces starts the page: a paragraph holding only Word's `Ctrl+Enter`
    /// break produces none itself, and the break belongs to what follows it.
    page_break_pending: bool,
}

struct ParaBuild {
    kind: ParagraphKind,
    runs: Vec<Run>,
    len: usize,
    props: BlockProps,
    /// Whether this is a table cell's text, where a line break is a space: a cell is one
    /// block of its table, never a paragraph of its own.
    in_cell: bool,
}

impl ParaBuild {
    /// The rest of the same paragraph after a line break: same kind, same alignment and
    /// direction, and not the start of a page.
    fn continuation(&self) -> ParaBuild {
        ParaBuild {
            kind: self.kind,
            runs: Vec::new(),
            len: 0,
            props: BlockProps {
                page_break_before: false,
                ..self.props
            },
            in_cell: self.in_cell,
        }
    }
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
            row_marks: Vec::new(),
            comment_marks: Vec::new(),
            open_marks: HashMap::new(),
            seen: HashSet::new(),
            diagnostics: Vec::new(),
            tracked_changes: 0,
            tracked_authors: Vec::new(),
            text_boxes: 0,
            embedded_objects: 0,
            fields: 0,
            footnotes_dropped: 0,
            footnotes: Vec::new(),
            unknown_styles: HashSet::new(),
            page_break_pending: false,
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
                            self.paragraph(p, None);
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
                // Between paragraphs rather than inside one — it belongs to whatever comes
                // next, the same rule `CommentStart` above follows.
                DocumentChild::BookmarkStart(start) => self.open_mark(start.id, &start.name, 0),
                DocumentChild::BookmarkEnd(end) => self.close_mark(end.id, None),
                DocumentChild::TableOfContents(_) | DocumentChild::Section(_) => {}
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
                    self.annotations[index].replies.push(RichReply {
                        uid: meta.uid,
                        author: meta.author.clone(),
                        author_initials: meta.initials.clone(),
                        created: meta.created,
                        paragraphs: meta.paragraphs.clone(),
                    });
                }
                None => {
                    // No range and no thread. `assemble` puts an annotation whose block
                    // does not exist on the last block it placed, flagged as moved, and
                    // the planner reports it, or reports it as not imported when the
                    // file stores no prose at all.
                    self.annotations.push(RichAnnotation {
                        block_index: usize::MAX,
                        start: 0,
                        length: 0,
                        end: None,
                        uid: meta.uid,
                        // A comment with no range has nothing for a mark to bracket, so no
                        // mark can name it.
                        uid_tag: None,
                        author: meta.author.clone(),
                        author_initials: meta.initials.clone(),
                        created: meta.created,
                        paragraphs: meta.paragraphs.clone(),
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

        attach_comment_marks(&mut self.annotations, &self.comment_marks);

        RichDocument {
            blocks: std::mem::take(&mut self.blocks),
            annotations: std::mem::take(&mut self.annotations),
            row_marks: std::mem::take(&mut self.row_marks),
            footnotes: std::mem::take(&mut self.footnotes),
        }
    }

    /// A paragraph that is a direct child of `w:body`, and so has an ordinal the
    /// supplementary pass can key on.
    fn top_level_paragraph(&mut self, paragraph: &Paragraph, ordinal: usize) {
        if self.raw.rules.contains(&ordinal) {
            // A horizontal rule. Emitted as its glyph so `skribisto_model` stays the
            // one authority on what a break is — the same treatment the ODT scanner
            // gives ODF's spelling of the same construct. A scene break carries no
            // paragraph formatting, so a pending page break ends here.
            self.page_break_pending = false;
            self.blocks.push(RichBlock::body(vec![Run::plain(
                skribisto_model::scene_break::CANONICAL_MINOR,
            )]));
            return;
        }
        let first_block = self.blocks.len();
        self.paragraph(paragraph, Some(ordinal));
        let produced = self.blocks.len() > first_block;

        for (id, offset) in self.raw.references.get(&ordinal).into_iter().flatten() {
            self.point_comment(
                *id,
                if produced { first_block } else { usize::MAX },
                *offset,
            );
        }

        // After the comments, and safely so: a footnote run carries no text, so
        // splicing one shifts none of the offsets just consumed. Doing it here
        // rather than inside the build is what lets the reference find its place
        // across a `<w:br>`, which restarts `ParaBuild::len` while the raw walk's
        // offsets keep counting through the whole paragraph.
        if let Some(refs) = self.raw.footnotes.get(&ordinal) {
            let refs = refs.clone();
            self.splice_footnotes(first_block, &refs);
        }
    }

    /// Put this paragraph's footnote references back into the runs they sat between.
    ///
    /// `refs` are `(w:id, offset)` in the paragraph's own plain-text space, which is
    /// the concatenation of the blocks it produced — a `<w:br>` contributes a
    /// character to neither side, so walking the produced blocks in order
    /// reconstructs exactly the space the raw pass counted in.
    ///
    /// Splicing back-to-front matters: an earlier insertion would shift the runs a
    /// later offset is measured against, and two notes in one paragraph is ordinary
    /// (a sentence citing two sources). Iterating in reverse means every offset is
    /// still measured against the runs it was measured against when it was recorded.
    fn splice_footnotes(&mut self, first_block: usize, refs: &[(usize, usize)]) {
        let last_block = self.blocks.len();
        for (id, offset) in refs.iter().rev() {
            let Some(paragraphs) = self.raw.footnote_bodies.get(id) else {
                // A reference naming a note `word/footnotes.xml` does not define.
                // Carrying the marker with nothing behind it would put a citation
                // in the book pointing at an empty note.
                self.footnotes_dropped += 1;
                continue;
            };
            let label = format!("{FOOTNOTE_LABEL_PREFIX}{id}");
            if !self.place_footnote(first_block, last_block, *offset, &label) {
                self.footnotes_dropped += 1;
                continue;
            }
            if !self.footnotes.iter().any(|f| f.label == label) {
                self.footnotes.push(rich::RichFootnote {
                    label,
                    paragraphs: paragraphs.clone(),
                });
            }
        }
    }

    /// Insert one footnote run at `offset` within `first_block..last_block`.
    ///
    /// Returns whether it landed anywhere. An offset past the end of everything the
    /// paragraph produced is clamped to the end of its last block rather than
    /// refused: the two walks agree on ordinary prose, and where they cannot (a
    /// field's result text, a construct the typed walk drops), a note at the end of
    /// the paragraph it belongs to is much closer to right than no note at all.
    fn place_footnote(
        &mut self,
        first_block: usize,
        last_block: usize,
        offset: usize,
        label: &str,
    ) -> bool {
        let mut remaining = offset;
        let mut target: Option<(usize, usize)> = None;
        for index in first_block..last_block {
            let len = self.blocks[index].plain_text().chars().count();
            if remaining <= len {
                target = Some((index, remaining));
                break;
            }
            remaining -= len;
            target = Some((index, len));
        }
        let Some((index, within)) = target else {
            return false;
        };
        let RichBlock::Paragraph { runs, .. } = &mut self.blocks[index] else {
            // A table: its cells are their own coordinate space and a paragraph
            // ordinal does not address them.
            return false;
        };
        insert_footnote_run(runs, within, label);
        self.shift_offsets(index, within);
        true
    }

    /// Move every offset in `block` at or after `at` along by the one character a
    /// footnote reference now occupies there.
    ///
    /// The reference is spliced in **after** the comments of the same paragraph have
    /// been placed, so every offset recorded up to that point was measured in a text
    /// that did not contain it yet — see [`rich::Run::plain_push`] for why it counts
    /// as a character at all. Rebasing here rather than adjusting each offset at the
    /// point it is recorded keeps one rule in one place: the typed walk's ranged
    /// comments, the raw pass's point comments and the round-trip bookmarks are three
    /// separate paths to an offset, and every one of them would otherwise need the
    /// same correction applied consistently.
    ///
    /// A range **containing** the insertion point grows by one instead of moving: the
    /// note was cited inside the passage the editor commented on, so the passage is
    /// now one character longer.
    fn shift_offsets(&mut self, block: usize, at: usize) {
        for annotation in &mut self.annotations {
            if let Some(end) = annotation.end.as_mut()
                && end.block_index == block
                && end.offset >= at
            {
                end.offset += 1;
            }
            if annotation.block_index != block {
                continue;
            }
            if annotation.start >= at {
                annotation.start += 1;
            } else if annotation.start + annotation.length > at {
                annotation.length += 1;
            }
        }
        for open in self.open.values_mut() {
            if open.block == block && open.start >= at {
                open.start += 1;
            }
        }
        for mark in self.open_marks.values_mut() {
            if mark.block == block && mark.start >= at {
                mark.start += 1;
            }
        }
        for mark in &mut self.comment_marks {
            if mark.block != block {
                continue;
            }
            if mark.start >= at {
                mark.start += 1;
            } else if mark.start + mark.length > at {
                mark.length += 1;
            }
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

    /// Count one tracked change and remember who made it.
    ///
    /// `docx-rs` defaults an absent `w:author` to the literal `"unnamed"`
    /// (`Insert::default`), which is not a person and must not be shown as one —
    /// nor is it distinguishable here from an editor who is genuinely called that,
    /// so it is treated as absent. An anonymised file therefore reports its count
    /// with no names, which is the honest answer.
    fn tracked_change_by(&mut self, author: &str) {
        self.tracked_changes += 1;
        let author = author.trim();
        if author.is_empty() || author == "unnamed" {
            return;
        }
        if !self.tracked_authors.iter().any(|a| a == author) {
            self.tracked_authors.push(author.to_string());
        }
    }

    /// What the paragraph states about its alignment, direction and page break, its style
    /// chain resolved. `ordinal` is its top-level position, the key of the raw pass's own
    /// reading of its `w:pPr`; a paragraph inside a content control has none.
    fn paragraph_props(&self, property: &ParagraphProperty, ordinal: Option<usize>) -> BlockProps {
        let own = ordinal
            .and_then(|o| self.raw.paragraph_props.get(&o))
            .copied()
            .unwrap_or_default();
        let style = property.style.as_ref().map(|s| s.val.as_str());
        let styles = &self.raw.styles;
        let bidi = own.bidi.or_else(|| styles.resolve(style, |p| p.bidi));
        let page_break_before = own
            .page_break_before
            .or(property.page_break_before)
            .or_else(|| styles.resolve(style, |p| p.page_break_before))
            .unwrap_or(false);
        let right_to_left = bidi == Some(true);
        BlockProps {
            alignment: self
                .styles
                .justification(property, styles.default_style.as_deref())
                .and_then(|jc| alignment_of(&jc, right_to_left)),
            direction: bidi.map(|rtl| {
                if rtl {
                    Direction::RightToLeft
                } else {
                    Direction::LeftToRight
                }
            }),
            page_break_before,
        }
    }

    fn paragraph(&mut self, paragraph: &Paragraph, ordinal: Option<usize>) {
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
                // A list item stays a list item even inside a quotation: the
                // numbering is the stronger claim, and Djot can only carry one of
                // the two on a paragraph. An epigraph is never a list in practice.
                None => self
                    .styles
                    .quoted_as(property)
                    .map_or(ParagraphKind::Body, rich::kind_for_style),
            },
        };

        let mut build = ParaBuild {
            kind,
            runs: Vec::new(),
            len: 0,
            props: self.paragraph_props(property, ordinal),
            in_cell: false,
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
                    self.tracked_change_by(&insert.author);
                    self.insert_children(insert, property, build, link);
                }
                ParagraphChild::MoveTo(move_to) => {
                    self.tracked_change_by(&move_to.author);
                    for child in &move_to.children {
                        match child {
                            MoveToChild::Run(run) => self.run(run, property, build, link),
                            MoveToChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), build.len)
                            }
                            MoveToChild::CommentEnd(end) => {
                                self.close_comment(end, Some(build.len))
                            }
                            MoveToChild::Delete(delete) => self.tracked_change_by(&delete.author),
                        }
                    }
                }
                // Dropped: this is text somebody removed.
                ParagraphChild::Delete(delete) => {
                    self.tracked_change_by(&delete.author);
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
                ParagraphChild::MoveFrom(move_from) => self.tracked_change_by(&move_from.author),
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
                // Round-trip marks (`skrb_r…`, `skrb_c…`) carry this app's own identity
                // through an editor's save; every other bookmark in the document — a
                // cross-reference target, a table-of-contents entry — falls through to being
                // ignored, exactly as before.
                ParagraphChild::BookmarkStart(start) => {
                    self.open_mark(start.id, &start.name, build.len)
                }
                ParagraphChild::BookmarkEnd(end) => self.close_mark(end.id, Some(build.len)),
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
                        footnote: None,
                    });
                }
                RunChild::Tab(_) | RunChild::PTab(_) => {
                    build.len += 1;
                    build.runs.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                // Inside a table cell a break of any kind is a space: the cell is one
                // block of its table.
                RunChild::Break(_) | RunChild::CarriageReturn(_) if build.in_cell => {
                    build.len += 1;
                    build.runs.push(Run {
                        text: " ".into(),
                        style,
                        link: link.map(str::to_string),
                        image: None,
                        footnote: None,
                    });
                }
                // A line break ends the paragraph here and the rest starts a new one:
                // Djot has no line break inside a paragraph that the editor keeps. A page
                // break does the same, and the text after it starts the new page.
                RunChild::Break(kind) => {
                    let next = build.continuation();
                    let finished = std::mem::replace(build, next);
                    self.flush(finished);
                    if *kind == Break::new(BreakType::Page) {
                        self.page_break_pending = true;
                    }
                }
                RunChild::CarriageReturn(_) => {
                    let next = build.continuation();
                    let finished = std::mem::replace(build, next);
                    self.flush(finished);
                }
                RunChild::Drawing(drawing) => match &drawing.data {
                    Some(DrawingData::Pic(pic)) => {
                        self.diagnostics.push(ImportDiagnostic::ImageNotIngested {
                            path: self.origin.clone(),
                            target: pic.id.clone(),
                        });
                        build.len += 1;
                        build.runs.push(Run::sized_image(
                            "",
                            pic.id.clone(),
                            emu_to_pixels(pic.size.0),
                            emu_to_pixels(pic.size.1),
                        ));
                    }
                    Some(DrawingData::TextBox(_)) => self.text_boxes += 1,
                    None => self.embedded_objects += 1,
                },
                RunChild::Shape(_) => self.embedded_objects += 1,
                // Unreachable: `docx-rs`'s reader never constructs this variant
                // (nothing in its `src/reader/` produces one), which is why both the
                // reference and its body come from raw passes — see
                // `RawScan::footnotes` and `read_footnote_bodies`. Kept as an explicit
                // arm so a future version of the crate that *does* produce it does not
                // silently fall into the catch-all.
                RunChild::FootnoteReference(_) => {}
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

    /// A table, every cell read with the same walk a paragraph gets.
    ///
    /// One buffer runs through the whole table, so its offsets are the table's own plain
    /// text (cells joined by a line break, a cell's paragraphs by a space): the space a
    /// comment or a round-trip mark inside a cell is measured in, which is what lets
    /// `rich::assemble` place it on its words, and a cell keeps its formatting.
    fn table(&mut self, table: &Table) {
        let mut build = ParaBuild {
            kind: ParagraphKind::Body,
            runs: Vec::new(),
            len: 0,
            props: BlockProps::default(),
            in_cell: true,
        };
        let mut rows: Vec<Vec<Vec<Run>>> = Vec::new();
        let mut first_cell = true;
        for TableChild::TableRow(row) in &table.rows {
            let mut cells: Vec<Vec<Run>> = Vec::new();
            for TableRowChild::TableCell(cell) in &row.cells {
                if !first_cell {
                    build.len += 1;
                }
                first_cell = false;
                let start = build.runs.len();
                let mut first_paragraph = true;
                for content in &cell.children {
                    if let TableCellContent::Paragraph(paragraph) = content {
                        if !first_paragraph {
                            build.len += 1;
                            build.runs.push(Run::plain(" "));
                        }
                        first_paragraph = false;
                        self.paragraph_children(
                            &paragraph.children,
                            &paragraph.property,
                            &mut build,
                            None,
                        );
                    }
                }
                cells.push(build.runs.split_off(start));
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
        if !rows.is_empty() {
            // A table carries no paragraph formatting of its own, so a page break before
            // it is not carried either.
            self.page_break_pending = false;
            self.blocks.push(RichBlock::Table { rows });
        }
    }

    fn flush(&mut self, build: ParaBuild) {
        if build.runs.is_empty() {
            return;
        }
        let mut props = build.props;
        props.page_break_before |= std::mem::take(&mut self.page_break_pending);
        self.blocks.push(RichBlock::Paragraph {
            kind: build.kind,
            runs: build.runs,
            props,
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
            self.annotations[index].replies.push(RichReply {
                uid: meta.uid,
                author: meta.author.clone(),
                author_initials: meta.initials.clone(),
                created: meta.created,
                paragraphs: meta.paragraphs.clone(),
            });
            return;
        }

        let index = self.annotations.len();
        self.annotations.push(RichAnnotation {
            block_index: block,
            start,
            length: 0,
            end: None,
            uid: meta.uid,
            // Filled by `attach_comment_marks` after the walk — the bookmark carrying it
            // closes later, and on a file an editor has saved may even open first.
            uid_tag: None,
            author: meta.author.clone(),
            author_initials: meta.initials.clone(),
            created: meta.created,
            paragraphs: meta.paragraphs.clone(),
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
        let current = self.blocks.len();
        if let Some(offset) = at
            && open.block == current
        {
            self.annotations[open.annotation].length = offset.saturating_sub(open.start);
            return;
        }
        // The range ran on past the paragraph it started in. Its length there runs to the
        // end of that paragraph, and where it really ends is kept beside it: in the
        // paragraph being built when the end sits inside one, or at the end of the last
        // paragraph produced when it sits between two.
        let start_len = self
            .blocks
            .get(open.block)
            .map(|b| b.plain_text().chars().count())
            .unwrap_or(open.start);
        let end = match at {
            Some(offset) => Some(AnnotationEnd {
                block_index: current,
                offset,
            }),
            None => current.checked_sub(1).map(|last| AnnotationEnd {
                block_index: last,
                offset: self.blocks[last].plain_text().chars().count(),
            }),
        };
        let annotation = &mut self.annotations[open.annotation];
        annotation.length = start_len.saturating_sub(open.start);
        annotation.end = end.filter(|e| e.block_index > open.block);
    }

    fn annotation_of_matches(&self, index: usize, end: &CommentRangeEnd) -> bool {
        self.annotation_of
            .iter()
            .any(|(id, at)| *at == index && *end == CommentRangeEnd::new(*id))
    }

    /// `<w:bookmarkStart>` — a round-trip mark, or one of the many bookmarks that are not.
    ///
    /// A **row** mark is recorded straight away: OOXML has no self-closing bookmark, so this
    /// app writes a point mark as a start immediately followed by its end, and the position
    /// that matters is the start's. A **comment** mark is held open until its end tag gives it
    /// an extent.
    fn open_mark(&mut self, id: usize, name: &str, start: usize) {
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
                    id,
                    OpenMark {
                        uid_tag,
                        block: self.blocks.len(),
                        start,
                    },
                );
            }
            None => {}
        }
    }

    /// `<w:bookmarkEnd>` — closes a comment mark.
    ///
    /// **By numeric id, never by name.** OOXML spells a bookmark's name only on its start; the
    /// end carries `w:id` alone. A reader looking for the name on both halves finds a range
    /// that never closes, which is the shape of this bug most likely to be written by someone
    /// porting the ODF reader across, where both halves *are* named.
    fn close_mark(&mut self, id: usize, at: Option<usize>) {
        let Some(open) = self.open_marks.remove(&id) else {
            return;
        };
        let end = match at {
            Some(offset) if open.block == self.blocks.len() => offset,
            _ => open.start,
        };
        self.comment_marks.push(CommentMark {
            uid_tag: open.uid_tag,
            block: open.block,
            start: open.start,
            length: end.saturating_sub(open.start),
        });
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

    /// How deep `docx-rs`'s own event reader goes into `xml`: the stream its
    /// readers consume, counted as [`check_as_docx_rs_reads`] counts and read past
    /// errors as they can be.
    fn depth_docx_rs_reads(xml: &[u8]) -> usize {
        let mut reader = docx_rs::EventReader::new(xml);
        let (mut depth, mut deepest) = (0usize, 0usize);
        for _ in 0..=4 * xml.len() + 16 {
            match reader.next_event() {
                Ok(docx_rs::XmlEvent::StartElement { name, .. })
                    if !name.local_name.contains('\0') =>
                {
                    depth += 1;
                    deepest = deepest.max(depth);
                }
                Ok(docx_rs::XmlEvent::EndElement { name }) if !name.local_name.contains('\0') => {
                    depth = depth.saturating_sub(1);
                }
                Ok(docx_rs::XmlEvent::EndDocument) => break,
                _ => {}
            }
        }
        deepest
    }

    /// The depth check runs `quick-xml` itself rather than trusting a reading of
    /// how `docx-rs` configures it, so it must never be less strict than
    /// `docx-rs`'s own event reader: on every shape that has hidden nesting from a
    /// check before, it refuses what that reader would take past the ceiling, and
    /// it passes what that reader keeps under it, a part in UTF-16 included. If a
    /// later `docx-rs` reads differently, this is where it shows.
    #[test]
    fn the_depth_check_sees_what_docx_rs_reads() {
        // `<w:document>` is 1 and `<w:body>` 2, and each table takes three levels,
        // so 84 tables put the innermost cell at 254, and 85 at 257.
        let part = |preamble: &str, head: &str, tables: usize, inner: &str| {
            format!(
                "<?xml version=\"1.0\"?>{preamble}<w:document xmlns:w=\"w\"><w:body>{head}{}\
                 {inner}{}</w:body></w:document>",
                "<w:tbl><w:tr><w:tc>".repeat(tables),
                "</w:tc></w:tr></w:tbl>".repeat(tables)
            )
            .into_bytes()
        };
        let with_mark = |bytes: Vec<u8>| [vec![0xFF, 0xFE], bytes].concat();
        let in_utf16 = |text: String| {
            let mut bytes = vec![0xFF, 0xFE];
            for unit in text.encode_utf16() {
                bytes.extend_from_slice(&unit.to_le_bytes());
            }
            bytes
        };
        let shapes = [
            (
                "at the ceiling",
                part("", "", 84, "<w:p><w:r/></w:p>"),
                false,
            ),
            ("past the ceiling", part("", "", 85, ""), true),
            (
                "a doctype holding a comment",
                part("<!DOCTYPE w:document [<x <!-- >]>", "", 85, ""),
                true,
            ),
            (
                "a lowercase doctype",
                part(
                    "<!doctype w:document [<!ENTITY e \"> <!-- \">]>",
                    "",
                    85,
                    "",
                ),
                true,
            ),
            (
                "an ill-formed end tag",
                part("", "<w:tbl></w:x a='>' <!-- >", 85, ""),
                true,
            ),
            ("a byte-order mark", with_mark(part("", "", 86, "")), true),
            (
                "a part in UTF-16",
                in_utf16(String::from_utf8_lossy(&part("", "", 85, "")).into_owned()),
                false,
            ),
        ];
        for (shape, xml, past) in shapes {
            let deepest = depth_docx_rs_reads(&xml);
            let refused = check_as_docx_rs_reads("part", &xml).err();
            assert_eq!(
                deepest > skrib_format::MAX_XML_DEPTH,
                past,
                "{shape}: docx-rs reads {deepest} deep"
            );
            assert_eq!(
                refused.is_some(),
                past,
                "{shape}: the check says {refused:?}"
            );
        }
    }
}
