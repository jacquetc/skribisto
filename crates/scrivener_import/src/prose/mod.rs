// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A Scrivener project's prose: every text read, Scrivener's dialect rewritten, and mapped
//! onto the document importer's rich model, one part per row the structure plan gives it.
//!
//! Three steps, each owned elsewhere as far as it can be:
//!
//! 1. **Reading.** `rtf_reader` turns a document's RTF into its model: runs with the RTF
//!    group each was written in, fields, pictures, paragraphs and tables.
//! 2. **The dialect** (the `dialect` module): what Scrivener writes as text inside that RTF (inline
//!    notes and annotations, style markers, linked images) and as fields (comments, linked
//!    footnotes, document links) is read and rewritten into what the model says directly:
//!    a footnote, a comment with its range, a paragraph's kind.
//! 3. **The mapping** (`document_ingest::sources::rtf::to_rich`, then `rich::assemble`):
//!    the same code that brings a Word document in, so Scrivener's prose is written as Djot
//!    by the same emitter, proved by the same parser, and its comments anchored by the same
//!    rule (X-01, X-02).
//!
//! What cannot be carried is counted on the way and reported as [`ProseNotice`]s, never as a
//! sentence.

pub(crate) mod dialect;
pub mod notice;
pub(crate) mod sidecar;
pub(crate) mod styles;

use std::collections::{BTreeMap, HashMap};

use document_ingest::pictures::PictureRefusal;
use document_ingest::sources::rich::{self, RichBlock, RichDocument};
use document_ingest::sources::rtf::{self, FootnoteLabels, NotCarried, RtfOptions, SourcedPicture};
use document_ingest::{DiagnosticSeverity, ImportDiagnostic, SourceBlock, SourceDocument};
use serde::{Deserialize, Serialize};

pub use notice::{CompileTreatment, Formatting, LeftOut, Place, ProseNotice, RangeReading};

/// The writer's choices about the prose, from the review (UQ-33).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProseChoices {
    /// Blank lines between paragraphs of prose in the manuscript become scene breaks.
    pub blank_lines_as_scene_breaks: bool,
    /// Centred lines of ornaments in the manuscript become scene breaks.
    pub ornaments_as_scene_breaks: bool,
}

impl Default for ProseChoices {
    fn default() -> Self {
        ProseChoices {
            blank_lines_as_scene_breaks: true,
            ornaments_as_scene_breaks: false,
        }
    }
}

/// Where a text's pictures come from, for the project that keeps them.
pub(crate) trait Pictures {
    /// A picture written into a text: `key` names it uniquely in the project, `name` is what
    /// the text calls it.
    fn embedded(
        &self,
        key: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<SourcedPicture, PictureRefusal>;
    /// A picture a text links to: one of the project's own items, by UUID.
    fn linked(&self, uuid: &str) -> Result<SourcedPicture, PictureRefusal>;
}

/// One text, and what it is read with.
pub(crate) struct Stream<'a> {
    pub(crate) at: Place,
    /// What names the text's pictures uniquely in the project: its member path.
    pub(crate) key: &'a str,
    pub(crate) styles: styles::StreamStyles<'a>,
    pub(crate) sidecar: Option<&'a sidecar::Sidecar>,
    pub(crate) title: Option<&'a str>,
    pub(crate) footnote_marker: Option<&'a str>,
    pub(crate) scene_breaks: dialect::SceneBreaks,
    /// The top-level blocks of the text where a part starts, each a chapter heading whose
    /// words become that part's title: part 0 is what comes before the first, part `n`
    /// what follows heading `n`. Empty for a text read whole.
    pub(crate) splits: &'a [u32],
    /// The text is manuscript prose that exports: what Scrivener's compile left out of it
    /// (struck-through text, styles a compile format deletes or passes raw) is kept here,
    /// will print, and is said. Elsewhere (notes, text that does not export) nothing prints
    /// it either, so nothing is said.
    pub(crate) compiled: bool,
    /// The project's compile removes struck-through text (D5.21).
    pub(crate) struck_compiled_away: bool,
}

/// A text converted: one document per part, what it links to, and what to report.
pub(crate) struct Converted {
    pub(crate) parts: Vec<SourceDocument>,
    /// Targets of links to other documents, as written, each time one occurs.
    pub(crate) links: Vec<String>,
    pub(crate) notices: Vec<ProseNotice>,
    /// Compile placeholders the text keeps, by name: reported by the converter, which knows
    /// whether the row exports.
    pub(crate) placeholders: BTreeMap<String, u32>,
}

