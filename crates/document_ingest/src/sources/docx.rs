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
//! Several constructs a manuscript genuinely uses are invisible to the typed reader, and
//! `RawScan` reads them straight out of the container's own XML:
//!
//! * **A comment anchored to a point rather than a range.** LibreOffice's `.docx`
//!   export writes these as a bare `w:commentReference` with no
//!   `w:commentRangeStart` at all — verified by converting a document with one. Left
//!   to the typed tree, every such comment would land on its row as a whole and
//!   report itself unanchored. Read here, it becomes a comment on the paragraph, or the
//!   table cell, the reference sat in, which is what it is.
//! * **Footnotes and endnotes**, their references and their text, which the typed
//!   reader does not surface at all. Word numbers the two kinds separately, so each has
//!   its own map and its own label.
//! * **A horizontal rule** — an empty paragraph carrying a bottom border and nothing
//!   else. `ParagraphBorders` keeps every side private, so this cannot be asked of
//!   the typed tree; and refusing to read it would make DOCX unable to carry a break
//!   its writer can see, for the same reason the ODT scanner reads ODF's spelling.
//! * **Where a link goes, and which part a picture is.** The typed reader gives a
//!   hyperlink and a picture a relationship id and never what it names; the addresses
//!   are in the relationships of the part holding them, the comments part having its
//!   own.
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
//!
//! Two things the typed reader cannot read at all are put right before either reader
//! sees the file (`prepared_package`): a no-break or soft hyphen, which Word writes as
//! an element of its own, and a main part with no relationships part.
//!
//! ## A part `docx-rs` could not finish is refused before it is read
//!
//! `docx_rs::read_docx` does not return on a part cut short, one with a syntax error or
//! an ill-formed tag somewhere a reader reads past, or a style sheet or relationships
//! part whose root is not the element its loop waits for: it spins on the import's
//! thread for good, and no cancel reaches it. Every part it will read is checked first
//! (`check_as_docx_rs_reads`, and `Demand` for why each shape spins), and a file holding
//! such a part is reported as unreadable, naming the part. A part well-formed to its end,
//! under the root element its reader waits for, is never refused.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

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
    /// will be parsed has been checked against `MAX_XML_DEPTH`, and every part
    /// `docx-rs` reads checked for being one it can finish reading (see
    /// `refuse_unreadable_parts`). `docx-rs` descends once for every table nested
    /// in a table cell, about 50 KB of stack each in a debug build, and cannot be
    /// stopped once it has started; the check is what bounds it, and the stack is
    /// what gives the bound room.
    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        skrib_format::xml_depth::on_parser_stack(|| scan_document(bytes, display_name, origin))
            .map_err(anyhow::Error::new)?
    }
}

/// [`DocxScanner::scan`], on the parser stack.
fn scan_document(bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
    refuse_oversized_parts(bytes)?;
    refuse_unreadable_parts(bytes)?;
    // Both readers below read the same prepared package, so they count the same text.
    let package = prepared_package(bytes);
    let bytes: &[u8] = &package;
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
    let comments = CommentTable::new(&docx, &styles, &raw);
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
// What reaches docx-rs
// ---------------------------------------------------------------------------

/// The package-level relationship naming the main document part, and the one
/// naming the custom properties: the two `_rels/.rels` targets `docx-rs` follows.
const OFFICE_DOCUMENT: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const CUSTOM_PROPERTIES: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties";

/// The style sheet, whose part `docx-rs` reads until a `styles` end tag.
const STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
/// The comments part, which `docx-rs` reads for the comments' own text.
const COMMENTS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
/// A header and a footer, each of whose parts has relationships of its own that
/// `read_docx` reads.
const HEADER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
const FOOTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer";

/// The document-level relationships whose targets `docx-rs` parses as XML: every
/// type `read_docx` follows out of the document's own `.rels` except `image` and
/// `hyperlink`, spelled and matched exactly as its `reader/namespace.rs` spells
/// them. `docx-rs` is pinned to one release (`=0.4.22`, see Cargo.toml), and this
/// list is part of what moving off it has to re-read.
const XML_RELATIONSHIPS: [&str; 9] = [
    STYLES,
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
    COMMENTS,
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
    HEADER,
    FOOTER,
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
];

/// The root element of a relationships part, as `docx-rs` names it.
const RELATIONSHIPS_ROOT: &str = "Relationships";
/// The root element of the style sheet, as `docx-rs` names it.
const STYLES_ROOT: &str = "styles";

/// What `docx-rs` needs of a part to finish reading it.
///
/// `docx-rs` 0.4.22 reads a part through an event reader that answers every call
/// past the end of its input with one more `EndDocument`, and most of its readers
/// are loops that leave only at the end tag of the element they were entered on,
/// passing over `EndDocument` like any other event they have no use for
/// (`_ => {}`). Several also catch a child's error and read on (`if let Ok(table)
/// = Table::read(..)`), and after a syntax error `quick-xml` reports nothing but
/// the end of the input, while `ignore_element`, which skips a tracked move or a
/// property change, passes over errors altogether. So a reader still waiting for
/// its end tag when the input runs out waits for ever, at full speed, on the
/// thread the import runs on, and no cancel reaches it. [`check_as_docx_rs_reads`]
/// refuses such a part before `docx-rs` sees it.
///
/// None of this can happen to a part that is well-formed to its end: every reader
/// leaves at the end tag carrying the local name it was entered on, a child reader
/// always leaves at or before its own element's end, and so the only readers left
/// when the input runs out are the whole-part loops. All of those stop at the end
/// of the input except two, which is what [`Demand::roots`] is for.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Demand {
    /// Read by those element readers, so it must be well-formed to its end: every
    /// element it opens closed, and no syntax error or ill-formed tag anywhere.
    /// False for a part read only by a loop that stops at the end of its input,
    /// whatever the input holds: `[Content_Types].xml` and `_rels/.rels`, read by
    /// iterators, and the raw pass's own parts, read by `roxmltree`. Those are left
    /// as lenient as they were.
    to_its_end: bool,
    /// The name its root element must carry, for a part whose whole read stops at
    /// an end tag of that name rather than at the end of its input: `styles` for the
    /// style sheet (`Styles::from_xml`), `Relationships` for a part's relationships
    /// (`read_rels_xml`). Read in anything else, even a well-formed part, such a
    /// loop never stops. Two names for one part named for both, which no part can
    /// satisfy.
    roots: BTreeSet<&'static str>,
}

impl Demand {
    /// Read by the element readers.
    fn to_its_end() -> Self {
        Demand {
            to_its_end: true,
            roots: BTreeSet::new(),
        }
    }

    /// Read by the element readers, by a loop that stops only at the end of `root`.
    fn root(root: &'static str) -> Self {
        Demand {
            to_its_end: true,
            roots: BTreeSet::from([root]),
        }
    }

    /// Everything either demand asks, for a part read both ways.
    fn merge(&mut self, other: Demand) {
        self.to_its_end |= other.to_its_end;
        self.roots.extend(other.roots);
    }
}

/// A part `docx-rs` would never finish reading, refused before it is handed over.
///
/// Reported as the file being unreadable (`ImportDiagnostic::FileUnreadable`), which
/// is what it is: a Word document with a part cut short or damaged is refused by
/// Word as well. Its message completes that diagnostic's sentence, "could not be
/// read: …".
#[derive(Debug, Clone, PartialEq, Eq)]
struct UnfinishedPart {
    /// The member refused: `word/document.xml`, `word/_rels/document.xml.rels`.
    part: String,
    flaw: Flaw,
}

/// Why a part was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Flaw {
    /// A syntax error or an ill-formed tag, in the parser's own words, on the
    /// 1-based `line`.
    Malformed { error: String, line: usize },
    /// The part ends on `line` with `open` elements still open.
    CutShort { open: usize, line: usize },
    /// The root element is not the one the part must have; `found` is `None` when
    /// there is no element at all.
    Root {
        expected: &'static str,
        found: Option<String>,
    },
}

impl std::fmt::Display for UnfinishedPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part = &self.part;
        match &self.flaw {
            Flaw::Malformed { error, line } => {
                write!(f, "its part {part} is damaged at line {line} ({error})")
            }
            Flaw::CutShort { open: 1, line } => write!(
                f,
                "its part {part} is cut short at line {line}, with an element still open"
            ),
            Flaw::CutShort { open, line } => write!(
                f,
                "its part {part} is cut short at line {line}, with {open} elements still open"
            ),
            Flaw::Root {
                expected,
                found: Some(found),
            } => write!(
                f,
                "its part {part} should hold a <{expected}> element and holds <{found}> instead"
            ),
            Flaw::Root {
                expected,
                found: None,
            } => write!(
                f,
                "its part {part} should hold a <{expected}> element and holds none"
            ),
        }
    }
}

impl std::error::Error for UnfinishedPart {}

/// Refuse the file if a part this scan will parse as XML nests past
/// `MAX_XML_DEPTH`, or is one `docx-rs` would never finish reading ([`Demand`]).
///
/// **Which parts is the whole question**, because a `.docx` also carries images and
/// embedded objects, and a binary checked as if it were XML reads as a random
/// depth that a large enough picture always exceeds. So the set is exactly what
/// the two readers will parse, found the way they find it: the fixed names, the
/// main part wherever `_rels/.rels` points, every target of an XML-bearing type in
/// that part's own relationships, which `docx-rs` follows whatever the target is
/// called, and the relationships of every header and footer. The relationship
/// files themselves are read with `docx-rs`'s own readers, so a relationship this
/// check cannot see is one `docx-rs` cannot follow either; the main part's are
/// checked before they are read (see [`xml_parts`]).
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
fn refuse_unreadable_parts(bytes: &[u8]) -> Result<()> {
    let Ok(mut archive) = zip::ZipArchive::new(std::io::Cursor::new(bytes)) else {
        // Not a zip at all: `read_docx` says so, and parses nothing.
        return Ok(());
    };
    for (part, demand) in xml_parts(&mut archive)? {
        let Some(data) = read_member(&mut archive, &part) else {
            continue;
        };
        skrib_format::xml_depth::check(&part, &data)?;
        check_as_docx_rs_reads(&part, &data, &demand)?;
    }
    Ok(())
}

/// Refuse the file before `docx-rs` reads it if it is a zip built to unpack far
/// larger than it is: a decompression bomb, or a header declaring a member's size as
/// an enormous number it does not really hold.
///
/// **This must run before [`docx_rs::read_docx`]**, which is the one reader here that
/// does not go through [`skrib_format::zip_guard`]: it reserves each part's *declared*
/// size before reading a byte of it (`Vec::with_capacity`, in `docx-rs`'s own
/// `reader/read_zip.rs`), so a zip64 header claiming an exabyte aborts the process on
/// that reservation, and a member whose stream inflates past its declared size fills
/// memory as `docx-rs` reads it to the end. The guard weighs every member's declared
/// size over the central directory, then inflates each one under the ratio and the
/// ceiling without keeping it — so once this passes, every part `docx-rs` opens
/// reserves and reads no more than its budget.
///
/// Not a zip at all is `Ok`: `read_docx` says so, and parses nothing.
fn refuse_oversized_parts(bytes: &[u8]) -> Result<()> {
    let Ok((mut archive, mut guard)) = skrib_format::zip_guard::ZipGuard::open(
        crate::sources::DOCUMENT_ZIP_LIMITS,
        std::io::Cursor::new(bytes),
    ) else {
        return Ok(());
    };
    // Every member, because `docx-rs` follows the document's relationships to parts
    // this check cannot predict, and reads every image besides.
    guard.check_directory(&mut archive, |_| true)?;
    guard.verify_all(&mut archive)?;
    Ok(())
}

/// An empty relationships part: what a main part with no relationships of its own has.
const EMPTY_RELATIONSHIPS: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
    <Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"/>";