/// The most paragraphs a row holds before the report says the editor will be slow to open
/// it (X-11): text-document's load grows with the square of a document's paragraphs.
const LONG_DOCUMENT: usize = 4_000;

/// Convert one RTF text.
pub(crate) fn rtf_stream(
    bytes: &[u8],
    stream: &Stream<'_>,
    pictures: &dyn Pictures,
    labels: FootnoteLabels<'_>,
) -> Converted {
    let parts_wanted = stream.splits.len() + 1;
    let doc = match rtf_reader::parse(bytes) {
        Ok(doc) => doc,
        Err(rtf_reader::Refusal::NotRtf) => {
            // Some files named `.rtf` hold plain text: read them as such, and say so.
            let text = String::from_utf8_lossy(bytes);
            let mut converted = plain_stream(&text, stream.at, parts_wanted);
            if !text.trim().is_empty() {
                converted
                    .notices
                    .push(ProseNotice::ReadAsPlainText { at: stream.at });
            }
            return converted;
        }
        Err(_) => {
            let mut converted = plain_stream("", stream.at, parts_wanted);
            converted
                .notices
                .push(ProseNotice::Unreadable { at: stream.at });
            return converted;
        }
    };

    let input = dialect::Input {
        role: dialect::Role::Stream,
        styles: stream.styles,
        sidecar: stream.sidecar,
        title: stream.title,
        footnote_marker: stream.footnote_marker,
        scene_breaks: stream.scene_breaks,
        annotations_as_prose: false,
    };
    // A document holding nothing but annotations keeps them as its text (X-03): known only
    // once they are taken out, so the model is kept for a second reading when it may.
    let second = memchr_str(bytes, b"Scrv_annot").then(|| doc.clone());
    let mut rewritten = dialect::rewrite(doc, &input);
    if let Some(original) = second
        && rewritten.found.annotations > 0
        && !holds_writing(&rewritten.doc.body)
    {
        rewritten = dialect::rewrite(
            original,
            &dialect::Input {
                annotations_as_prose: true,
                ..input
            },
        );
        rewritten.found.annotations_kept_as_prose = true;
    }

    let linked: HashMap<usize, String> = rewritten
        .image_links
        .iter()
        .map(|link| (link.picture, link.uuid.clone()))
        .collect();
    let source = |id: usize, picture: &rtf_reader::Picture| match linked.get(&id) {
        Some(uuid) => pictures.linked(uuid),
        None if picture.data.is_empty() => Err(match picture.format {
            rtf_reader::PictureFormat::NextAttachment => PictureRefusal::Missing,
            rtf_reader::PictureFormat::Pdf => PictureRefusal::Unsupported { format: "PDF" },
            _ => PictureRefusal::Unreadable,
        }),
        None => {
            // A picture the file names nothing is named by its number: the name stays with
            // the picture in the project, so it holds no word of any language.
            let name = picture
                .name
                .clone()
                .filter(|n| !n.trim().is_empty())
                .unwrap_or_else(|| (id + 1).to_string());
            pictures.embedded(&format!("{}#{id}", stream.key), &name, &picture.data)
        }
    };
    let code_fonts: Vec<i32> = rewritten.code_font.into_iter().collect();
    let options = RtfOptions {
        origin: "",
        picture_source: Some(&source),
        headings_in_prose: true,
        code_fonts: &code_fonts,
        footnote_labels: Some(labels),
    };
    let conversion = rtf::to_rich(&rewritten.doc, &options);

    let mut found = rewritten.found;
    if !stream.compiled {
        found.styles_compiled_away.clear();
    }
    let mut notices = found_notices(stream.at, &found);
    notices.extend(carried_notices(stream.at, &conversion.not_carried));
    if stream.compiled && stream.struck_compiled_away {
        let struck = struck_characters(&conversion.document);
        if struck > 0 {
            notices.push(ProseNotice::StruckTextCompiledAway {
                at: stream.at,
                count: struck,
            });
        }
    }
    for diagnostic in &conversion.diagnostics {
        notices.push(document_notice(stream.at, diagnostic));
    }

    // Which part each block belongs to, through the rewrite's own map back to the RTF the
    // headings were found in.
    let part_of = |block: usize| -> PartOf {
        let original = conversion
            .block_origins
            .get(block)
            .and_then(|rewritten_index| rewritten.origins.get(*rewritten_index))
            .map_or(0, |original| *original as u32);
        match stream.splits.binary_search(&original) {
            // The heading itself: its words are the part's title.
            Ok(k) => PartOf::Heading(k + 1),
            Err(k) => PartOf::Body(k),
        }
    };
    let (parts, lost_on_headings, heading_pictures) =
        split(conversion.document, parts_wanted, &part_of);
    if lost_on_headings > 0 {
        notices.push(ProseNotice::NoteOnHeadingLost {
            at: stream.at,
            count: lost_on_headings,
        });
    }
    // Brought in with the text, then left with the heading that became a title: named, as
    // the document importer names a heading's picture. The converter stores only the
    // pictures the prose shows, so none is kept unseen.
    for name in heading_pictures {
        notices.push(document_notice(
            stream.at,
            &ImportDiagnostic::ImageNotIngested {
                path: String::new(),
                target: name,
                reason: PictureRefusal::InHeading,
            },
        ));
    }

    let mut documents = Vec::with_capacity(parts.len());
    for part in parts {
        let mut out = SourceDocument::new("", "");
        if rich::assemble(&part, &mut out).is_err() {
            // The emitter's proof could not run at all (the parser failed on its own
            // output): the part's words are kept as plain text, reported. The parser's
            // own account of why is English and technical, and stays out of the report.
            let text = part
                .blocks
                .iter()
                .map(RichBlock::plain_text)
                .collect::<Vec<_>>()
                .join("\n");
            out = plain_document(&text);
            notices.push(ProseNotice::Document {
                at: stream.at,
                key: "prose-not-verbatim".to_string(),
                warning: true,
                count: 1,
                detail: String::new(),
            });
        }
        for diagnostic in std::mem::take(&mut out.diagnostics) {
            notices.push(document_notice(stream.at, &diagnostic));
        }
        let paragraphs: usize = out
            .blocks
            .iter()
            .map(|b| b.plain_text().split('\n').count())
            .sum();
        if paragraphs > LONG_DOCUMENT {
            notices.push(ProseNotice::LongDocument {
                at: stream.at,
                paragraphs: paragraphs as u32,
            });
        }
        documents.push(out);
    }
    Converted {
        parts: documents,
        links: rewritten.links,
        notices,
        placeholders: found.placeholders,
    }
}