/// The package both readers read: the file itself, or a copy of it with what `docx-rs`
/// would otherwise get wrong put right. Called once the file has passed
/// [`refuse_oversized_parts`] and [`refuse_unreadable_parts`], so every part read here is
/// one those let through.
///
/// * **A main part with no relationships.** OPC makes a part's relationships optional,
///   and a package holding only `[Content_Types].xml`, `_rels/.rels` and
///   `word/document.xml` is the smallest Word document there is, the shape many scripts
///   write. `docx-rs` refuses it (`ZipError(FileNotFound)`) for want of
///   `word/_rels/document.xml.rels`, so the copy carries an empty one.
/// * **A no-break hyphen and a soft hyphen.** Word writes them as elements of their own,
///   `<w:noBreakHyphen/>` and `<w:softHyphen/>`, where LibreOffice's ODF writes the
///   characters. `docx-rs` has no reading for either element and drops it, so
///   "twenty‑one" arrived as "twentyone". The copy spells each as the character it
///   stands for, in a text element (see [`with_characters_as_text`]), in every part
///   either reader takes text from.
/// * **A symbol.** Word's Insert ▸ Symbol writes one from a symbol font as
///   `<w:sym w:font w:char>`, which `docx-rs` reads and nothing then shows: "α" arrived
///   as nothing at all. The copy spells it as a character too ([`symbol_character`]).
///
/// Rewriting the parts rather than patching the walk afterwards is what keeps the typed
/// walk and the raw pass counting the same characters: both read the copy.
///
/// When nothing needs changing, which is nearly always, the file's own bytes are
/// returned. Should the copy fail to build, which writing a zip into memory does not,
/// the file's own bytes are returned too: those corrections are lost, not the file.
fn prepared_package(bytes: &[u8]) -> std::borrow::Cow<'_, [u8]> {
    use std::borrow::Cow;

    let Ok(mut archive) = zip::ZipArchive::new(std::io::Cursor::new(bytes)) else {
        return Cow::Borrowed(bytes);
    };
    let main = main_part(&mut archive);
    let mut replaced: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    if let Some(rels) = rels_part_for(std::path::Path::new(&main))
        && archive.by_name(&rels).is_err()
    {
        replaced.insert(rels, EMPTY_RELATIONSHIPS.as_bytes().to_vec());
    }
    let mut text_parts: BTreeSet<String> = [
        "word/document.xml",
        "word/comments.xml",
        "word/footnotes.xml",
        "word/endnotes.xml",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    text_parts.insert(normalise_part(&main));
    if let Ok(rels) = docx_rs::read_document_rels(&mut archive, &main)
        && let Some(targets) = rels.find_target_path(COMMENTS)
    {
        for (_, path, _) in targets {
            text_parts.insert(normalise_part(&path.to_string_lossy()));
        }
    }
    for part in text_parts {
        if let Some(data) = read_member(&mut archive, &part)
            && let Some(rewritten) = with_characters_as_text(&data)
        {
            replaced.insert(part, rewritten);
        }
    }
    if replaced.is_empty() {
        return Cow::Borrowed(bytes);
    }
    match rebuilt(&mut archive, &replaced) {
        Ok(copy) => Cow::Owned(copy),
        Err(_) => Cow::Borrowed(bytes),
    }
}

/// The main document part, found as `docx-rs` finds it: through `_rels/.rels`, and
/// `word/document.xml` when the package names none.
fn main_part(archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>) -> String {
    use docx_rs::FromXML;

    docx_rs::read_zip(archive, "_rels/.rels")
        .ok()
        .and_then(|data| docx_rs::Rels::from_xml(&data[..]).ok())
        .and_then(|rels| rels.find_target(OFFICE_DOCUMENT).map(|rel| rel.2.clone()))
        .unwrap_or_else(|| "word/document.xml".to_string())
}

/// A copy of `archive` with the members in `replaced` holding those bytes instead, and
/// added where the archive has no such member. Every other member is copied as it is
/// stored, without being inflated again.
fn rebuilt(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
    replaced: &BTreeMap<String, Vec<u8>>,
) -> zip::result::ZipResult<Vec<u8>> {
    use std::io::Write;

    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let mut written: HashSet<String> = HashSet::new();
    for index in 0..archive.len() {
        let member = archive.by_index_raw(index)?;
        let name = member.name().to_string();
        if !written.insert(name.clone()) {
            // Two stored names that read as the same one: the writer refuses a name twice,
            // so the first is kept.
            continue;
        }
        match replaced.get(&name) {
            Some(data) => {
                drop(member);
                writer.start_file(name.as_str(), options)?;
                writer.write_all(data)?;
            }
            None => writer.raw_copy_file(member)?,
        }
    }
    for (name, data) in replaced {
        if written.insert(name.clone()) {
            writer.start_file(name.as_str(), options)?;
            writer.write_all(data)?;
        }
    }
    Ok(writer.finish()?.into_inner())
}

/// `xml` with every `<w:noBreakHyphen/>`, `<w:softHyphen/>` and `<w:sym>` written as the
/// character it stands for, in a text element of the same namespace prefix: a `<w:t>`, or
/// a `<w:delText>` inside a tracked deletion, where the text is somebody's removed words
/// and both readers leave it out. `None` when the part holds none of them, or cannot be
/// read to its end, in which case it is left as it is.
///
/// A no-break hyphen is U+2011 and a soft hyphen U+00AD. A symbol is the character
/// [`symbol_character`] finds for it; one it finds none for is left as the element it
/// was, which neither reader counts or shows.
///
/// The character goes in as a character reference, so the rewrite holds whatever
/// encoding the part declares. Everything else is written back event for event, exactly
/// as it was read.
///
/// An element spelled with an end tag rather than as an empty one is written as its
/// character with everything up to its own end tag left out, one of the same name nested
/// in it included, so no stray end tag is left behind. A part that ends before that end
/// tag, cut short, is left as it is: nothing in the parts this rewrites holds them to
/// their end before `docx-rs` reads them, and waiting for an end tag that never comes read
/// the end of the part for ever.
fn with_characters_as_text(xml: &[u8]) -> Option<Vec<u8>> {
    use quick_xml::events::Event;

    const NO_BREAK_HYPHEN: &[u8] = b"noBreakHyphen";
    const SOFT_HYPHEN: &[u8] = b"softHyphen";
    const SYMBOL: &[u8] = b"sym";
    let holds = |needle: &[u8]| xml.windows(needle.len()).any(|w| w == needle);
    // `sym` is also a piece of ordinary words ("symphony"), so it counts only as an
    // element's name: after the `<` or the prefix's `:` opening a tag, and before what
    // ends a name.
    let names_symbol = || {
        xml.windows(SYMBOL.len() + 2).any(|w| {
            matches!(w[0], b'<' | b':')
                && &w[1..=SYMBOL.len()] == SYMBOL
                && matches!(
                    w[SYMBOL.len() + 1],
                    b' ' | b'\t' | b'\n' | b'\r' | b'/' | b'>'
                )
        })
    };
    if !holds(NO_BREAK_HYPHEN) && !holds(SOFT_HYPHEN) && !names_symbol() {
        return None;
    }

    let mut reader = quick_xml::Reader::from_reader(xml);
    let config = reader.config_mut();
    config.trim_text(false);
    config.check_end_names = false;
    config.expand_empty_elements = false;
    let mut writer = quick_xml::Writer::new(Vec::with_capacity(xml.len() + 64));
    // How many tracked deletions (`w:del`, `w:moveFrom`) the reader is inside.
    let mut removed = 0usize;
    // Inside an element written as its character but spelled with an end tag, which is
    // skipped up to it: its name, and how many elements of that name are open.
    let mut skipping: Option<(Vec<u8>, usize)> = None;
    let mut changed = false;
    let character_of =
        |tag: &quick_xml::events::BytesStart<'_>| match local_part(tag.name().as_ref()) {
            NO_BREAK_HYPHEN => Some('\u{2011}'),
            SOFT_HYPHEN => Some('\u{AD}'),
            SYMBOL => {
                let mut font: Option<String> = None;
                let mut code: Option<String> = None;
                for attribute in tag.attributes().flatten() {
                    let value = String::from_utf8(attribute.value.to_vec()).ok();
                    match local_part(attribute.key.as_ref()) {
                        b"font" => font = value,
                        b"char" => code = value,
                        _ => {}
                    }
                }
                symbol_character(font.as_deref(), code.as_deref()?)
            }
            _ => None,
        };
    loop {
        let event = reader.read_event().ok()?;
        if let Some((name, open)) = &mut skipping {
            match &event {
                // Cut short inside it: the part is left as it is.
                Event::Eof => return None,
                Event::Start(tag) if tag.name().as_ref() == name.as_slice() => *open += 1,
                Event::End(end) if end.name().as_ref() == name.as_slice() => {
                    *open -= 1;
                    if *open == 0 {
                        skipping = None;
                    }
                }
                _ => {}
            }
            continue;
        }
        match &event {
            Event::Eof => break,
            Event::Start(tag) | Event::Empty(tag) => {
                if let Some(character) = character_of(tag) {
                    let qualified = tag.name().as_ref().to_vec();
                    let prefix = &qualified[..qualified.len() - local_part(&qualified).len()];
                    let element = if removed > 0 { "delText" } else { "t" };
                    let reference = format!("&#x{:X};", u32::from(character));
                    let out = writer.get_mut();
                    for piece in [
                        b"<".as_slice(),
                        prefix,
                        element.as_bytes(),
                        b">",
                        reference.as_bytes(),
                        b"</",
                        prefix,
                        element.as_bytes(),
                        b">",
                    ] {
                        out.extend_from_slice(piece);
                    }
                    if matches!(event, Event::Start(_)) {
                        skipping = Some((qualified, 1));
                    }
                    changed = true;
                    continue;
                }
                if matches!(event, Event::Start(_))
                    && matches!(local_part(tag.name().as_ref()), b"del" | b"moveFrom")
                {
                    removed += 1;
                }
            }
            Event::End(tag) if matches!(local_part(tag.name().as_ref()), b"del" | b"moveFrom") => {
                removed = removed.saturating_sub(1);
            }
            _ => {}
        }
        writer.write_event(event).ok()?;
    }
    changed.then(|| writer.into_inner())
}

/// The character a `<w:sym>` stands for, from its `w:font` and its `w:char`, a code
/// written in hexadecimal.
///
/// Word's Symbol font draws Greek letters, arrows and mathematical signs at the codes of
/// ordinary letters, and Word stores them from `F020` to `F0FF`; each becomes the
/// character it shows ([`symbol_font_character`]), so the "α" a writer picked arrives as
/// "α". Any other font's code is the character the file names. For a font of pictures
/// such as Wingdings that is a private-use character, which few fonts draw, the same one
/// LibreOffice writes into an OpenDocument file for it; for a font of letters it is that
/// letter.
///
/// `None` for a code that is not a number, or names a character a text element cannot
/// hold (a control character), which would make the part unreadable.
fn symbol_character(font: Option<&str>, code: &str) -> Option<char> {
    let code = u32::from_str_radix(code.trim(), 16).ok()?;
    let symbol_font = font.is_some_and(|font| font.trim().eq_ignore_ascii_case("Symbol"));
    // The code as the font's own byte: Word adds `F000` to it, some writers do not.
    let byte = match code {
        0x20..=0xFF => u8::try_from(code).ok(),
        0xF020..=0xF0FF => u8::try_from(code - 0xF000).ok(),
        _ => None,
    };
    let character = match byte {
        Some(byte) if symbol_font => {
            symbol_font_character(byte).or_else(|| char::from_u32(0xF000 | u32::from(byte)))?
        }
        _ => char::from_u32(code)?,
    };
    (!character.is_control() && !matches!(character, '\u{FFFE}' | '\u{FFFF}')).then_some(character)
}