/// The characters a document keeps struck through, in its text, its tables and its
/// footnotes: white space aside, since struck blank space is not text a reader would miss.
fn struck_characters(doc: &RichDocument) -> u32 {
    let count = |runs: &[rich::Run]| -> usize {
        runs.iter()
            .filter(|run| run.style.strikethrough && run.image.is_none() && run.footnote.is_none())
            .map(|run| run.text.chars().filter(|c| !c.is_whitespace()).count())
            .sum()
    };
    let mut total = 0usize;
    for block in &doc.blocks {
        match block {
            RichBlock::Paragraph { runs, .. } => total += count(runs),
            RichBlock::Table { rows } => {
                total += rows
                    .iter()
                    .flat_map(|row| row.iter())
                    .map(|cell| count(cell))
                    .sum::<usize>();
            }
        }
    }
    for note in &doc.footnotes {
        total += note.paragraphs.iter().map(|p| count(p)).sum::<usize>();
    }
    u32::try_from(total).unwrap_or(u32::MAX)
}

/// Plain text (a synopsis, a `.txt` research file, text Scrivener's search index kept):
/// one paragraph per line, every character kept as written.
pub(crate) fn plain_stream(text: &str, _at: Place, parts: usize) -> Converted {
    let mut documents = vec![plain_document(text)];
    documents.extend((1..parts).map(|_| SourceDocument::new("", "")));
    Converted {
        parts: documents,
        links: Vec::new(),
        notices: Vec::new(),
        placeholders: BTreeMap::new(),
    }
}

/// A document holding `text` as one block of plain paragraphs.
pub(crate) fn plain_document(text: &str) -> SourceDocument {
    let mut doc = SourceDocument::new("", "");
    let djot = skrib_format::plain_text_to_djot_verbatim(text);
    if !djot.trim().is_empty() {
        let lines: Vec<&str> = text
            .split(['\n', '\r'])
            .map(skrib_format::trim_djot_whitespace)
            .filter(|l| !l.is_empty())
            .collect();
        doc.blocks.push(SourceBlock::prose(djot, lines.join("\n")));
    }
    doc
}

/// Where a block of a split text goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartOf {
    /// Into this part.
    Body(usize),
    /// Nowhere: it is the heading that opens this part, and its words are its title.
    Heading(usize),
}

/// Split a mapped text into its parts. A heading that became a part's title is left out: a
/// comment on it goes to the first paragraph of its part, and a footnote cited from it is
/// lost with it, both counted; a picture it showed is lost with it too, and named (the
/// third value, the pictures' names).
fn split(
    document: RichDocument,
    parts: usize,
    part_of: &dyn Fn(usize) -> PartOf,
) -> (Vec<RichDocument>, u32, Vec<String>) {
    if parts <= 1 {
        return (vec![document], 0, Vec::new());
    }
    let last = parts - 1;
    let RichDocument {
        blocks,
        annotations,
        footnotes,
        ..
    } = document;
    let mut out: Vec<RichDocument> = (0..parts).map(|_| RichDocument::default()).collect();
    // Where each block lands: its part and its index there, or the part it heads.
    let mut landing: Vec<Result<(usize, usize), usize>> = Vec::with_capacity(blocks.len());
    let mut lost = 0u32;
    let mut pictures: Vec<String> = Vec::new();
    let mut cited_in: HashMap<String, usize> = HashMap::new();
    for (index, block) in blocks.into_iter().enumerate() {
        match part_of(index) {
            PartOf::Body(part) => {
                let part = part.min(last);
                for label in footnote_labels(&block) {
                    cited_in.insert(label, part);
                }
                let slot = &mut out[part];
                landing.push(Ok((part, slot.blocks.len())));
                slot.blocks.push(block);
            }
            PartOf::Heading(part) => {
                lost += footnote_labels(&block).len() as u32;
                if let RichBlock::Paragraph { runs, .. } = &block {
                    pictures.extend(
                        runs.iter()
                            .filter_map(|run| run.image.as_ref())
                            .map(|image| image.name.clone()),
                    );
                }
                landing.push(Err(part.min(last)));
            }
        }
    }
    for mut annotation in annotations {
        match landing.get(annotation.block_index).copied() {
            Some(Ok((part, index))) => {
                annotation.block_index = index;
                annotation.end = annotation.end.and_then(|end| {
                    match landing.get(end.block_index).copied() {
                        Some(Ok((end_part, end_index))) if end_part == part => {
                            Some(rich::AnnotationEnd {
                                block_index: end_index,
                                offset: end.offset,
                            })
                        }
                        // Its range runs past its part: it stops where the part does.
                        _ => None,
                    }
                });
                out[part].annotations.push(annotation);
            }
            Some(Err(part)) => {
                lost += 1;
                annotation.block_index = 0;
                annotation.start = 0;
                annotation.length = 0;
                annotation.end = None;
                out[part].annotations.push(annotation);
            }
            // Placed nowhere by the mapping: the first part's own rule places it.
            None => out[0].annotations.push(annotation),
        }
    }
    for footnote in footnotes {
        if let Some(part) = cited_in.get(&footnote.label) {
            out[*part].footnotes.push(footnote);
        }
    }
    rehome_comments(&mut out);
    (out, lost, pictures)
}

/// A part holding no writing (the text before a first heading that has none, a chapter
/// whose heading the next one follows at once) stores nothing, and a comment left in it
/// would go with it: it goes on the first paragraph of writing after it, else on the last
/// before it, as a comment on that paragraph.
fn rehome_comments(parts: &mut [RichDocument]) {
    let writing = |block: &RichBlock| !block.plain_text().trim().is_empty();
    for k in 0..parts.len() {
        if parts[k].annotations.is_empty() || parts[k].blocks.iter().any(writing) {
            continue;
        }
        let after = (k + 1..parts.len())
            .find_map(|j| parts[j].blocks.iter().position(writing).map(|b| (j, b)));
        let before = || {
            (0..k)
                .rev()
                .find_map(|j| parts[j].blocks.iter().rposition(writing).map(|b| (j, b)))
        };
        let Some((home, block)) = after.or_else(before) else {
            continue;
        };
        let moved: Vec<rich::RichAnnotation> = std::mem::take(&mut parts[k].annotations)
            .into_iter()
            .map(|mut annotation| {
                annotation.block_index = block;
                annotation.start = 0;
                annotation.length = 0;
                annotation.end = None;
                annotation.between_blocks = false;
                annotation
            })
            .collect();
        parts[home].annotations.extend(moved);
    }
}