/// The character Word's Symbol font shows at `code`, as Unicode's own mapping for that
/// font gives it (`VENDORS/APPLE/SYMBOL.TXT`), from `0x20` on. `None` where the font holds
/// nothing, or a piece of a drawn sign that Unicode has no character for: the radical's
/// extension at `0x60`.
fn symbol_font_character(code: u8) -> Option<char> {
    #[rustfmt::skip]
    const FROM_0X20: [u16; 224] = [
        0x0020, 0x0021, 0x2200, 0x0023, 0x2203, 0x0025, 0x0026, 0x220B, 0x0028, 0x0029, 0x2217, 0x002B, 0x002C, 0x2212, 0x002E, 0x002F,
        0x0030, 0x0031, 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x003A, 0x003B, 0x003C, 0x003D, 0x003E, 0x003F,
        0x2245, 0x0391, 0x0392, 0x03A7, 0x0394, 0x0395, 0x03A6, 0x0393, 0x0397, 0x0399, 0x03D1, 0x039A, 0x039B, 0x039C, 0x039D, 0x039F,
        0x03A0, 0x0398, 0x03A1, 0x03A3, 0x03A4, 0x03A5, 0x03C2, 0x03A9, 0x039E, 0x03A8, 0x0396, 0x005B, 0x2234, 0x005D, 0x22A5, 0x005F,
        0x0000, 0x03B1, 0x03B2, 0x03C7, 0x03B4, 0x03B5, 0x03C6, 0x03B3, 0x03B7, 0x03B9, 0x03D5, 0x03BA, 0x03BB, 0x03BC, 0x03BD, 0x03BF,
        0x03C0, 0x03B8, 0x03C1, 0x03C3, 0x03C4, 0x03C5, 0x03D6, 0x03C9, 0x03BE, 0x03C8, 0x03B6, 0x007B, 0x007C, 0x007D, 0x223C, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x20AC, 0x03D2, 0x2032, 0x2264, 0x2044, 0x221E, 0x0192, 0x2663, 0x2666, 0x2665, 0x2660, 0x2194, 0x2190, 0x2191, 0x2192, 0x2193,
        0x00B0, 0x00B1, 0x2033, 0x2265, 0x00D7, 0x221D, 0x2202, 0x2022, 0x00F7, 0x2260, 0x2261, 0x2248, 0x2026, 0x23D0, 0x23AF, 0x21B5,
        0x2135, 0x2111, 0x211C, 0x2118, 0x2297, 0x2295, 0x2205, 0x2229, 0x222A, 0x2283, 0x2287, 0x2284, 0x2282, 0x2286, 0x2208, 0x2209,
        0x2220, 0x2207, 0x00AE, 0x00A9, 0x2122, 0x220F, 0x221A, 0x22C5, 0x00AC, 0x2227, 0x2228, 0x21D4, 0x21D0, 0x21D1, 0x21D2, 0x21D3,
        0x25CA, 0x27E8, 0x00AE, 0x00A9, 0x2122, 0x2211, 0x239B, 0x239C, 0x239D, 0x23A1, 0x23A2, 0x23A3, 0x23A7, 0x23A8, 0x23A9, 0x23AA,
        0x0000, 0x27E9, 0x222B, 0x2320, 0x23AE, 0x2321, 0x239E, 0x239F, 0x23A0, 0x23A4, 0x23A5, 0x23A6, 0x23AB, 0x23AC, 0x23AD, 0x0000,
    ];
    let unicode = *FROM_0X20.get(usize::from(code.checked_sub(0x20)?))?;
    (unicode != 0)
        .then(|| char::from_u32(u32::from(unicode)))
        .flatten()
}

/// The bytes of the member `name`, or `None` when there is no such member or it
/// cannot be read, in which case `docx-rs` has nothing to parse either.
fn read_member(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
    name: &str,
) -> Option<Vec<u8>> {
    let mut member = archive.by_name(name).ok()?;
    let mut data = Vec::new();
    std::io::Read::read_to_end(&mut member, &mut data).ok()?;
    Some(data)
}

/// Refuse `xml` if `docx-rs` would read it nested past `MAX_XML_DEPTH`
/// (`skrib_format::XmlTooDeep`), or would never finish reading it as `demand`
/// describes ([`UnfinishedPart`]).
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
/// same stream, so the elements after it are parsed and have to be counted. That
/// is also why the depth is refused first: a part both damaged and nested too deep
/// is named for the nesting, which is the refusal with a sentence of its own.
///
/// One kind of element is left out of the count: one whose local name holds a NUL
/// byte. `docx-rs` recognises an element by comparing its local name with names
/// that hold none, so such an element can never enter one of its readers. It is
/// what every tag of a part genuinely written in UTF-16 looks like to a parser
/// reading it byte by byte, which `quick-xml` does, and counting them would refuse
/// such a file as nested past the ceiling, or as cut short, when `docx-rs` reads
/// nothing from it.
fn check_as_docx_rs_reads(part: &str, xml: &[u8], demand: &Demand) -> Result<()> {
    use quick_xml::events::Event;

    let mut reader = quick_xml::Reader::from_reader(std::io::BufReader::new(xml));
    let config = reader.config_mut();
    config.trim_text(false);
    config.check_end_names = true;
    config.expand_empty_elements = false;

    let line_at = |offset: u64| {
        let offset = usize::try_from(offset).unwrap_or(xml.len()).min(xml.len());
        1 + xml[..offset].iter().filter(|&&b| b == b'\n').count()
    };
    let refuse = |depth: usize, read_to: u64| {
        anyhow::Error::new(skrib_format::XmlTooDeep {
            part: part.to_string(),
            depth,
            line: line_at(read_to),
        })
    };
    let mut buf = Vec::new();
    let mut depth = 0usize;
    let mut errors_in_a_row = 0usize;
    // The first thing in the part that would keep a reader waiting for ever.
    let mut flaw: Option<Flaw> = None;
    // The local name of the first element `docx-rs` could recognise: the root.
    let mut root: Option<String> = None;
    loop {
        buf.clear();
        let event = reader.read_event_into(&mut buf);
        if event.is_ok() {
            errors_in_a_row = 0;
        }
        match event {
            Ok(Event::Start(tag)) if names_an_element(tag.name().as_ref()) => {
                root.get_or_insert_with(|| local_name(tag.name().as_ref()));
                depth += 1;
                if depth > skrib_format::MAX_XML_DEPTH {
                    return Err(refuse(depth, reader.buffer_position()));
                }
            }
            Ok(Event::Empty(tag)) if names_an_element(tag.name().as_ref()) => {
                root.get_or_insert_with(|| local_name(tag.name().as_ref()));
                if depth + 1 > skrib_format::MAX_XML_DEPTH {
                    return Err(refuse(depth + 1, reader.buffer_position()));
                }
            }
            Ok(Event::End(tag)) if names_an_element(tag.name().as_ref()) => {
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Eof) => {
                if depth > 0 && flaw.is_none() {
                    flaw = Some(Flaw::CutShort {
                        open: depth,
                        line: line_at(reader.buffer_position()),
                    });
                }
                break;
            }
            Ok(_) => {}
            Err(error) => {
                if flaw.is_none() {
                    flaw = Some(Flaw::Malformed {
                        error: error.to_string(),
                        line: line_at(reader.error_position()),
                    });
                }
                // An ill-formed error consumes what it rejected, a syntax error
                // ends the stream, and reading from memory has no I/O error, so
                // the stream reaches its end long before this. The count only
                // guarantees that the loop does.
                errors_in_a_row += 1;
                if errors_in_a_row > xml.len() {
                    break;
                }
            }
        }
    }

    let unfinished = |flaw: Flaw| {
        anyhow::Error::new(UnfinishedPart {
            part: part.to_string(),
            flaw,
        })
    };
    if demand.to_its_end
        && let Some(flaw) = flaw
    {
        return Err(unfinished(flaw));
    }
    if let Some(expected) = demand
        .roots
        .iter()
        .find(|expected| root.as_deref() != Some(**expected))
    {
        return Err(unfinished(Flaw::Root {
            expected,
            found: root,
        }));
    }
    Ok(())
}

/// Whether `docx-rs` could recognise an element by this qualified name: whether
/// its local part, everything after the first `:` as `docx-rs` splits it, is free
/// of NUL bytes. See [`check_as_docx_rs_reads`].
fn names_an_element(name: &[u8]) -> bool {
    !local_part(name).contains(&0)
}

/// The local part of a qualified name, split at the first `:` as `docx-rs` splits
/// it.
fn local_part(name: &[u8]) -> &[u8] {
    name.iter()
        .position(|&b| b == b':')
        .map_or(name, |colon| &name[colon + 1..])
}

/// [`local_part`], as text.
fn local_name(name: &[u8]) -> String {
    String::from_utf8_lossy(local_part(name)).into_owned()
}

/// A member name as `docx-rs`'s `read_zip` normalises it before looking it up.
fn normalise_part(name: &str) -> String {
    name.replace('\\', "/").trim_start_matches('/').to_string()
}

/// The name `docx-rs` reads `part`'s relationships from: `_rels/` beside it, then
/// its stem with `xml.rels` put in place of its last extension. Spelled as the
/// private `find_rels_filename` spells it, quirk included (`main.v1.xml` has its
/// relationships in `_rels/main.xml.rels`), and normalised as `read_zip` normalises
/// it. `None` where `docx-rs` finds no name either.
fn rels_part_for(part: &std::path::Path) -> Option<String> {
    let dir = part.parent()?;
    let base = part.file_stem()?;
    let rels = dir.join("_rels").join(base).with_extension("xml.rels");
    Some(normalise_part(rels.to_str()?))
}

/// Add `demand` to what is asked of the part `name`.
fn want(parts: &mut BTreeMap<String, Demand>, name: &str, demand: Demand) {
    parts.entry(normalise_part(name)).or_default().merge(demand);
}