/// The labels of the footnotes a block cites.
fn footnote_labels(block: &RichBlock) -> Vec<String> {
    let mut out = Vec::new();
    let mut add = |runs: &[rich::Run]| {
        out.extend(runs.iter().filter_map(|r| r.footnote.clone()));
    };
    match block {
        RichBlock::Paragraph { runs, .. } => add(runs),
        RichBlock::Table { rows } => {
            for cell in rows.iter().flatten() {
                add(cell);
            }
        }
    }
    out
}

/// Whether blocks hold any writing: text that is not blank, a picture, a note.
fn holds_writing(blocks: &[rtf_reader::Block]) -> bool {
    rtf_reader::paragraphs(blocks).any(|p| {
        p.inlines.iter().any(|inline| match inline {
            rtf_reader::Inline::Text(run) => run
                .text
                .chars()
                .any(|c| !c.is_whitespace() && c != '\u{FFFC}'),
            rtf_reader::Inline::Picture(_) | rtf_reader::Inline::NoteReference(_) => true,
            _ => false,
        })
    })
}

/// Whether `needle` occurs in `haystack`.
fn memchr_str(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|w| w == needle)
}

/// What the dialect pass found, as notices.
fn found_notices(at: Place, found: &dialect::Found) -> Vec<ProseNotice> {
    let mut out = Vec::new();
    let mut count = |n: u32, make: &dyn Fn(u32) -> ProseNotice| {
        if n > 0 {
            out.push(make(n));
        }
    };
    count(found.unmatched, &|count| ProseNotice::InlineNoteUnmatched {
        at,
        count,
    });
    count(found.styles_unresolved, &|count| {
        ProseNotice::StyleUnresolved { at, count }
    });
    count(found.anchors_without_entry, &|count| {
        ProseNotice::CommentAnchorWithoutEntry { at, count }
    });
    count(found.comments_without_anchor, &|count| {
        ProseNotice::CommentEntryWithoutAnchor { at, count }
    });
    count(found.footnotes_without_anchor, &|count| {
        ProseNotice::FootnoteEntryWithoutAnchor { at, count }
    });
    count(found.ranges_approximate, &|count| {
        ProseNotice::RangeCommentApproximate { at, count }
    });
    count(found.markdown, &|paragraphs| ProseNotice::MarkdownAsText {
        at,
        paragraphs,
    });
    count(found.citations, &|paragraphs| {
        ProseNotice::CitationKeysAsText { at, paragraphs }
    });
    count(found.ornaments_kept, &|count| {
        ProseNotice::OrnamentLinesKept { at, count }
    });
    count(found.merged, &|count| ProseNotice::InlineNotesMerged {
        at,
        count,
    });
    count(found.joined_across, &|count| {
        ProseNotice::InlineNotesJoinedAcrossParagraphs { at, count }
    });
    count(found.blank_notes, &|count| ProseNotice::BlankNotesDropped {
        at,
        count,
    });
    count(found.annotations, &|count| {
        ProseNotice::AnnotationsBecameComments { at, count }
    });
    count(found.preserve_formatting, &|count| {
        ProseNotice::PreserveFormattingPassages { at, count }
    });
    count(found.nested_unwrapped, &|count| {
        ProseNotice::NestedNoteUnwrapped { at, count }
    });
    count(found.soft_breaks, &|count| ProseNotice::SoftBreaksSplit {
        at,
        count,
    });
    count(found.typed_indents, &|count| {
        ProseNotice::TypedIndentsRemoved { at, count }
    });
    count(found.left_indents, &|count| {
        ProseNotice::LeftIndentsNotKept { at, count }
    });
    count(found.blank_lines_removed, &|count| {
        ProseNotice::BlankLinesRemoved { at, count }
    });
    count(found.stanza_gaps, &|count| {
        ProseNotice::VerseStanzaGapsLost { at, count }
    });
    count(found.blank_lines_breaks, &|count| {
        ProseNotice::BlankLinesBecameSceneBreaks { at, count }
    });
    count(found.ornament_breaks, &|count| {
        ProseNotice::OrnamentsBecameSceneBreaks { at, count }
    });
    count(found.tables, &|count| ProseNotice::TableFirstRowHeader {
        at,
        count,
    });
    count(found.justified, &|count| {
        ProseNotice::JustifiedTextNotKept { at, count }
    });
    count(found.comment_colours + found.annotation_colours, &|count| {
        ProseNotice::CommentColoursLost { at, count }
    });
    for (key, n) in &found.range_readings {
        out.push(ProseNotice::RangeCommentReading {
            at,
            reading: key.reading(),
            count: *n,
        });
    }
    for (name, n) in &found.styles_not_carried {
        out.push(ProseNotice::StyleNotCarried {
            at,
            name: name.clone(),
            count: *n,
        });
    }
    for ((name, treatment), n) in &found.styles_compiled_away {
        out.push(ProseNotice::CompiledAwayStylePrinted {
            at,
            name: name.clone(),
            treatment: *treatment,
            count: *n,
        });
    }
    for name in &found.linked_on_disk {
        out.push(ProseNotice::LinkedPictureNotRead {
            at,
            name: name.clone(),
        });
    }
    if found.annotations_kept_as_prose {
        out.push(ProseNotice::AnnotationOnlyDocumentKept { at });
    }
    if found.heading_repeats_title {
        out.push(ProseNotice::HeadingRepeatsTitle { at });
    }
    out
}

/// What the mapping counted and has no place for, as notices.
fn carried_notices(at: Place, carried: &NotCarried) -> Vec<ProseNotice> {
    let mut out = Vec::new();
    for (what, n) in [
        (Formatting::Colour, carried.coloured_runs),
        (Formatting::Highlight, carried.highlighted_runs),
        (Formatting::SmallCaps, carried.small_caps_runs),
        (Formatting::AllCaps, carried.all_caps_runs),
        (Formatting::Hidden, carried.hidden_characters),
    ] {
        if n > 0 {
            out.push(ProseNotice::FormattingLost {
                at,
                what,
                count: n as u32,
            });
        }
    }
    if carried.links_kept_as_text > 0 {
        out.push(ProseNotice::OtherLinksKeptAsText {
            at,
            count: carried.links_kept_as_text as u32,
        });
    }
    if carried.joined_cells > 0 {
        out.push(ProseNotice::TableCellsJoined {
            at,
            count: carried.joined_cells as u32,
        });
    }
    let removed = carried.control_characters + carried.object_replacement_characters;
    if removed > 0 {
        out.push(ProseNotice::ControlCharactersRemoved {
            at,
            count: removed as u32,
        });
    }
    // What holds a writer's words or pictures and no text carries. The mapping's other
    // counts are not: a skipped ignorable group is what RTF asks of a reader (Scrivener's
    // own write several into every document), a page number is not the writer's, and a
    // nested table or an endnote arrives in another form.
    for (what, n) in [
        (LeftOut::AfterDocument, carried.after_document),
        (LeftOut::HeaderOrFooter, carried.headers_and_footers),
        (LeftOut::ListLabelPicture, carried.label_pictures),
    ] {
        if n > 0 {
            out.push(ProseNotice::LeftOut {
                at,
                what,
                count: u32::try_from(n).unwrap_or(u32::MAX),
            });
        }
    }
    if carried.undecodable_characters > 0 {
        out.push(ProseNotice::UndecodableCharacters {
            at,
            count: u32::try_from(carried.undecodable_characters).unwrap_or(u32::MAX),
        });
    }
    out
}

/// One of the document importer's own findings, under its own key.
pub(crate) fn document_notice(at: Place, diagnostic: &ImportDiagnostic) -> ProseNotice {
    // What the document import's own review row carries, so the sentences written for
    // its keys word this finding alike.
    let (detail, count) = diagnostic.detail_and_count();
    ProseNotice::Document {
        at,
        key: diagnostic.key().to_string(),
        warning: diagnostic.severity() != DiagnosticSeverity::Info,
        count: u32::try_from(count).unwrap_or(u32::MAX),
        detail,
    }
}

#[cfg(test)]
mod tests;