/// Every part [`refuse_unreadable_parts`] has to check, by the name `docx-rs`'s
/// `read_zip` looks it up by, with what `docx-rs` needs of it.
///
/// **The main part's relationships are checked here, before they are read.**
/// Finding most of the parts means reading those relationships with `docx-rs`'s
/// own `read_document_rels`, whose loop stops only at a `Relationships` end tag
/// and passes over the end of its input: a relationships part cut short, empty,
/// or holding anything else would stop this very check, as it would stop
/// `read_docx`. Those are refused here instead.
fn xml_parts(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
) -> Result<BTreeMap<String, Demand>> {
    use docx_rs::FromXML;

    let mut parts: BTreeMap<String, Demand> = BTreeMap::new();
    for name in [
        // Read by `docx-rs`'s iterators, which stop at the end of their input.
        "[Content_Types].xml",
        "_rels/.rels",
        // The raw pass's parts, read by these names whatever the relationships
        // say, with `roxmltree`.
        "word/document.xml",
        "word/comments.xml",
        "word/footnotes.xml",
        "word/endnotes.xml",
        "word/_rels/document.xml.rels",
        "word/_rels/comments.xml.rels",
    ] {
        want(&mut parts, name, Demand::default());
    }

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
        want(&mut parts, &custom.2, Demand::to_its_end());
    }
    want(&mut parts, &main, Demand::to_its_end());

    if let Some(main_rels) = rels_part_for(std::path::Path::new(&main))
        && let Some(data) = read_member(archive, &main_rels)
    {
        check_as_docx_rs_reads(&main_rels, &data, &Demand::root(RELATIONSHIPS_ROOT))?;
    }
    if let Ok(rels) = docx_rs::read_document_rels(archive, &main) {
        for kind in XML_RELATIONSHIPS {
            for (_, path, _) in rels.find_target_path(kind).unwrap_or_default() {
                let demand = if kind == STYLES {
                    Demand::root(STYLES_ROOT)
                } else {
                    Demand::to_its_end()
                };
                want(&mut parts, &path.to_string_lossy(), demand);
                if (kind == HEADER || kind == FOOTER)
                    && let Some(own) = rels_part_for(&path)
                {
                    want(&mut parts, &own, Demand::root(RELATIONSHIPS_ROOT));
                }
            }
        }
    }
    Ok(parts)
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
///
/// The `+ 4762` that rounds to the nearest pixel overflows `u32` for an `emu` within
/// 4,762 of `u32::MAX` — a panic in a debug build, a wrong size in release — so the
/// rounding is saturating. A picture that large is not one anyone drew (`u32::MAX`
/// EMU is about 470 metres), but a `.docx`'s sizes are numbers from an untrusted
/// file, and a scanner must not abort on one.
fn emu_to_pixels(emu: u32) -> u32 {
    emu.saturating_add(9_525 / 2) / 9_525
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
    fn new(docx: &Docx, styles: &StyleTable, raw: &RawScan) -> Self {
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
            let attrs = raw.comment_attrs.get(&comment.id);
            by_id.insert(
                comment.id,
                meta_of(
                    comment,
                    &done_by_paragraph,
                    styles,
                    &raw.comment_links,
                    attrs,
                ),
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
    links: &HashMap<String, String>,
    attrs: Option<&CommentAttrs>,
) -> CommentMeta {
    let mut builder = CommentBodyBuilder::new(styles, links);
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
    /// The comments part's own links, by relationship id ([`RawScan::comment_links`]).
    links: &'a HashMap<String, String>,
    paragraphs: Vec<Vec<Run>>,
    current: Vec<Run>,
}

impl<'a> CommentBodyBuilder<'a> {
    fn new(styles: &'a StyleTable, links: &'a HashMap<String, String>) -> Self {
        CommentBodyBuilder {
            styles,
            links,
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
                    let url = link_target(&hyperlink.link, self.links);
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

/// The address a `w:hyperlink` goes to: `#bookmark` for one inside the document, and for
/// any other the target of the relationship it names, looked up in `links` (the
/// relationships of the part it sits in). `None` when that relationship is missing, and
/// the words are kept without a link.
///
/// `docx-rs` reads the relationship id and never the address (its reader leaves `path`
/// empty, "not used"), so the address is looked up here; a `path` it does fill is taken
/// as it is.
fn link_target(link: &docx_rs::HyperlinkData, links: &HashMap<String, String>) -> Option<String> {
    match link {
        docx_rs::HyperlinkData::External { rid, path } => {
            if path.trim().is_empty() {
                links.get(rid).cloned()
            } else {
                Some(path.clone())
            }
        }
        docx_rs::HyperlinkData::Anchor { anchor } => Some(format!("#{anchor}")),
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
/// **Keyed by where the typed walk meets the same thing.** The body is read as one
/// sequence of paragraphs and tables in document order, reached through any content
/// control (`w:sdt`) or other wrapper around them, which is how `docx-rs` reads it: its
/// body and content-control readers take every `w:p` and `w:tbl` they meet, whatever
/// holds them. The *n*-th paragraph of that sequence is the *n*-th the typed walk meets
/// ([`Walker::body_paragraph`]), and the same for tables, so the two passes agree without
/// either knowing about the other. `comment_attrs` needs no such join: it is keyed by the
/// comment's own `w:id`, which both this pass and `docx_rs::Comment::id` read off the
/// identical attribute.
#[derive(Default)]
struct RawScan {
    /// Paragraph ordinal → what sits between its characters: point comments, footnote and
    /// endnote references ([`RawMark`]).
    ///
    /// Notes are found here, in the raw pass, because **`docx-rs`'s reader never produces
    /// one**: nothing in its `src/reader/` constructs `RunChild::FootnoteReference`, so the
    /// typed walk's arm for it is unreachable and a scanner that trusted it would see a
    /// document with no notes in it. That is what made this the one silent loss in the
    /// whole import: a `.docx` coming back from an editor lost its footnotes and said
    /// nothing.
    paragraph_marks: HashMap<usize, Vec<RawMark>>,
    /// Table ordinal → the same, measured in the table's own plain text: its cells in
    /// the order [`Walker::table`] reads them, joined by one character, a cell's
    /// paragraphs by another.
    table_marks: HashMap<usize, Vec<RawMark>>,
    /// Paragraph ordinals that are a horizontal rule.
    rules: HashSet<usize>,
    /// `w:id` → the `skrb:uid`/`w:initials` attributes only Skribisto's own writer
    /// puts on that comment's `<w:comment>` — see [`CommentAttrs`].
    comment_attrs: HashMap<usize, CommentAttrs>,
    /// `w:id` → that footnote's own styled paragraphs, from `word/footnotes.xml`.
    footnote_bodies: HashMap<usize, Vec<Vec<Run>>>,
    /// `w:id` → that endnote's own, from `word/endnotes.xml`. A map of its own, because
    /// Word numbers footnotes and endnotes separately, both from 1: looked up among the
    /// footnotes, endnote 1 cited footnote 1's text.
    endnote_bodies: HashMap<usize, Vec<Vec<Run>>>,
    /// Paragraph ordinal → the direction and page break its own `w:pPr` states.
    ///
    /// Read here because `docx-rs` never reads `w:bidi` at all, and reads
    /// `w:pageBreakBefore` only when it is on, so a paragraph turning off the break its
    /// style asks for would be indistinguishable from one saying nothing.
    paragraph_props: HashMap<usize, RawParagraphProps>,
    /// The same two properties for each paragraph style of `word/styles.xml`, and the
    /// document's defaults, for a paragraph that states neither itself.
    styles: RawStyles,
    /// Relationship id → the address of each link the document's prose makes.
    ///
    /// Read here because `docx-rs` reads a hyperlink's relationship id and never its
    /// address: the `path` it gives `HyperlinkData::External` is always empty. Every
    /// external link Word writes is a relationship, so without this map every one of
    /// them arrived as its words alone.
    links: HashMap<String, String>,
    /// The same for the links inside comments, whose relationships are the comments
    /// part's own (`word/_rels/comments.xml.rels`), a part `docx-rs` never reads.
    comment_links: HashMap<String, String>,
    /// Relationship id → the picture it names: the part inside the file
    /// (`word/media/image1.png`), or the address of a linked one. What `docx-rs` gives a
    /// picture is the relationship id alone.
    images: HashMap<String, String>,
    /// How many equations the body holds ([`RawCount::equations`]). `docx-rs` reads each
    /// as a run with no text, which the typed walk cannot tell from any other, so without
    /// this count an equation vanished without a word.
    equations: usize,
}

/// Something the raw pass found between a paragraph's characters, and where.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawMark {
    kind: RawMarkKind,
    /// Which stretch of the paragraph it sits in: how many line breaks come before it.
    /// The typed walk makes a block of each stretch that holds anything
    /// ([`Walker::stretches`]). Always 0 in a table, where a line break is a space.
    stretch: usize,
    /// Characters into that stretch, counted as the typed walk counts them.
    offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawMarkKind {
    /// A bare `w:commentReference`, the comment with this id anchored to a point.
    Comment(usize),
    /// A `w:footnoteReference`.
    Footnote(usize),
    /// A `w:endnoteReference`.
    Endnote(usize),
}

/// One relationship a part makes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Relationship {
    kind: String,
    target: String,
    external: bool,
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
    /// as it would have without this pass at all. Every other part is read
    /// best-effort within the same zip open — a document with no comments has no
    /// `word/comments.xml`, and that is not an error either, just an empty map.
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

        let mut units = Vec::new();
        collect_blocks(body, &mut units, 0);
        let (mut paragraphs, mut tables) = (0usize, 0usize);
        for unit in units {
            match unit {
                RawBlock::Paragraph(paragraph) => {
                    let ordinal = paragraphs;
                    paragraphs += 1;
                    if is_horizontal_rule(paragraph) {
                        scan.rules.insert(ordinal);
                    }
                    let mut count = RawCount::new(&ranged, false);
                    count.walk(paragraph, 0);
                    scan.equations += count.equations;
                    if !count.marks.is_empty() {
                        scan.paragraph_marks.insert(ordinal, count.marks);
                    }
                    let props = RawParagraphProps::read(paragraph);
                    if props != RawParagraphProps::default() {
                        scan.paragraph_props.insert(ordinal, props);
                    }
                }
                RawBlock::Table(table) => {
                    let ordinal = tables;
                    tables += 1;
                    let mut count = RawCount::new(&ranged, true);
                    let mut first_cell = true;
                    count.table(table, &mut first_cell, 0);
                    scan.equations += count.equations;
                    if !count.marks.is_empty() {
                        scan.table_marks.insert(ordinal, count.marks);
                    }
                }
            }
        }

        scan.comment_attrs = read_comment_attrs(&mut zip).unwrap_or_default();
        scan.footnote_bodies = read_note_bodies(&mut zip, "word/footnotes.xml", "footnote");
        scan.endnote_bodies = read_note_bodies(&mut zip, "word/endnotes.xml", "endnote");
        scan.styles = RawStyles::read(&mut zip).unwrap_or_default();
        let document_relationships = read_relationships(&mut zip, "word/document.xml");
        scan.links = links_of(&document_relationships);
        scan.images = document_relationships
            .iter()
            .filter(|(_, rel)| rel.kind.ends_with("/image"))
            .map(|(id, rel)| {
                let target = if rel.external {
                    rel.target.clone()
                } else {
                    part_target("word/document.xml", &rel.target)
                };
                (id.clone(), target)
            })
            .collect();
        scan.comment_links = links_of(&read_relationships(&mut zip, "word/comments.xml"));
        Some(scan)
    }
}

/// A paragraph or a table of the body, in the order `docx-rs` meets them.
#[derive(Clone, Copy)]
enum RawBlock<'a, 'input> {
    Paragraph(roxmltree::Node<'a, 'input>),
    Table(roxmltree::Node<'a, 'input>),
}

/// Every paragraph and table under `node`, in document order, as `docx-rs`'s body and
/// content-control readers take them: a `w:p` or a `w:tbl` is taken whole, whatever
/// holds it; a `w:sectPr` holds none; anything else, a content control above all, is
/// looked through. Bounded by [`MAX_RUN_NESTING`] on the terms [`RawCount::walk`] is.
fn collect_blocks<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
    out: &mut Vec<RawBlock<'a, 'input>>,
    depth: u32,
) {
    if depth >= MAX_RUN_NESTING {
        return;
    }
    for child in node.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "p" => out.push(RawBlock::Paragraph(child)),
            "tbl" => out.push(RawBlock::Table(child)),
            "sectPr" => {}
            _ => collect_blocks(child, out, depth + 1),
        }
    }
}

/// The elements named `name` under `node`, looked for through any wrapper and not inside
/// one another nor inside a nested table: the rows of a table, the cells of a row, as
/// `docx-rs`'s table and row readers take them.
fn children_named<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
    name: &str,
    out: &mut Vec<roxmltree::Node<'a, 'input>>,
    depth: u32,
) {
    if depth >= MAX_RUN_NESTING {
        return;
    }
    for child in node.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            found if found == name => out.push(child),
            "tbl" | "tblPr" | "tblGrid" | "trPr" | "tcPr" => {}
            _ => children_named(child, name, out, depth + 1),
        }
    }
}

/// The relationships of `part`, by id, from the relationships part beside it. Empty when
/// it has none or they cannot be read: a document without links has nothing to find.
fn read_relationships<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    part: &str,
) -> HashMap<String, Relationship> {
    let Some(name) = rels_part_for(std::path::Path::new(part)) else {
        return HashMap::new();
    };
    let xml = {
        let Ok(mut file) = zip.by_name(&name) else {
            return HashMap::new();
        };
        let mut buffer = Vec::new();
        if std::io::Read::read_to_end(&mut file, &mut buffer).is_err() {
            return HashMap::new();
        }
        String::from_utf8_lossy(&buffer).into_owned()
    };
    let Ok(document) = parse_part(&name, &xml) else {
        return HashMap::new();
    };
    document
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "Relationship")
        .filter_map(|n| {
            Some((
                n.attribute("Id")?.to_string(),
                Relationship {
                    kind: n.attribute("Type").unwrap_or_default().to_string(),
                    target: n.attribute("Target").unwrap_or_default().to_string(),
                    external: n
                        .attribute("TargetMode")
                        .is_some_and(|mode| mode.eq_ignore_ascii_case("External")),
                },
            ))
        })
        .collect()
}

/// Relationship id → address, for the links among `relationships`.
fn links_of(relationships: &HashMap<String, Relationship>) -> HashMap<String, String> {
    relationships
        .iter()
        .filter(|(_, rel)| rel.kind.ends_with("/hyperlink") && !rel.target.trim().is_empty())
        .map(|(id, rel)| (id.clone(), rel.target.clone()))
        .collect()
}

/// The part a relationship of `part` points at: `target` resolved against the folder
/// `part` sits in, or from the package root when it starts with `/`.
fn part_target(part: &str, target: &str) -> String {
    let (mut segments, rest): (Vec<&str>, &str) = match target.strip_prefix('/') {
        Some(absolute) => (Vec::new(), absolute),
        None => (
            part.rsplit_once('/')
                .map_or_else(Vec::new, |(folder, _)| folder.split('/').collect()),
            target,
        ),
    };
    for segment in rest.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            name => segments.push(name),
        }
    }
    segments.join("/")
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

/// Read the notes part `part` for each note's own text: `word/footnotes.xml`, whose
/// notes are `<w:footnote>`, or `word/endnotes.xml`, whose notes are `<w:endnote>`
/// (`element`). Empty when the part is absent, as it is in a document with no notes of
/// that kind, or cannot be read.
///
/// Mirrors [`read_comment_attrs`], and for the same reason: `docx-rs`'s reader
/// does not surface notes at all, neither the part nor the reference that names one, so
/// the only way to either is the raw member.
///
/// Word puts two synthetic notes at the top of every such part, the separator rule and
/// its continuation; both are chrome rather than content and are skipped by their
/// `w:type`. What comes back is styled paragraphs on exactly the terms
/// [`CommentBodyBuilder`] produces for a comment, so a note goes through
/// `rich::assemble`'s single Djot conversion rather than acquiring a second one
/// here: a note is prose the writer wrote, and the four marks
/// [`rich::RunStyle`] carries are the four they could have applied to it.
fn read_note_bodies<R: std::io::Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    part: &str,
    element: &str,
) -> HashMap<usize, Vec<Vec<Run>>> {
    let xml = {
        let Ok(mut file) = zip.by_name(part) else {
            return HashMap::new();
        };
        let mut buffer = Vec::new();
        if std::io::Read::read_to_end(&mut file, &mut buffer).is_err() {
            return HashMap::new();
        }
        String::from_utf8_lossy(&buffer).into_owned()
    };
    let Ok(document) = parse_part(part, &xml) else {
        return HashMap::new();
    };
    let mut out = HashMap::new();
    for node in document
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == element)
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
    out
}

/// The styled paragraphs of one `<w:footnote>` or `<w:endnote>`, from the raw tree.
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
/// reason as [`RawCount::walk`]: a hostile file's nesting is not a manuscript's,
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

/// What an endnote's placeholder label starts with: a prefix of its own, so an endnote
/// and a footnote sharing an id can never share a label. See [`FOOTNOTE_LABEL_PREFIX`].
const ENDNOTE_LABEL_PREFIX: &str = "srcen-";

/// How many characters `run` is in its block's plain text: one for an image or a note's
/// reference (each one `IMAGE_PLACEHOLDER`), its own characters for anything else.
fn counted_length(run: &Run) -> usize {
    if run.image.is_some() || run.footnote.is_some() {
        1
    } else {
        run.text.chars().count()
    }
}

/// Insert a footnote-reference run `within` characters into `runs`, splitting the
/// run that straddles that point.
///
/// The offset is in characters of the runs' plain text ([`counted_length`]), so an
/// image counts as one, and so does a run already carrying a note's reference. An
/// offset at such a reference goes before it: the notes of a paragraph are spliced
/// from the last back ([`Walker::splice_notes`]), so two notes cited at one point
/// arrive in the order they were cited rather than the reverse.
fn insert_footnote_run(runs: &mut Vec<Run>, within: usize, label: &str) {
    let reference = Run {
        footnote: Some(label.to_string()),
        ..Default::default()
    };
    let mut seen = 0usize;
    for index in 0..runs.len() {
        let len = counted_length(&runs[index]);
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

/// Counts a paragraph's or a table's characters exactly as the typed walk does, and notes
/// where each bare `w:commentReference`, `w:footnoteReference` and `w:endnoteReference`
/// sits among them.
///
/// If the two counted differently, every comment and note after the difference would be
/// placed by an offset nobody could reproduce. So each rule below is the typed walk's:
///
/// * text is `w:t`; a tab is one character, and a picture is one ([`holds_picture`]);
/// * an equation's own text (`m:t`) is none: `docx-rs` reads an equation's run for its
///   `w:` children alone, so an equation arrives as nothing, and is counted
///   ([`RawCount::equations`]) to be reported as an object left out, as a formula from an
///   OpenDocument file is. Counted as text, it put every note and point comment after it
///   that many characters late, most often past the end of the paragraph;
/// * a line break or a carriage return ends one stretch of the paragraph and starts the
///   next, since the typed walk makes a block of each ([`Walker::stretches`]); in a table
///   cell it is one character, a space;
/// * nothing inside a tracked deletion (`w:del`, `w:moveFrom`) counts, since that text is
///   dropped; a comment reference there still names where its comment was, but a note
///   whose reference was deleted is not cited;
/// * a field's instruction, formatting (`w:pPr`, whose tab stops are `w:tab` too, and
///   `w:rPr`), a content control's properties, and whatever sits inside a drawing, a
///   legacy shape or an embedded object hold no text the typed walk reads: a text box's
///   words are reported, not read;
/// * of the choices a producer offers for something newer (`mc:AlternateContent`), inside
///   a run only the first counts, since `docx-rs`'s run reader skips `mc:Fallback`; between
///   runs both do, since its paragraph reader looks through both and reads the runs of
///   each.
struct RawCount<'r> {
    ranged: &'r HashSet<usize>,
    /// In a table, where a line break is a space and every mark is in stretch 0.
    in_table: bool,
    stretch: usize,
    offset: usize,
    marks: Vec<RawMark>,
    /// The equations met (`m:oMath`), outside any tracked deletion: each arrives as nothing
    /// and is reported with the embedded objects.
    equations: usize,
}

impl<'r> RawCount<'r> {
    fn new(ranged: &'r HashSet<usize>, in_table: bool) -> Self {
        RawCount {
            ranged,
            in_table,
            stretch: 0,
            offset: 0,
            marks: Vec::new(),
            equations: 0,
        }
    }

    fn walk(&mut self, node: roxmltree::Node<'_, '_>, depth: u32) {
        self.walk_in(node, depth, false, false);
    }

    /// `removed` is whether `node` sits inside a tracked deletion, `in_run` whether it
    /// sits inside a run, where `docx-rs` skips a `mc:Fallback`. A run is any element
    /// named `r`, an equation's `m:r` too: `docx-rs` recognises its elements by their local
    /// name, and reads that one with its run reader.
    fn walk_in(&mut self, node: roxmltree::Node<'_, '_>, depth: u32, removed: bool, in_run: bool) {
        if depth >= MAX_RUN_NESTING {
            return;
        }
        for child in node.children().filter(roxmltree::Node::is_element) {
            let id = || {
                child
                    .attribute((NS_W, "id"))
                    .and_then(|v| v.parse::<usize>().ok())
            };
            match child.tag_name().name() {
                "t" if !removed && !is_math(child) => self.offset += text_of(child).chars().count(),
                "tab" | "ptab" if !removed => self.offset += 1,
                "br" | "cr" if !removed => {
                    if self.in_table {
                        self.offset += 1;
                    } else {
                        self.stretch += 1;
                        self.offset = 0;
                    }
                }
                "drawing" => {
                    if !removed && holds_picture(child) {
                        self.offset += 1;
                    }
                }
                "commentReference" => {
                    if let Some(id) = id()
                        && !self.ranged.contains(&id)
                    {
                        self.mark(RawMarkKind::Comment(id));
                    }
                }
                // **Does not advance the offset.** Word draws a superscript number
                // here, but these offsets index the text the *typed* walk produces,
                // and that walk contributes no character for a reference, so
                // counting one would put every comment after a footnote one
                // character late. The footnote run is spliced in afterwards, and
                // `Walker::shift_offsets` moves what follows it.
                "footnoteReference" if !removed => {
                    if let Some(id) = id() {
                        self.mark(RawMarkKind::Footnote(id));
                    }
                }
                "endnoteReference" if !removed => {
                    if let Some(id) = id() {
                        self.mark(RawMarkKind::Endnote(id));
                    }
                }
                "del" | "moveFrom" => self.walk_in(child, depth + 1, true, in_run),
                "r" => self.walk_in(child, depth + 1, removed, true),
                "oMath" if is_math(child) => {
                    if !removed {
                        self.equations += 1;
                    }
                    self.walk_in(child, depth + 1, removed, in_run);
                }
                "Fallback" if in_run => {}
                "delText" | "instrText" | "delInstrText" | "pPr" | "rPr" | "sdtPr" | "sdtEndPr"
                | "pict" | "object" => {}
                _ => self.walk_in(child, depth + 1, removed, in_run),
            }
        }
    }

    fn mark(&mut self, kind: RawMarkKind) {
        self.marks.push(RawMark {
            kind,
            stretch: self.stretch,
            offset: self.offset,
        });
    }

    /// Count a table as [`Walker::table`] reads it: row by row, a row's cells in order,
    /// then the rows of the tables nested in them; a cell's paragraphs, including those
    /// inside a content control, joined by one character, and one more between cells.
    /// `first_cell` runs across the nested tables, which share the outer one's text.
    fn table(&mut self, table: roxmltree::Node<'_, '_>, first_cell: &mut bool, depth: u32) {
        if depth >= MAX_RUN_NESTING {
            return;
        }
        let mut rows = Vec::new();
        children_named(table, "tr", &mut rows, 0);
        for row in rows {
            let mut cells = Vec::new();
            children_named(row, "tc", &mut cells, 0);
            let mut nested = Vec::new();
            for cell in cells {
                if !*first_cell {
                    self.offset += 1;
                }
                *first_cell = false;
                let mut content = Vec::new();
                collect_blocks(cell, &mut content, 0);
                let mut first_paragraph = true;
                for block in content {
                    match block {
                        RawBlock::Paragraph(paragraph) => {
                            if !first_paragraph {
                                self.offset += 1;
                            }
                            first_paragraph = false;
                            self.walk(paragraph, 0);
                        }
                        RawBlock::Table(inner) => nested.push(inner),
                    }
                }
            }
            for inner in nested {
                self.table(inner, first_cell, depth + 1);
            }
        }
    }
}

/// Whether `node` is Office Math markup (`m:`), in either of the namespaces OOXML spells
/// it with: the transitional one Word writes, or the strict one.
fn is_math(node: roxmltree::Node<'_, '_>) -> bool {
    node.tag_name()
        .namespace()
        .is_some_and(|namespace| namespace.ends_with("/math"))
}

/// Whether a `w:drawing` shows a picture: what `docx-rs` reads as `DrawingData::Pic`, and
/// the typed walk counts as one character. A drawing that is a text box or a shape counts
/// as none.
///
/// Decided as `docx-rs` decides it: the drawing is whichever of a `pic:pic` and a
/// `wps:txbx` comes last, each taken whole, so a picture set *inside* a text box belongs to
/// the box, which the typed walk leaves out. Counting that picture put every note and point
/// comment after the box one character late.
fn holds_picture(drawing: roxmltree::Node<'_, '_>) -> bool {
    fn last_read(node: roxmltree::Node<'_, '_>, last: &mut Option<bool>, depth: u32) {
        if depth >= MAX_RUN_NESTING {
            return;
        }
        for child in node.children().filter(roxmltree::Node::is_element) {
            match child.tag_name().name() {
                "pic" => *last = Some(true),
                "txbx" => *last = Some(false),
                _ => last_read(child, last, depth + 1),
            }
        }
    }
    let mut last = None;
    last_read(drawing, &mut last, 0);
    last == Some(true)
}

/// Parse one part of the raw pass through `skrib_format::xml_depth`, so it is
/// bounded like everything else. [`refuse_unreadable_parts`] has already checked these
/// parts by name, so a refusal cannot reach here from [`DocxScanner::scan`].
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
    /// How many paragraphs of the body have been processed, those inside a content
    /// control included: the key `RawScan` uses.
    paragraph_ordinal: usize,
    /// The same for the body's tables.
    table_ordinal: usize,
    /// The paragraph being walked, one entry per stretch between its line breaks: the
    /// block that stretch produced, or `None` for one that produced nothing. What a raw
    /// mark's `stretch` is looked up in.
    stretches: Vec<Option<usize>>,
    /// The paragraph being walked: its empty stretches and the comments opened on each,
    /// placed once the paragraph is read ([`rich::settle_empty_lines`]).
    empty_lines: Vec<rich::EmptyLine>,
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
    /// [`Walker::splice_notes`]. Not a count of the document's footnotes: the
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
    /// How many annotations there were when this stretch started: the ones after it were
    /// opened in it, and are placed by [`rich::settle_empty_lines`] if it produces none.
    first_annotation: usize,
}

impl ParaBuild {
    /// The rest of the same paragraph after a line break: same kind, same alignment and
    /// direction, and not the start of a page. `first_annotation` is the walk's count at
    /// the break.
    fn continuation(&self, first_annotation: usize) -> ParaBuild {
        ParaBuild {
            kind: self.kind,
            runs: Vec::new(),
            len: 0,
            props: BlockProps {
                page_break_before: false,
                ..self.props
            },
            in_cell: self.in_cell,
            first_annotation,
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
            table_ordinal: 0,
            stretches: Vec::new(),
            empty_lines: Vec::new(),
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
                DocumentChild::Paragraph(paragraph) => self.body_paragraph(paragraph),
                DocumentChild::Table(table) => self.body_table(table),
                DocumentChild::StructuredDataTag(tag) => self.body_control(tag),
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

    /// A content control of the body: its paragraphs and tables are the body's own, in
    /// the order they come, as the raw pass counts them (see [`RawScan`]).
    fn body_control(&mut self, tag: &docx_rs::StructuredDataTag) {
        use docx_rs::StructuredDataTagChild as Child;
        for child in &tag.children {
            match child {
                Child::Paragraph(paragraph) => self.body_paragraph(paragraph),
                Child::Table(table) => self.body_table(table),
                Child::StructuredDataTag(inner) => self.body_control(inner),
                Child::CommentStart(start) => {
                    let block = self.blocks.len();
                    self.open_comment(start.id, block, 0);
                }
                Child::CommentEnd(end) => self.close_comment(end, None),
                Child::BookmarkStart(start) => self.open_mark(start.id, &start.name, 0),
                Child::BookmarkEnd(end) => self.close_mark(end.id, None),
                // A run outside any paragraph is not something Word writes, and the raw
                // pass reads none there either.
                Child::Run(_) => {}
            }
        }
    }

    /// A paragraph of the body, and the ordinal the raw pass knows it by.
    fn body_paragraph(&mut self, paragraph: &Paragraph) {
        let ordinal = self.paragraph_ordinal;
        self.paragraph_ordinal += 1;
        self.top_level_paragraph(paragraph, ordinal);
    }

    /// A table of the body, with the point comments and notes the raw pass found in it.
    fn body_table(&mut self, table: &Table) {
        let ordinal = self.table_ordinal;
        self.table_ordinal += 1;
        let first_block = self.blocks.len();
        let first_annotation = self.annotations.len();
        self.table(table);
        let block = (self.blocks.len() > first_block).then_some(first_block);
        let raw = self.raw;
        let marks = raw.table_marks.get(&ordinal).map_or(&[][..], Vec::as_slice);
        for mark in marks {
            if let RawMarkKind::Comment(id) = mark.kind {
                match block {
                    Some(block) => self.point_comment(id, block, mark.offset, false),
                    None => self.point_comment(id, self.blocks.len(), 0, true),
                }
            }
        }
        self.splice_notes(marks, |_, mark| block.map(|block| (block, mark.offset)));
        if block.is_none() {
            rich::mark_between_blocks(&mut self.annotations, first_annotation, self.blocks.len());
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
                        between_blocks: false,
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
                self.embedded_objects + self.raw.equations,
                ImportDiagnostic::EmbeddedObjectDropped {
                    path: self.origin.clone(),
                    count: self.embedded_objects + self.raw.equations,
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

    /// A paragraph of the body, and so one the supplementary pass keys on by `ordinal`.
    fn top_level_paragraph(&mut self, paragraph: &Paragraph, ordinal: usize) {
        if self.raw.rules.contains(&ordinal) {
            // A horizontal rule. Emitted as its glyph so `skribisto_model` stays the
            // one authority on what a break is — the same treatment the ODT scanner
            // gives ODF's spelling of the same construct. A scene break carries no
            // paragraph formatting, so a pending page break ends here.
            self.page_break_pending = false;
            let block = self.blocks.len();
            // What the rule's paragraph holds besides its border: a comment made on it and
            // the bookmarks in it, all at the rule's own glyph. It holds no text, so a range
            // opened and closed in it covers nothing, and like a comment with no range it
            // becomes a comment on the break line. Left unread, the comment went to the
            // last paragraph of the whole book.
            self.rule_children(&paragraph.children);
            self.blocks.push(RichBlock::body(vec![Run::plain(
                skribisto_model::scene_break::CANONICAL_MINOR,
            )]));
            let raw = self.raw;
            let marks = raw
                .paragraph_marks
                .get(&ordinal)
                .map_or(&[][..], Vec::as_slice);
            for mark in marks {
                match mark.kind {
                    RawMarkKind::Comment(id) => self.point_comment(id, block, 0, false),
                    // A scene break's line holds its glyph alone and cites no note, so the
                    // note is reported as not brought over, never dropped unsaid.
                    RawMarkKind::Footnote(_) | RawMarkKind::Endnote(_) => {
                        self.footnotes_dropped += 1;
                    }
                }
            }
            return;
        }
        self.stretches.clear();
        self.empty_lines.clear();
        self.paragraph(paragraph, ordinal);
        let stretches = std::mem::take(&mut self.stretches);
        let empty_lines = std::mem::take(&mut self.empty_lines);
        rich::settle_empty_lines(
            &mut self.annotations,
            &self.blocks,
            &stretches,
            &empty_lines,
            |index| self.open.contains_key(&index),
        );
        let raw = self.raw;
        let marks = raw
            .paragraph_marks
            .get(&ordinal)
            .map_or(&[][..], Vec::as_slice);

        // A point comment on a line of its own that holds nothing stays in its paragraph,
        // as a range covering nothing there does (see `rich::settle_empty_lines`). Only
        // when the paragraph holds no words at all was it made between blocks.
        for mark in marks {
            if let RawMarkKind::Comment(id) = mark.kind {
                match self.line_position(&stretches, mark.stretch, mark.offset) {
                    Some((block, within)) => self.point_comment(id, block, within, false),
                    None => self.point_comment(id, self.blocks.len(), 0, true),
                }
            }
        }

        // After the comments, and safely so: a note's reference carries no text, and
        // `shift_offsets` moves every offset already placed after it. Doing it here
        // rather than inside the build is what lets the reference find its block across
        // a `<w:br>`, which ends one block and starts the next.
        self.splice_notes(marks, |walker, mark| {
            walker.line_position(&stretches, mark.stretch, mark.offset)
        });
    }

    /// The comment ranges and bookmarks of a paragraph read as a horizontal rule, opened and
    /// closed at the start of the rule's glyph, the block about to be pushed. Nothing else in
    /// it is read: it holds no text ([`is_horizontal_rule`]), and a line break in it must not
    /// make a block of its own.
    fn rule_children(&mut self, children: &[ParagraphChild]) {
        for child in children {
            match child {
                ParagraphChild::Run(run) => self.rule_run(run),
                ParagraphChild::Insert(insert) => {
                    for child in &insert.children {
                        match child {
                            InsertChild::Run(run) => self.rule_run(run),
                            InsertChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), 0)
                            }
                            InsertChild::CommentEnd(end) => self.close_comment(end, Some(0)),
                            InsertChild::Delete(_) => {}
                        }
                    }
                }
                ParagraphChild::Delete(delete) => {
                    for child in &delete.children {
                        match child {
                            DeleteChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), 0)
                            }
                            DeleteChild::CommentEnd(end) => self.close_comment(end, Some(0)),
                            DeleteChild::Run(_) => {}
                        }
                    }
                }
                ParagraphChild::MoveTo(move_to) => {
                    for child in &move_to.children {
                        match child {
                            MoveToChild::Run(run) => self.rule_run(run),
                            MoveToChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), 0)
                            }
                            MoveToChild::CommentEnd(end) => self.close_comment(end, Some(0)),
                            MoveToChild::Delete(_) => {}
                        }
                    }
                }
                ParagraphChild::Hyperlink(hyperlink) => self.rule_children(&hyperlink.children),
                ParagraphChild::StructuredDataTag(tag) => {
                    for child in &tag.children {
                        match child {
                            docx_rs::StructuredDataTagChild::Run(run) => self.rule_run(run),
                            docx_rs::StructuredDataTagChild::CommentStart(start) => {
                                self.open_comment(start.id, self.blocks.len(), 0)
                            }
                            docx_rs::StructuredDataTagChild::CommentEnd(end) => {
                                self.close_comment(end, Some(0))
                            }
                            docx_rs::StructuredDataTagChild::BookmarkStart(start) => {
                                self.open_mark(start.id, &start.name, 0)
                            }
                            docx_rs::StructuredDataTagChild::BookmarkEnd(end) => {
                                self.close_mark(end.id, Some(0))
                            }
                            docx_rs::StructuredDataTagChild::Paragraph(_)
                            | docx_rs::StructuredDataTagChild::Table(_)
                            | docx_rs::StructuredDataTagChild::StructuredDataTag(_) => {}
                        }
                    }
                }
                ParagraphChild::CommentStart(start) => {
                    self.open_comment(start.id, self.blocks.len(), 0)
                }
                ParagraphChild::CommentEnd(end) => self.close_comment(end, Some(0)),
                ParagraphChild::BookmarkStart(start) => self.open_mark(start.id, &start.name, 0),
                ParagraphChild::BookmarkEnd(end) => self.close_mark(end.id, Some(0)),
                ParagraphChild::MoveFrom(_)
                | ParagraphChild::PageNum(_)
                | ParagraphChild::NumPages(_) => {}
            }
        }
    }

    /// The comment ranges inside one run of a horizontal rule. See [`Self::rule_children`].
    fn rule_run(&mut self, run: &DocxRun) {
        for child in &run.children {
            match child {
                RunChild::CommentStart(start) => self.open_comment(start.id, self.blocks.len(), 0),
                RunChild::CommentEnd(end) => self.close_comment(end, Some(0)),
                _ => {}
            }
        }
    }

    /// Where a note's reference or a point comment goes: [`Self::stretch_position`], and
    /// when its stretch produced no block, the nearest line of the same paragraph that did.
    /// The end of the one before it first, since a note most often closes what it follows
    /// and a comment on an empty line most often belongs to the passage before it, and
    /// otherwise the start of the one after it. `None` only when the paragraph produced no
    /// block.
    ///
    /// Either keeps its paragraph silently: a reference alone after a trailing line break,
    /// or before a leading one, cites its note from that paragraph, and a line of its own
    /// holding nothing but the reference is not something the writer made. Dropping the
    /// note, as this did for a while, lost one the same document read from OpenDocument
    /// carried; sending the comment to the paragraph before, as it did for a while too,
    /// moved one made at the top of a page into the page before, and said so.
    ///
    /// The end of the line before is where its words and pictures end
    /// ([`Self::content_end`]), before any reference already spliced in there: the notes
    /// are spliced from the last back, so those are the notes cited after this one, and
    /// measured past them this note, cited first, would land after them.
    fn line_position(
        &self,
        stretches: &[Option<usize>],
        stretch: usize,
        offset: usize,
    ) -> Option<(usize, usize)> {
        if let Ok(position) = self.stretch_position(stretches, stretch, offset) {
            return Some(position);
        }
        let split = stretch.min(stretches.len());
        let before = stretches[..split].iter().rev().flatten().next();
        let after = stretches[split..].iter().flatten().next();
        match (before, after) {
            (Some(block), _) => Some((*block, self.content_end(*block))),
            (None, Some(block)) => Some((*block, 0)),
            (None, None) => None,
        }
    }

    /// Where `block`'s own words and pictures end, in the characters
    /// [`insert_footnote_run`] counts: its whole plain text, less the note references
    /// that follow its last word or picture.
    fn content_end(&self, block: usize) -> usize {
        match self.blocks.get(block) {
            Some(RichBlock::Paragraph { runs, .. }) => runs
                .iter()
                .rposition(|run| {
                    run.footnote.is_none() && (run.image.is_some() || !run.text.is_empty())
                })
                .map_or(0, |last| runs[..=last].iter().map(counted_length).sum()),
            Some(table) => table.plain_text().chars().count(),
            None => 0,
        }
    }

    /// Where the character `offset` into the paragraph's stretch `stretch` is:
    /// `Ok((block, offset within it))`, or `Err(next block)` when that stretch produced
    /// no block, the index of the block that follows it.
    ///
    /// A stretch the typed walk did not see (the raw pass counted a line break it did not
    /// make) is read as the end of the paragraph's last block: a note at the end of the
    /// paragraph it belongs to is much closer to right than no note at all. An offset past
    /// the end of its block is clamped to that end, for the same reason.
    fn stretch_position(
        &self,
        stretches: &[Option<usize>],
        stretch: usize,
        offset: usize,
    ) -> Result<(usize, usize), usize> {
        let length = |block: usize| {
            self.blocks
                .get(block)
                .map_or(0, |b| b.plain_text().chars().count())
        };
        match stretches.get(stretch) {
            Some(Some(block)) => Ok((*block, offset.min(length(*block)))),
            Some(None) => Err(stretches
                .get(stretch + 1..)
                .into_iter()
                .flatten()
                .flatten()
                .next()
                .copied()
                .unwrap_or(self.blocks.len())),
            None => match stretches.iter().rev().flatten().next() {
                Some(block) => Ok((*block, length(*block))),
                None => Err(self.blocks.len()),
            },
        }
    }

    /// Put the footnote and endnote references among `marks` back into the runs they sat
    /// between, at the position `locate` finds for each, or count them as not carried.
    ///
    /// A footnote is labelled `srcfn-{id}` and an endnote `srcen-{id}`, each with the body
    /// its own part defines: Word numbers the two kinds separately, both from 1, so an
    /// endnote looked up among the footnotes cited footnote 1's text instead of its own.
    /// Both arrive as footnotes, the one kind of note a project holds, as an endnote from
    /// an OpenDocument file does.
    ///
    /// Splicing back-to-front matters: an earlier insertion would shift the runs a
    /// later offset is measured against, and two notes in one paragraph is ordinary
    /// (a sentence citing two sources). Iterating in reverse means every offset is
    /// still measured against the runs it was measured against when it was recorded.
    fn splice_notes(
        &mut self,
        marks: &[RawMark],
        locate: impl Fn(&Self, &RawMark) -> Option<(usize, usize)>,
    ) {
        let raw = self.raw;
        for mark in marks.iter().rev() {
            let (bodies, prefix, id) = match mark.kind {
                RawMarkKind::Footnote(id) => (&raw.footnote_bodies, FOOTNOTE_LABEL_PREFIX, id),
                RawMarkKind::Endnote(id) => (&raw.endnote_bodies, ENDNOTE_LABEL_PREFIX, id),
                RawMarkKind::Comment(_) => continue,
            };
            let Some(paragraphs) = bodies.get(&id) else {
                // A reference naming a note its part does not define. Carrying the
                // marker with nothing behind it would put a citation in the book
                // pointing at an empty note.
                self.footnotes_dropped += 1;
                continue;
            };
            let label = format!("{prefix}{id}");
            let Some((block, within)) = locate(self, mark) else {
                // Its paragraph or table produced no block to hold the reference.
                self.footnotes_dropped += 1;
                continue;
            };
            if !self.insert_note(block, within, &label) {
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

    /// Insert one note's reference run `within` characters into `block`, a paragraph or
    /// a table, whose plain text runs through its cells joined by one character each.
    /// Returns whether there was such a block.
    fn insert_note(&mut self, block: usize, within: usize, label: &str) -> bool {
        match self.blocks.get_mut(block) {
            Some(RichBlock::Paragraph { runs, .. }) => insert_footnote_run(runs, within, label),
            Some(RichBlock::Table { rows }) => {
                let mut cell_start = 0usize;
                let mut cells = rows.iter_mut().flatten().peekable();
                while let Some(cell) = cells.next() {
                    let length: usize = cell.iter().map(counted_length).sum();
                    if within <= cell_start + length || cells.peek().is_none() {
                        insert_footnote_run(cell, within.saturating_sub(cell_start), label);
                        break;
                    }
                    cell_start += length + 1;
                }
            }
            None => return false,
        }
        self.shift_offsets(block, within);
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

    /// A comment anchored to a point rather than a range: it belongs to the paragraph,
    /// or the table cell, its reference sat in, which a zero length says downstream.
    ///
    /// `between` says the reference sat where the file holds no text, a paragraph or a
    /// table holding no words, so `block` is the block after it (see
    /// [`RichAnnotation::between_blocks`]). Such a comment goes on the paragraph before
    /// it and is reported; it used to land on the last paragraph of the whole document.
    fn point_comment(&mut self, id: usize, block: usize, offset: usize, between: bool) {
        let index = self.annotations.len();
        self.open_comment(id, block, offset);
        if self.annotations.len() > index {
            // No range end names a point comment, so it is never left open.
            self.open.remove(&index);
            self.annotations[index].between_blocks = between;
        }
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
    /// chain resolved. `ordinal` is its position among the body's paragraphs, the key of
    /// the raw pass's own reading of its `w:pPr`.
    fn paragraph_props(&self, property: &ParagraphProperty, ordinal: usize) -> BlockProps {
        let own = self
            .raw
            .paragraph_props
            .get(&ordinal)
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

    fn paragraph(&mut self, paragraph: &Paragraph, ordinal: usize) {
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
            first_annotation: self.annotations.len(),
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
                    let url = link_target(&hyperlink.link, &self.raw.links);
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
                    let next = build.continuation(self.annotations.len());
                    let finished = std::mem::replace(build, next);
                    self.flush(finished);
                    if *kind == Break::new(BreakType::Page) {
                        self.page_break_pending = true;
                    }
                }
                RunChild::CarriageReturn(_) => {
                    let next = build.continuation(self.annotations.len());
                    let finished = std::mem::replace(build, next);
                    self.flush(finished);
                }
                RunChild::Drawing(drawing) => match &drawing.data {
                    Some(DrawingData::Pic(pic)) => {
                        // The picture's part (`word/media/image1.png`), not the
                        // relationship id `docx-rs` gives it, which names nothing
                        // outside this one file's relationships.
                        let source = self
                            .raw
                            .images
                            .get(&pic.id)
                            .cloned()
                            .unwrap_or_else(|| pic.id.clone());
                        self.diagnostics.push(ImportDiagnostic::ImageNotIngested {
                            path: self.origin.clone(),
                            target: source.clone(),
                        });
                        build.len += 1;
                        build.runs.push(Run::sized_image(
                            "",
                            source,
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
                // `RawScan::paragraph_marks` and `read_note_bodies`. Kept as an explicit
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
                // A tracked deletion's text, an instruction's source, a symbol naming no
                // character (every other one arrives as text, `with_characters_as_text`):
                // none of them are prose.
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
    ///
    /// A cell's paragraphs are all of them, those inside a content control included. A
    /// table nested in a cell cannot be one inside a Djot table, so its rows follow the
    /// row holding it, as the ODT scanner reads the same table: its words are kept, and
    /// they used to be dropped without a word.
    fn table(&mut self, table: &Table) {
        let mut build = ParaBuild {
            kind: ParagraphKind::Body,
            runs: Vec::new(),
            len: 0,
            props: BlockProps::default(),
            in_cell: true,
            first_annotation: self.annotations.len(),
        };
        let mut rows: Vec<Vec<Vec<Run>>> = Vec::new();
        let mut first_cell = true;
        self.table_rows(table, &mut build, &mut rows, &mut first_cell);
        if !rows.is_empty() {
            // A table carries no paragraph formatting of its own, so a page break before
            // it is not carried either.
            self.page_break_pending = false;
            self.blocks.push(RichBlock::Table { rows });
        }
    }

    /// The rows of `table` into `rows`, each followed by the rows of the tables nested in
    /// its cells. `first_cell` runs across all of them, since they share one text.
    fn table_rows<'t>(
        &mut self,
        table: &'t Table,
        build: &mut ParaBuild,
        rows: &mut Vec<Vec<Vec<Run>>>,
        first_cell: &mut bool,
    ) {
        for TableChild::TableRow(row) in &table.rows {
            let mut cells: Vec<Vec<Run>> = Vec::new();
            let mut nested: Vec<&'t Table> = Vec::new();
            for TableRowChild::TableCell(cell) in &row.cells {
                if !*first_cell {
                    build.len += 1;
                }
                *first_cell = false;
                let start = build.runs.len();
                let mut first_paragraph = true;
                for content in &cell.children {
                    match content {
                        TableCellContent::Paragraph(paragraph) => {
                            self.cell_paragraph(paragraph, build, &mut first_paragraph);
                        }
                        TableCellContent::Table(inner) => nested.push(inner),
                        TableCellContent::StructuredDataTag(tag) => {
                            self.cell_control(tag, build, &mut nested, &mut first_paragraph);
                        }
                        // `docx-rs`'s reader never makes one: a table of contents arrives
                        // as the content control holding it.
                        TableCellContent::TableOfContents(_) => {}
                    }
                }
                cells.push(build.runs.split_off(start));
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
            for inner in nested {
                self.table_rows(inner, build, rows, first_cell);
            }
        }
    }

    /// One paragraph of a cell, a space before it when it is not the cell's first.
    fn cell_paragraph(
        &mut self,
        paragraph: &Paragraph,
        build: &mut ParaBuild,
        first_paragraph: &mut bool,
    ) {
        if !*first_paragraph {
            build.len += 1;
            build.runs.push(Run::plain(" "));
        }
        *first_paragraph = false;
        self.paragraph_children(&paragraph.children, &paragraph.property, build, None);
    }

    /// A content control inside a cell: its paragraphs are the cell's, a table in it is
    /// nested in the cell, and its comment and bookmark ends are where they sit in the
    /// cell's text.
    fn cell_control<'t>(
        &mut self,
        tag: &'t docx_rs::StructuredDataTag,
        build: &mut ParaBuild,
        nested: &mut Vec<&'t Table>,
        first_paragraph: &mut bool,
    ) {
        use docx_rs::StructuredDataTagChild as Child;
        for child in &tag.children {
            match child {
                Child::Paragraph(paragraph) => {
                    self.cell_paragraph(paragraph, build, first_paragraph)
                }
                Child::Table(inner) => nested.push(inner),
                Child::StructuredDataTag(inner) => {
                    self.cell_control(inner, build, nested, first_paragraph)
                }
                Child::CommentStart(start) => {
                    self.open_comment(start.id, self.blocks.len(), build.len)
                }
                Child::CommentEnd(end) => self.close_comment(end, Some(build.len)),
                Child::BookmarkStart(start) => self.open_mark(start.id, &start.name, build.len),
                Child::BookmarkEnd(end) => self.close_mark(end.id, Some(build.len)),
                // A run outside any paragraph is not something Word writes, and the raw
                // pass reads none there either.
                Child::Run(_) => {}
            }
        }
    }

    /// Finish one stretch of a paragraph: the whole of it, or what came before a line
    /// break. A stretch that holds nothing produces no block, and where the comments opened
    /// in it go is decided once the whole paragraph is read, since that depends on whether
    /// any other line of it holds words ([`rich::settle_empty_lines`]).
    fn flush(&mut self, build: ParaBuild) {
        if build.runs.is_empty() {
            self.empty_lines.push(rich::EmptyLine {
                line: self.stretches.len(),
                annotations: build.first_annotation..self.annotations.len(),
            });
            self.stretches.push(None);
            return;
        }
        let mut props = build.props;
        props.page_break_before |= std::mem::take(&mut self.page_break_pending);
        self.stretches.push(Some(self.blocks.len()));
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
            between_blocks: false,
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
            let annotation = &mut self.annotations[open.annotation];
            annotation.length = offset.saturating_sub(open.start);
            rich::reaches_words(annotation);
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
        rich::reaches_words(annotation);
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

    /// A picture size within 4,762 EMU of `u32::MAX` used to overflow the `+ 4762`
    /// that rounds to the nearest pixel: a panic in debug, a wrong size in release.
    /// It saturates now, so the largest a `.docx` can claim comes back as a bounded
    /// pixel count rather than aborting the scan.
    #[test]
    fn emu_to_pixels_saturates_rather_than_overflowing() {
        assert_eq!(emu_to_pixels(0), 0);
        assert_eq!(emu_to_pixels(9_525), 1);
        assert_eq!(emu_to_pixels(9_525 * 96), 96);
        // Within the rounding constant of the ceiling: the old code panicked here.
        assert_eq!(emu_to_pixels(u32::MAX), u32::MAX / 9_525);
        assert_eq!(emu_to_pixels(u32::MAX - 1), u32::MAX / 9_525);
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

    /// Each hyphen element becomes its character in a text element of the same prefix, a
    /// removed one in a `delText` inside a tracked deletion, and nothing else of the part
    /// changes. A part holding none of the elements is left alone.
    #[test]
    fn a_hyphen_element_is_written_as_its_character() {
        let xml = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="w"><!-- softHyphen -->"#,
            r#"<w:p><w:r><w:t xml:space="preserve">twenty &amp; </w:t><w:noBreakHyphen/><w:t>one</w:t></w:r>"#,
            r#"<w:del w:id="1"><w:r><w:softHyphen></w:softHyphen></w:r></w:del>"#,
            r#"<x:r xmlns:x="w"><x:softHyphen/></x:r></w:p></w:document>"#,
        );
        let rewritten = with_characters_as_text(xml.as_bytes()).expect("rewritten");
        assert_eq!(
            String::from_utf8(rewritten).expect("UTF-8"),
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="w"><!-- softHyphen -->"#,
                r#"<w:p><w:r><w:t xml:space="preserve">twenty &amp; </w:t><w:t>&#x2011;</w:t><w:t>one</w:t></w:r>"#,
                r#"<w:del w:id="1"><w:r><w:delText>&#xAD;</w:delText></w:r></w:del>"#,
                r#"<x:r xmlns:x="w"><x:t>&#xAD;</x:t></x:r></w:p></w:document>"#,
            )
        );
        assert_eq!(
            with_characters_as_text(
                b"<w:document><w:p><w:r><w:t>plain symphony: sym</w:t></w:r></w:p></w:document>"
            ),
            None
        );
    }

    /// A symbol becomes the character it shows, in a text element like a hyphen's: a
    /// Symbol-font code the letter that font draws for it, any other font's code the
    /// character it names. One naming no character a part can hold is left as it was.
    #[test]
    fn a_symbol_is_written_as_its_character() {
        let xml = concat!(
            r#"<w:document xmlns:w="w"><w:p><w:r><w:rPr><w:rFonts w:ascii="Symbol"/></w:rPr>"#,
            r#"<w:sym w:font="Symbol" w:char="F061"/><w:sym w:font="Wingdings" w:char="F0E0"/>"#,
            r#"<w:sym w:font="Symbol" w:char="0070"/><w:sym w:font="Symbol" w:char="F060"/>"#,
            r#"<w:sym w:font="Times New Roman" w:char="00E9"/><w:sym w:font="Symbol" w:char="0007"/>"#,
            r#"<w:sym w:font="Symbol" w:char="nothing"/></w:r>"#,
            r#"<w:del w:id="1"><w:r><w:sym w:char="F0AE" w:font="Symbol"></w:sym></w:r></w:del>"#,
            r#"</w:p></w:document>"#,
        );
        let rewritten = with_characters_as_text(xml.as_bytes()).expect("rewritten");
        assert_eq!(
            String::from_utf8(rewritten).expect("UTF-8"),
            concat!(
                r#"<w:document xmlns:w="w"><w:p><w:r><w:rPr><w:rFonts w:ascii="Symbol"/></w:rPr>"#,
                r#"<w:t>&#x3B1;</w:t><w:t>&#xF0E0;</w:t>"#,
                r#"<w:t>&#x3C0;</w:t><w:t>&#xF060;</w:t>"#,
                r#"<w:t>&#xE9;</w:t><w:sym w:font="Symbol" w:char="0007"/>"#,
                r#"<w:sym w:font="Symbol" w:char="nothing"/></w:r>"#,
                r#"<w:del w:id="1"><w:r><w:delText>&#x2192;</w:delText></w:r></w:del>"#,
                r#"</w:p></w:document>"#,
            )
        );
    }

    /// The Symbol font's letters are Greek at the codes of the Latin ones, and its
    /// upper half holds arrows and mathematical signs.
    #[test]
    fn the_symbol_font_shows_greek_letters_arrows_and_signs() {
        let shown: String = [b'a', b'b', b'g', b'W', b'D', 0xAE, 0xB3, 0xD6, 0xA5, 0xB4]
            .into_iter()
            .filter_map(symbol_font_character)
            .collect();
        assert_eq!(shown, "αβγΩΔ→≥√∞×");
        assert_eq!(symbol_font_character(0x1F), None);
        assert_eq!(symbol_font_character(0x80), None);
        assert_eq!(symbol_font_character(0x60), None, "the radical's extension");
        assert_eq!(symbol_font_character(0xFF), None);
    }

    /// A relationship's target is resolved against the folder of the part making it.
    #[test]
    fn a_relationship_target_is_found_from_its_part() {
        assert_eq!(
            part_target("word/document.xml", "media/image1.png"),
            "word/media/image1.png"
        );
        assert_eq!(
            part_target("word/document.xml", "../media/image1.png"),
            "media/image1.png"
        );
        assert_eq!(
            part_target("word/document.xml", "/word/media/./image1.png"),
            "word/media/image1.png"
        );
    }

    /// The name of a part's relationships is `docx-rs`'s own, quirk included: asked
    /// for the relationships of each main part below, `read_document_rels` finds the
    /// member [`rels_part_for`] names and reads the relationship in it.
    #[test]
    fn a_parts_relationships_are_found_where_docx_rs_looks_for_them() {
        use std::io::Write;
        for main in [
            "word/document.xml",
            "word/main.v1.xml",
            "document.xml",
            "a/b/c.xml",
        ] {
            let rels_name = rels_part_for(std::path::Path::new(main)).expect("a name");
            let mut out = std::io::Cursor::new(Vec::new());
            {
                let mut writer = zip::ZipWriter::new(&mut out);
                let options = zip::write::SimpleFileOptions::default();
                writer
                    .start_file(rels_name.as_str(), options)
                    .expect("member");
                writer
                    .write_all(
                        format!(
                            "<Relationships><Relationship Id=\"rId1\" Type=\"{STYLES}\" \
                             Target=\"styles.xml\"/></Relationships>"
                        )
                        .as_bytes(),
                    )
                    .expect("member bytes");
                writer.finish().expect("finish");
            }
            let bytes = out.into_inner();
            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes[..])).expect("zip");
            let rels = docx_rs::read_document_rels(&mut archive, main)
                .unwrap_or_else(|e| panic!("{main}: docx-rs reads {rels_name}: {e:?}"));
            assert!(
                rels.find_target_path(STYLES).is_some(),
                "{main}: the relationship in {rels_name} is read"
            );
        }
        assert_eq!(
            rels_part_for(std::path::Path::new("word/main.v1.xml")).as_deref(),
            Some("word/_rels/main.xml.rels")
        );
    }

    /// Each way a part can keep `docx-rs` reading for ever, measured on the part alone,
    /// named with where it was found. A lenient part (read by a loop that stops at the
    /// end of its input) is refused only for its depth, as before.
    #[test]
    fn a_part_docx_rs_would_never_finish_is_named_with_where() {
        let unfinished = |xml: &str, demand: &Demand| {
            check_as_docx_rs_reads("word/document.xml", xml.as_bytes(), demand)
                .err()
                .map(|e| e.to_string())
        };
        let read = Demand::to_its_end();

        let cut = unfinished("<w:document>\n<w:body>\n<w:p>", &read).unwrap_or_default();
        assert!(cut.contains("cut short at line 3"), "{cut}");
        assert!(cut.contains("3 elements still open"), "{cut}");
        let one = unfinished("<w:document>", &read).unwrap_or_default();
        assert!(one.contains("an element still open"), "{one}");

        let damaged =
            unfinished("<w:document>\n<w:p></w:x></w:document>", &read).unwrap_or_default();
        assert!(damaged.contains("damaged at line 2"), "{damaged}");

        let syntax = unfinished("<w:document>\n\n<!- ></w:document>", &read).unwrap_or_default();
        assert!(syntax.contains("damaged at line 3"), "{syntax}");

        let rooted = Demand::root(STYLES_ROOT);
        let other = unfinished("<w:docDefaults/>", &rooted).unwrap_or_default();
        assert!(
            other.contains("<styles>") && other.contains("<docDefaults>"),
            "{other}"
        );
        let none = unfinished("", &rooted).unwrap_or_default();
        assert!(none.contains("holds none"), "{none}");

        // Whole, each is read.
        for (xml, demand) in [
            (
                "<w:document>\n<w:body>\n<w:p/></w:body></w:document>",
                &read,
            ),
            ("<w:styles><w:style/></w:styles>", &rooted),
            ("", &read),
        ] {
            assert_eq!(unfinished(xml, demand), None, "{xml:?}");
        }
        // Lenient, the damage is left to the reader that copes with it.
        assert_eq!(unfinished("<Types><Default", &Demand::default()), None);
    }

    /// A part genuinely written in UTF-16 is, to a parser reading it byte by byte, one
    /// start tag after another, every name holding NUL bytes: never closed, and never
    /// recognised by `docx-rs` either. It is not taken for a part cut short.
    #[test]
    fn a_part_in_utf16_is_not_taken_for_one_cut_short() {
        let text = "<?xml version=\"1.0\" encoding=\"UTF-16\"?><w:document><w:body><w:p><w:r>\
                    <w:t>Words.</w:t></w:r></w:p></w:body></w:document>";
        let mut utf16 = vec![0xFF, 0xFE];
        for unit in text.encode_utf16() {
            utf16.extend_from_slice(&unit.to_le_bytes());
        }
        let checked = check_as_docx_rs_reads("word/document.xml", &utf16, &Demand::to_its_end());
        assert!(checked.is_ok(), "{checked:?}");
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
            let refused = check_as_docx_rs_reads("part", &xml, &Demand::default())
                .err()
                .and_then(|e| skrib_format::xml_depth::too_deep(&e).cloned());
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
