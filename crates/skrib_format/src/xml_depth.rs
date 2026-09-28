// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A ceiling on how deeply nested the XML an importer reads may be, and a stack
//! deep enough to parse anything under it.
//!
//! # The failure this prevents
//!
//! `roxmltree` 0.21 descends once per element (`parse_element` calls
//! `parse_content`, which calls `parse_element` for the child) and has no depth
//! limit of its own. `docx-rs` 0.4.22 descends once per level of a table nested in
//! a table cell. Neither can be stopped from outside once it has started, and a
//! stack overflow is not a panic: **it aborts the process**. `catch_unwind` never
//! sees it, so a long operation's crash guard is no help, and every window of this
//! single-instance process goes with it, unsaved prose included.
//!
//! Measured on the 2 MiB stack a long operation's thread gets (`thread::spawn`'s
//! default), in a debug build: `roxmltree` parses 129 nested elements and aborts at
//! 130; `docx-rs` reads 34 tables nested in cells (102 elements) and aborts at 35.
//! A Manuskript `world.opml` of a few kilobytes was enough to take the app down.
//!
//! # Two defences, and why both
//!
//! 1. [`check`](crate::xml_depth::check) reads the bytes before any parser does and refuses a document nested
//!    past [`MAX_DEPTH`](crate::xml_depth::MAX_DEPTH) with a typed [`XmlTooDeep`](crate::xml_depth::XmlTooDeep). The scan is a loop, never a
//!    recursion, so it cannot overflow on the input it exists to refuse.
//! 2. [`parse`](crate::xml_depth::parse), [`on_parser_stack`](crate::xml_depth::on_parser_stack) and [`on_parser_stack_reporting`](crate::xml_depth::on_parser_stack_reporting) run the
//!    parser, and the walks that follow it, on a thread of their own with
//!    [`PARSER_STACK_BYTES`](crate::xml_depth::PARSER_STACK_BYTES) of stack. Without it the ceiling would have to sit
//!    below what the *calling* thread has left, which at 129 elements minus the
//!    caller's own frames is uncomfortably close to real documents.
//!
//! # The limit
//!
//! [`MAX_DEPTH`](crate::xml_depth::MAX_DEPTH) is 256 nested elements, the ceiling libxml2 applies by default
//! (`xmlParserMaxDepth`; libxml2 2.9.14 reads 257 and refuses 258). For scale, the
//! deepest file among 288 real office documents measured for this (`.odt`, `.docx`,
//! `.pptx`, `.xlsx`) nests 23 elements, no test fixture in this workspace passes 9,
//! and the deepest real Scrivener manifest seen nests 49.
//!
//! [`PARSER_STACK_BYTES`](crate::xml_depth::PARSER_STACK_BYTES) is sized from the same measurement, taken with the
//! hostile-fixture tests each importer carries, in a debug build: at the ceiling
//! `roxmltree` alone needs about 3.9 MB, and the heaviest whole import (a `.docx`
//! scan, `docx-rs` recursing through tables nested in cells) overflows 4 MiB and
//! completes in 5. 32 MiB is over six times the most any of them needs. The stack
//! is address space reserved for one import, not memory committed; only the pages
//! it touches are used.
//!
//! # What the scan counts
//!
//! It follows `roxmltree`'s own tokenizer: comments, CDATA sections, processing
//! instructions and the DOCTYPE are skipped whole, attribute values are skipped
//! quote to quote (so a `>` inside one does not end the tag), and every start tag
//! counts at its own depth, the root being one. A start tag opens a level that its
//! end tag closes; `<a/>` counts where it sits and opens nothing under it, which is
//! also how libxml2 counts toward its own ceiling. Wherever the two could disagree
//! about the document's own markup, `roxmltree` refuses the document at that point
//! and stops, so its own tags never take it deeper than the scan measured.
//!
//! Entities are the other way to nest a document, and are counted too. With a DTD
//! allowed, `roxmltree` expands `&name;` by parsing the declared value as content,
//! in place, at the depth of the reference, so a value holding markup nests the
//! document further than its own tags say. The scan measures each declared value
//! the same way, follows references between values as far as `roxmltree` does
//! (ten), and treats anything further as unbounded. A reference finds its name in
//! a map, so the scan stays linear however many entities a DTD declares.
//! [`parse`](crate::xml_depth::parse) refuses a document declaring an entity
//! before the scan runs (see the next section), but
//! [`check`](crate::xml_depth::check) is also called on its own, ahead of parsers
//! this module does not run, so it still measures every one.
//!
//! A value need not be balanced, either. `roxmltree` keeps building the tree where
//! a value left it, so `<!ENTITY o '<outline>'>` referenced a thousand times opens
//! a thousand levels, one after the other, and a matching entity holding
//! `</outline>` closes them again: no single expansion is deep, and the tree is.
//! So each value is measured for where it leaves the depth as well as how deep it
//! reaches, and the scan carries that on after the reference exactly as the tree
//! does. What passes the scan is therefore a tree no deeper than
//! [`MAX_DEPTH`](crate::xml_depth::MAX_DEPTH), which is what lets the recursive
//! walks after a parse treat that ceiling as a guarantee.
//!
//! # Entities a document declares
//!
//! Depth is not the only thing an entity multiplies. `roxmltree` bounds the
//! references made *inside* entity values (ten levels, 255 references under each
//! one the document makes: its billion-laughs guard) but not the references the
//! document itself makes, which it allows without limit. One 64 KiB entity named
//! 4,096 times turns a 78 KB file into a 256 MiB string, and a member within an
//! importer's size limits (256 to 512 MiB) into more memory than the computer
//! has. Running out of memory aborts the process, every window with it.
//!
//! So a reader that allows a DTD refuses any document declaring an entity, before
//! a parser sees it: [`parse`](crate::xml_depth::parse) calls
//! [`check_entities`](crate::xml_depth::check_entities) first, and a caller that
//! checks a whole project up front calls it too. No reader here needs one. Plume
//! Creator writes a bare `<!DOCTYPE plume-tree>` and nothing inside it (the 1,130
//! members of sixteen real `.plume` files, backups included, declare none),
//! Manuskript writes no DOCTYPE at all, and the five predefined entities (`&lt;`,
//! `&amp;`, …) need no declaration. A DOCTYPE alone is still read.
//!
//! The rule is read off the text, and deliberately wider than the grammar: any
//! `<!ENTITY` after the first `<!DOCTYPE` refuses the document, in the internal
//! subset or not. XML spells both keywords in capitals only and `roxmltree`
//! declares nothing without them, so what it could expand is always refused. A
//! narrower rule would have to find where the internal subset ends exactly as
//! `roxmltree` does, quoted literals and comments included, and a single
//! disagreement there would reopen the hole. The cost is a document holding
//! `<!ENTITY` in a comment or a CDATA section after a DOCTYPE, which neither
//! Plume nor Manuskript ever writes.
//!
//! # Folders
//!
//! A tree need not be XML to be walked recursively. A Manuskript project keeps its
//! outline as folders, one per level, and the importer recurses once per level of
//! it just as it would once per element. A folder path inside a zip is text, up to
//! 64 KiB of it, with no parser in front of the recursion. So the same ceiling
//! applies, and [`FoldersTooDeep`](crate::xml_depth::FoldersTooDeep) is its
//! refusal, travelling to the writer the same way.
//!
//! # Every platform, not only the one the tests ran on
//!
//! A frame is not the same size on every platform. Most of a debug build's frames
//! measured within about 10 % of each other between x86-64 Linux and arm64 macOS,
//! but a frame holding the standard library's `DirEntry` did not: on macOS it
//! carries the whole `dirent` record, a 1 KiB name buffer included, and the
//! Manuskript folder walk, recursive at the time, needed 8.7 KiB a level there
//! against 1.7 KiB on Linux. A folder project nested to the ceiling passed every
//! test on Linux and overflowed a long operation's stack on macOS. So nothing that
//! grows with an input's nesting runs on the calling thread's stack: a walk is a
//! loop, or it runs on the parser stack.
//!
//! The hostile-input tests hold every importer to that with less than a fifth of a
//! long operation's stack: an import or a scan runs on 384 KiB. What one needs
//! whatever the file, about 190 KiB in a debug build, is mostly fixed-size state
//! (the zip writer's deflate tables) that costs the same everywhere, and the rest
//! cannot hold 256 levels of anything costing more than 0.75 KiB a level. A pass on
//! Linux therefore still holds where frames are five times larger. A Djot parse is
//! the exception: `jotdown` recurses by design and the Djot ceiling is what bounds
//! it, at about 350 KiB in a debug build, so those tests run on a quarter of the
//! real stack instead.
//!
//! One recursion stays out of reach: `text-document` reads Markdown and HTML on a
//! thread of its own, with the 2 MiB `std::thread::spawn` gives it, and nothing
//! here can size that thread. At the ceilings its HTML reader needs about 0.55 MiB
//! on Linux, and its frames measured 5 % larger on macOS.

use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;

/// The most nested elements an XML document may reach, the root counting as one.
pub const MAX_DEPTH: usize = 256;

/// The stack every XML parse runs on; see the module note for the measurement.
pub const PARSER_STACK_BYTES: usize = 32 * 1024 * 1024;

/// How many entity references `roxmltree` follows inside one another before it
/// refuses the document (`LoopDetector` in its `parse.rs`).
const ENTITY_NESTING: u8 = 10;

/// The five names XML predefines. `roxmltree` turns them into characters before it
/// ever looks at a declared entity, so a DTD redeclaring one changes nothing.
const PREDEFINED: [&[u8]; 5] = [b"lt", b"gt", b"amp", b"apos", b"quot"];

/// What a refused document's failure message starts with when it has to cross a
/// boundary that only carries text; see [`XmlTooDeep::failure_message`].
const FAILURE_TAG: &str = "xml-nested-too-deep:";

/// Why an XML document was refused before it was parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlTooDeep {
    /// The file, or the member of a container, that was refused: `content.xml`,
    /// `word/document.xml`, `world.opml`, `tree`.
    pub part: String,
    /// How deep the scan had reached when it stopped. Always past [`MAX_DEPTH`];
    /// when an entity's own expansion is what goes past it, this is the first
    /// depth known to be too deep rather than the full depth of the expansion.
    pub depth: usize,
    /// The 1-based line it stopped on.
    pub line: usize,
}

impl XmlTooDeep {
    /// The ceiling this document went past.
    pub fn limit(&self) -> usize {
        MAX_DEPTH
    }

    /// This refusal as one line of text a reader can turn back into the value.
    ///
    /// A long operation reports its failure as a string (`OperationStatus::Failed`),
    /// so a typed error cannot reach the UI through it as a type. This is the typed
    /// value spelled so that [`Self::from_failure_message`] recovers all of it: the
    /// two numbers first, the part last, so a part name holding a colon still reads
    /// back whole.
    pub fn failure_message(&self) -> String {
        format!("{FAILURE_TAG}{}:{}:{}", self.depth, self.line, self.part)
    }

    /// Recover a refusal from a failure message, or `None` when the message is
    /// about something else.
    pub fn from_failure_message(message: &str) -> Option<Self> {
        let rest = message.strip_prefix(FAILURE_TAG)?;
        let (depth, rest) = rest.split_once(':')?;
        let (line, part) = rest.split_once(':')?;
        Some(Self {
            part: part.to_string(),
            depth: depth.parse().ok()?,
            line: line.parse().ok()?,
        })
    }
}

impl fmt::Display for XmlTooDeep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} nests its XML elements {} levels deep at line {}, and the limit is \
             {MAX_DEPTH}. A file nested this deeply would crash the XML parser, so it \
             is refused unread",
            self.part, self.depth, self.line
        )
    }
}

impl std::error::Error for XmlTooDeep {}

/// What a refused document's failure message starts with when it declared an
/// entity; see [`XmlDeclaresEntities::failure_message`].
const ENTITIES_FAILURE_TAG: &str = "xml-declares-entities:";

/// Why an XML document was refused before it was parsed: it declares an entity of
/// its own, which a parser allowing a DTD would expand as often as the document
/// names it. See the module note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlDeclaresEntities {
    /// The file, or the member of a container, that was refused: `tree`,
    /// `world.opml`.
    pub part: String,
    /// The 1-based line of the first declaration.
    pub line: usize,
}

impl XmlDeclaresEntities {
    /// This refusal as one line of text a reader can turn back into the value, for
    /// the same reason and in the same shape as [`XmlTooDeep::failure_message`]:
    /// the number first, the part last, so a part name holding a colon still reads
    /// back whole.
    pub fn failure_message(&self) -> String {
        format!("{ENTITIES_FAILURE_TAG}{}:{}", self.line, self.part)
    }

    /// Recover a refusal from a failure message, or `None` when the message is
    /// about something else.
    pub fn from_failure_message(message: &str) -> Option<Self> {
        let rest = message.strip_prefix(ENTITIES_FAILURE_TAG)?;
        let (line, part) = rest.split_once(':')?;
        Some(Self {
            part: part.to_string(),
            line: line.parse().ok()?,
        })
    }
}

impl fmt::Display for XmlDeclaresEntities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} declares an XML entity at line {}. An entity the document names over \
             and over can expand a small file into more memory than the computer has, \
             so a document declaring one is refused unread",
            self.part, self.line
        )
    }
}

impl std::error::Error for XmlDeclaresEntities {}

/// Refuse `xml` if it declares an entity: if `<!ENTITY` appears anywhere after its
/// first `<!DOCTYPE`.
///
/// Read off the text, so nothing is expanded to find out, and wider than the
/// grammar on purpose (see the module note). Bytes, decoded as [`check`] decodes
/// them, so a whole project can be checked before any member is read as text.
pub fn check_entities(part: &str, xml: &[u8]) -> Result<(), XmlDeclaresEntities> {
    let bytes = scannable(xml);
    let Some(doctype) = find(&bytes, b"<!DOCTYPE", 0) else {
        return Ok(());
    };
    match find(&bytes, b"<!ENTITY", doctype) {
        Some(at) => Err(XmlDeclaresEntities {
            part: part.to_string(),
            line: line_of(&bytes, at),
        }),
        None => Ok(()),
    }
}

/// The entity refusal inside `error`, wherever in its chain it sits, whether it
/// arrived as an [`XmlDeclaresEntities`] or wrapped in an [`XmlError`].
pub fn declares_entities(error: &anyhow::Error) -> Option<&XmlDeclaresEntities> {
    error.chain().find_map(|cause| {
        cause.downcast_ref::<XmlDeclaresEntities>().or_else(|| {
            match cause.downcast_ref::<XmlError>() {
                Some(XmlError::DeclaresEntities(refused)) => Some(refused),
                _ => None,
            }
        })
    })
}

/// What a refused project's failure message starts with when one of its members
/// sat too many folders deep; see [`FoldersTooDeep::failure_message`].
const FOLDERS_FAILURE_TAG: &str = "folders-nested-too-deep:";

/// Why a project whose tree is kept as folders was refused before it was read:
/// one of its members sits more than [`MAX_DEPTH`] folders deep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldersTooDeep {
    /// The member refused, or the folder a walk of the project stopped at, as the
    /// project names it: `outline/0-Part/…/0-Scene.md`.
    pub part: String,
    /// How many folders deep it sits, one for every `/` in its name.
    pub depth: usize,
}

impl FoldersTooDeep {
    /// The ceiling this project went past.
    pub fn limit(&self) -> usize {
        MAX_DEPTH
    }

    /// This refusal as one line of text a reader can turn back into the value,
    /// for the same reason and in the same shape as
    /// [`XmlTooDeep::failure_message`].
    pub fn failure_message(&self) -> String {
        format!("{FOLDERS_FAILURE_TAG}{}:{}", self.depth, self.part)
    }

    /// Recover a refusal from a failure message, or `None` when the message is
    /// about something else.
    pub fn from_failure_message(message: &str) -> Option<Self> {
        let rest = message.strip_prefix(FOLDERS_FAILURE_TAG)?;
        let (depth, part) = rest.split_once(':')?;
        Some(Self {
            part: part.to_string(),
            depth: depth.parse().ok()?,
        })
    }
}

impl fmt::Display for FoldersTooDeep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} sits {} folders deep, and the limit is {MAX_DEPTH}. A project nested this \
             deeply would crash the importer, so it is refused unread",
            self.part, self.depth
        )
    }
}

impl std::error::Error for FoldersTooDeep {}

/// Refuse the container member `member` if it sits more than [`MAX_DEPTH`]
/// folders deep, one for every `/` in its name.
pub fn check_folders(member: &str) -> Result<(), FoldersTooDeep> {
    let depth = member.bytes().filter(|&b| b == b'/').count();
    if depth > MAX_DEPTH {
        return Err(FoldersTooDeep {
            part: member.to_string(),
            depth,
        });
    }
    Ok(())
}

/// The folder refusal inside `error`, wherever in its chain it sits.
pub fn folders_too_deep(error: &anyhow::Error) -> Option<&FoldersTooDeep> {
    error
        .chain()
        .find_map(|cause| cause.downcast_ref::<FoldersTooDeep>())
}

/// Whether a document may carry a DTD. `roxmltree` refuses one unless told otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dtd {
    Refuse,
    Allow,
}

/// Why [`parse`] or [`on_parser_stack`] returned no document.
#[derive(Debug)]
pub enum XmlError {
    /// Nested past [`MAX_DEPTH`]; refused before any parser saw it.
    TooDeep(XmlTooDeep),
    /// Declares an entity while a DTD is allowed; refused before any parser could
    /// expand it.
    DeclaresEntities(XmlDeclaresEntities),
    /// Not well-formed XML, in `roxmltree`'s own words.
    Malformed(roxmltree::Error),
    /// The thread the parse runs on could not be started. Nothing was parsed.
    NoParserThread(std::io::Error),
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlError::TooDeep(too_deep) => too_deep.fmt(f),
            XmlError::DeclaresEntities(refused) => refused.fmt(f),
            XmlError::Malformed(error) => error.fmt(f),
            XmlError::NoParserThread(error) => {
                write!(f, "could not start a thread to read the XML on: {error}")
            }
        }
    }
}

impl std::error::Error for XmlError {}

impl From<XmlTooDeep> for XmlError {
    fn from(too_deep: XmlTooDeep) -> Self {
        XmlError::TooDeep(too_deep)
    }
}

impl From<XmlDeclaresEntities> for XmlError {
    fn from(refused: XmlDeclaresEntities) -> Self {
        XmlError::DeclaresEntities(refused)
    }
}

/// The refusal inside `error`, wherever in its chain it sits, whether it arrived
/// as an [`XmlTooDeep`] or wrapped in an [`XmlError`].
pub fn too_deep(error: &anyhow::Error) -> Option<&XmlTooDeep> {
    error.chain().find_map(|cause| {
        cause
            .downcast_ref::<XmlTooDeep>()
            .or_else(|| match cause.downcast_ref::<XmlError>() {
                Some(XmlError::TooDeep(too_deep)) => Some(too_deep),
                _ => None,
            })
    })
}

/// Parse `text` into a document, refusing it first if it nests past [`MAX_DEPTH`]
/// or, when `dtd` allows a DTD, if it declares an entity.
///
/// `part` names the file or container member in the refusal. The parse itself runs
/// on the parser stack (see [`on_parser_stack`]); the document it returns borrows
/// `text` as `roxmltree` always does.
///
/// The entity check comes first. It is one pass over the text, and it keeps a
/// hostile DTD away from the depth scan, whose entity bookkeeping it has no use
/// for once declarations are refused. With [`Dtd::Refuse`], `roxmltree` refuses
/// the whole DTD itself.
pub fn parse<'input>(
    part: &str,
    text: &'input str,
    dtd: Dtd,
) -> Result<roxmltree::Document<'input>, XmlError> {
    if dtd == Dtd::Allow {
        check_entities(part, text.as_bytes())?;
    }
    check(part, text.as_bytes())?;
    let allow_dtd = dtd == Dtd::Allow;
    on_parser_stack(move || {
        let options = roxmltree::ParsingOptions {
            allow_dtd,
            ..roxmltree::ParsingOptions::default()
        };
        roxmltree::Document::parse_with_options(text, options)
    })?
    .map_err(XmlError::Malformed)
}

thread_local! {
    /// Set on a thread [`on_parser_stack`] started, so work already running there
    /// does not start a second one for every document it parses.
    static ON_PARSER_STACK: Cell<bool> = const { Cell::new(false) };
}

/// Run `work` on a thread with [`PARSER_STACK_BYTES`] of stack and wait for it.
///
/// For parsers this module cannot wrap itself: a [`check`]ed `docx-rs` read, or a
/// whole scan that parses several parts and walks what it parsed. Called from a
/// thread this function already started, `work` simply runs there.
///
/// A panic inside `work` is carried back and resumed on the calling thread, so it
/// reaches the same `catch_unwind` it would have reached had `work` run in place.
pub fn on_parser_stack<T: Send>(work: impl FnOnce() -> T + Send) -> Result<T, XmlError> {
    if ON_PARSER_STACK.with(Cell::get) {
        return Ok(work());
    }
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("xml-parser".to_string())
            .stack_size(PARSER_STACK_BYTES)
            .spawn_scoped(scope, || {
                ON_PARSER_STACK.with(|flag| flag.set(true));
                work()
            })
            .map_err(XmlError::NoParserThread)?;
        match handle.join() {
            Ok(value) => Ok(value),
            Err(panic) => std::panic::resume_unwind(panic),
        }
    })
}

/// [`on_parser_stack`] for work that reports progress through a callback the
/// calling thread owns.
///
/// An importer's reporter is a `&dyn Fn`, which cannot cross to another thread,
/// and it is how a long operation's progress toast moves. So `work` is handed a
/// relay instead, and every report it makes is carried back and passed to
/// `report` on the calling thread, in order, while `work` is still running.
///
/// This is for the walks that follow a parse and are as deep as the tree they
/// walk: a converter that recurses once per level of a project's outline. At the
/// ceiling the Plume mapper alone needs about 1.7 MB of stack in a debug build,
/// most of a long operation's 2 MiB; on the parser stack it needs a twentieth of
/// what is there.
pub fn on_parser_stack_reporting<T: Send>(
    report: &dyn Fn(f32, &str),
    work: impl FnOnce(&dyn Fn(f32, &str)) -> T + Send,
) -> Result<T, XmlError> {
    if ON_PARSER_STACK.with(Cell::get) {
        return Ok(work(report));
    }
    let (sender, reports) = std::sync::mpsc::channel::<(f32, String)>();
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("xml-parser".to_string())
            .stack_size(PARSER_STACK_BYTES)
            .spawn_scoped(scope, move || {
                ON_PARSER_STACK.with(|flag| flag.set(true));
                let relay = move |percent: f32, label: &str| {
                    // Fails only once the receiving end is gone, and it is not
                    // dropped before this thread has finished.
                    let _ = sender.send((percent, label.to_string()));
                };
                work(&relay)
            })
            .map_err(XmlError::NoParserThread)?;
        // Ends when the worker drops its sender, which it does when it returns or
        // unwinds, so this can neither stop early nor wait for ever.
        for (percent, label) in reports {
            report(percent, &label);
        }
        match handle.join() {
            Ok(value) => Ok(value),
            Err(panic) => std::panic::resume_unwind(panic),
        }
    })
}

/// Refuse `xml` if its elements nest past [`MAX_DEPTH`].
///
/// Bytes rather than text, because a container member is bytes until a parser
/// decides otherwise. UTF-8 (with or without a byte-order mark) and every other
/// encoding that keeps `<`, `>`, `/`, `&`, `;` and the quotes as single ASCII bytes
/// are scanned as they are; UTF-16 is recognised by its byte-order mark and decoded
/// first.
pub fn check(part: &str, xml: &[u8]) -> Result<(), XmlTooDeep> {
    let bytes = scannable(xml);
    let mut scan = Scan {
        values: Vec::new(),
        by_name: HashMap::new(),
        measured: HashMap::new(),
    };
    scan.walk(&bytes, 0, true)
        .map(|_| ())
        .map_err(|(depth, at)| XmlTooDeep {
            part: part.to_string(),
            depth,
            line: line_of(&bytes, at),
        })
}

/// `xml` as bytes the scans can read: a UTF-8 byte-order mark dropped, UTF-16
/// with a byte-order mark decoded, anything else as it is (see [`check`]).
fn scannable(xml: &[u8]) -> Cow<'_, [u8]> {
    if let Some(rest) = xml.strip_prefix(b"\xEF\xBB\xBF") {
        Cow::Borrowed(rest)
    } else if xml.starts_with(b"\xFF\xFE") || xml.starts_with(b"\xFE\xFF") {
        Cow::Owned(decode_utf16(xml).into_bytes())
    } else {
        Cow::Borrowed(xml)
    }
}

/// The offset of the first `needle` at or after `from`.
fn find(text: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    text.get(from..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|offset| from + offset)
}

/// UTF-16 with a byte-order mark, as UTF-8. A lone surrogate becomes U+FFFD, which
/// cannot change where a tag starts or ends.
fn decode_utf16(bytes: &[u8]) -> String {
    let little_endian = bytes.starts_with(b"\xFF\xFE");
    let (pairs, _odd_byte) = bytes.get(2..).unwrap_or_default().as_chunks::<2>();
    let units = pairs.iter().map(|&pair| {
        if little_endian {
            u16::from_le_bytes(pair)
        } else {
            u16::from_be_bytes(pair)
        }
    });
    char::decode_utf16(units)
        .map(|unit| unit.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}

fn line_of(bytes: &[u8], at: usize) -> usize {
    1 + bytes
        .get(..at)
        .unwrap_or(bytes)
        .iter()
        .filter(|&&b| b == b'\n')
        .count()
}

/// One document's scan: the entities its DTD declared, and how each one's value
/// moves the depth it is expanded at.
struct Scan<'a> {
    /// The value of every name declared, in declaration order. A name declared
    /// again keeps its first value, as `roxmltree` keeps it, which is also the XML
    /// rule: the first declaration binds.
    values: Vec<&'a [u8]>,
    /// Each declared name's index in `values`.
    ///
    /// A map, because every reference in the document is looked up here. Found by
    /// walking the declarations, a lookup cost as many comparisons as there were
    /// names before it, so the scan cost declarations times references: 50,000
    /// names referenced 200,000 times, 2.6 MB of XML, took a minute in a debug
    /// build, and a part within an importer's size limits would take days, with no
    /// parser started and nothing to cancel it. `roxmltree` finds a name the same
    /// slow way, one more reason none of the readers here lets it expand a
    /// declared entity.
    by_name: HashMap<&'a [u8], usize>,
    /// `(entity index, reference level)` → what its value does to the depth, or
    /// `None` when it reaches past the ceiling or past `roxmltree`'s own
    /// reference limit. Memoised so a value referenced a thousand times is
    /// measured once.
    measured: HashMap<(usize, u8), Option<Extent>>,
}

/// What a run of content does to the depth it starts at, relative to that depth.
///
/// Relative, so one measurement of an entity's value serves every place it is
/// referenced, however deep.
#[derive(Debug, Clone, Copy)]
struct Extent {
    /// The deepest level anything in it reaches.
    peak: isize,
    /// Where it leaves the depth. Not always where it started: `roxmltree` lets an
    /// entity's value open an element and leave it open, and a later reference
    /// close it, so a value like `<outline>` nests every tag after the reference
    /// one level further, and the next reference one further again.
    end: isize,
}

impl<'a> Scan<'a> {
    /// Walk `text` as XML content that sits `level` entity expansions down,
    /// measuring what it does to the depth it starts at.
    ///
    /// `outermost` is the document itself, whose depth starts at nothing and
    /// cannot go below it. An entity's value is measured from zero as well, but
    /// may close more than it opens: closing elements the document opened before
    /// the reference is exactly what a value like `</outline>` does.
    ///
    /// `Err((depth, at))` as soon as a level past [`MAX_DEPTH`] is reached, `at`
    /// being the byte offset in `text` of the tag or reference that reached it.
    /// In an entity's value that level is relative, which only understates it.
    fn walk(
        &mut self,
        text: &'a [u8],
        level: u8,
        outermost: bool,
    ) -> Result<Extent, (usize, usize)> {
        let ceiling = MAX_DEPTH as isize;
        // Only ever called with a level past the ceiling, so always positive.
        let past = |reached: isize| usize::try_from(reached).unwrap_or(MAX_DEPTH + 1);
        let mut depth: isize = 0;
        let mut peak: isize = 0;
        let mut at = 0;
        while at < text.len() {
            match text[at] {
                b'<' => {
                    let rest = &text[at..];
                    at = if rest.starts_with(b"<!--") {
                        skip_past(text, at + 4, b"-->")
                    } else if rest.starts_with(b"<![CDATA[") {
                        skip_past(text, at + 9, b"]]>")
                    } else if rest.starts_with(b"<!DOCTYPE") && level == 0 {
                        self.doctype(text, at + 9)
                    } else if rest.starts_with(b"<!") {
                        skip_past(text, at + 2, b">")
                    } else if rest.starts_with(b"<?") {
                        skip_past(text, at + 2, b"?>")
                    } else if rest.starts_with(b"</") {
                        depth -= 1;
                        if outermost {
                            depth = depth.max(0);
                        }
                        skip_past(text, at + 2, b">")
                    } else {
                        let (end, empty) = start_tag_end(text, at + 1);
                        let reached = depth + 1;
                        if reached > ceiling {
                            return Err((past(reached), at));
                        }
                        peak = peak.max(reached);
                        if !empty {
                            depth = reached;
                        }
                        end
                    }
                }
                b'&' => {
                    let reference = at;
                    let Some((name, end)) = reference_name(text, at + 1) else {
                        at += 1;
                        continue;
                    };
                    at = end;
                    let Some(index) = self.entity(name) else {
                        continue;
                    };
                    let Some(value) = self.expansion(index, level + 1) else {
                        return Err((MAX_DEPTH + 1, reference));
                    };
                    // One level for the expansion itself, whose frames sit between
                    // the reference and the first element of its value.
                    let reached = depth + 1 + value.peak;
                    if reached > ceiling {
                        return Err((past(reached), reference));
                    }
                    peak = peak.max(reached);
                    depth += value.end;
                    if outermost {
                        depth = depth.max(0);
                    }
                }
                _ => at += 1,
            }
        }
        Ok(Extent { peak, end: depth })
    }

    /// What expanding entity `index` at reference `level` does to the depth, or
    /// `None` when it reaches past the ceiling wherever it is referenced.
    fn expansion(&mut self, index: usize, level: u8) -> Option<Extent> {
        if level > ENTITY_NESTING {
            return None;
        }
        if let Some(measured) = self.measured.get(&(index, level)) {
            return *measured;
        }
        let value = self.values.get(index).copied()?;
        let measured = self.walk(value, level, false).ok();
        self.measured.insert((index, level), measured);
        measured
    }

    fn entity(&self, name: &[u8]) -> Option<usize> {
        if PREDEFINED.contains(&name) {
            return None;
        }
        self.by_name.get(name).copied()
    }

    /// Skip a DOCTYPE whose `<!DOCTYPE` ends just before `at`, recording every
    /// entity its internal subset declares. Returns the offset just past it.
    fn doctype(&mut self, text: &'a [u8], mut at: usize) -> usize {
        // The root name and any external identifier, up to `[` or `>`. The
        // identifier's literals are quoted and may hold either.
        let mut quote = None;
        loop {
            let Some(&byte) = text.get(at) else {
                return text.len();
            };
            at += 1;
            match (quote, byte) {
                (Some(open), _) if byte == open => quote = None,
                (Some(_), _) => {}
                (None, b'"' | b'\'') => quote = Some(byte),
                (None, b'>') => return at,
                (None, b'[') => break,
                (None, _) => {}
            }
        }
        // The internal subset, read the way `roxmltree`'s `parse_doctype` reads it.
        loop {
            while text.get(at).is_some_and(u8::is_ascii_whitespace) {
                at += 1;
            }
            let Some(rest) = text.get(at..).filter(|rest| !rest.is_empty()) else {
                return text.len();
            };
            at = if rest.starts_with(b"<!ENTITY") {
                self.entity_declaration(text, at + 8)
            } else if rest.starts_with(b"<!--") {
                skip_past(text, at + 4, b"-->")
            } else if rest.starts_with(b"<?") {
                skip_past(text, at + 2, b"?>")
            } else if rest.starts_with(b"]") {
                return skip_past(text, at + 1, b">");
            } else if rest.starts_with(b"<!") {
                // `<!ELEMENT`, `<!ATTLIST`, `<!NOTATION`: skipped to the first `>`,
                // quoted or not, which is exactly how `roxmltree` skips them.
                skip_past(text, at + 2, b">")
            } else {
                // Anything else is a parameter-entity reference or an error, and
                // neither can open an element.
                at + 1
            };
        }
    }

    /// Record the declaration whose `<!ENTITY` ends just before `at`, and return
    /// the offset just past its closing `>`.
    fn entity_declaration(&mut self, text: &'a [u8], mut at: usize) -> usize {
        let skip_spaces = |mut at: usize| {
            while text.get(at).is_some_and(u8::is_ascii_whitespace) {
                at += 1;
            }
            at
        };
        at = skip_spaces(at);
        // A parameter entity. `roxmltree` records it under its name all the same.
        if text.get(at) == Some(&b'%') {
            at = skip_spaces(at + 1);
        }
        let name_start = at;
        while text
            .get(at)
            .is_some_and(|b| !b.is_ascii_whitespace() && !matches!(b, b'>' | b'"' | b'\''))
        {
            at += 1;
        }
        let name = &text[name_start..at];
        at = skip_spaces(at);
        if let Some(&quote @ (b'"' | b'\'')) = text.get(at) {
            let value_start = at + 1;
            let value_end = text
                .get(value_start..)
                .and_then(|rest| rest.iter().position(|&b| b == quote))
                .map_or(text.len(), |offset| value_start + offset);
            if !name.is_empty()
                && let Entry::Vacant(slot) = self.by_name.entry(name)
            {
                slot.insert(self.values.len());
                self.values.push(&text[value_start..value_end]);
            }
            at = value_end + 1;
        }
        // An external entity (`SYSTEM`/`PUBLIC`) is declared without a value: no
        // resolver is ever given to `roxmltree`, so it expands to nothing. Its
        // literals are quoted and may hold `>`.
        let mut quote = None;
        while let Some(&byte) = text.get(at) {
            at += 1;
            match (quote, byte) {
                (Some(open), _) if byte == open => quote = None,
                (Some(_), _) => {}
                (None, b'"' | b'\'') => quote = Some(byte),
                (None, b'>') => return at,
                (None, _) => {}
            }
        }
        text.len()
    }
}

/// The offset just past the first `needle` at or after `from`, or the end of
/// `text` when there is none.
fn skip_past(text: &[u8], from: usize, needle: &[u8]) -> usize {
    text.get(from..)
        .and_then(|rest| rest.windows(needle.len()).position(|w| w == needle))
        .map_or(text.len(), |offset| from + offset + needle.len())
}

/// Find the `>` closing a start tag whose name begins at `from`, skipping quoted
/// attribute values. Returns the offset just past it and whether the tag was
/// self-closing. An unterminated tag counts as an open one.
fn start_tag_end(text: &[u8], from: usize) -> (usize, bool) {
    let mut quote = None;
    let mut at = from;
    while let Some(&byte) = text.get(at) {
        match (quote, byte) {
            (Some(open), _) if byte == open => quote = None,
            (Some(_), _) => {}
            (None, b'"' | b'\'') => quote = Some(byte),
            (None, b'>') => {
                let empty = at > from && text[at - 1] == b'/';
                return (at + 1, empty);
            }
            (None, _) => {}
        }
        at += 1;
    }
    (text.len(), false)
}

/// The name of an entity reference starting just after its `&`, and the offset
/// just past its `;`. `None` for a character reference or for anything that is not
/// a reference at all.
fn reference_name(text: &[u8], from: usize) -> Option<(&[u8], usize)> {
    if text.get(from) == Some(&b'#') {
        return None;
    }
    let rest = text.get(from..)?;
    let end = rest
        .iter()
        .position(|&b| b == b';' || b == b'<' || b == b'&' || b.is_ascii_whitespace())?;
    (rest[end] == b';' && end > 0).then_some((&rest[..end], from + end + 1))
}

/// A global allocator that measures how far the heap rose during an import, for
/// the tests showing an entity bomb was refused without being expanded.
///
/// Behind the `hostile-fixtures` feature, which only the dev-dependencies of the
/// crates whose readers allow a DTD turn on.
#[cfg(any(test, feature = "hostile-fixtures"))]
pub mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    /// `levels` elements nested inside one another, the root included.
    fn nested(levels: usize) -> String {
        format!("{}{}", "<a>".repeat(levels), "</a>".repeat(levels))
    }

    #[test]
    fn a_document_at_the_ceiling_passes_and_one_level_more_is_refused() {
        assert!(check("at.xml", nested(MAX_DEPTH).as_bytes()).is_ok());
        let refused = check("past.xml", nested(MAX_DEPTH + 1).as_bytes())
            .expect_err("one level past the ceiling must be refused");
        assert_eq!(refused.part, "past.xml");
        assert_eq!(refused.depth, MAX_DEPTH + 1);
        assert_eq!(refused.line, 1);
    }

    #[test]
    fn siblings_do_not_accumulate() {
        let text = format!("<r>{}</r>", "<p><s>x</s><e/></p>".repeat(10_000));
        assert!(check("wide.xml", text.as_bytes()).is_ok());
    }

    /// Nothing in a comment, a CDATA section, a processing instruction or an
    /// attribute value is a tag, however much it looks like one.
    #[test]
    fn markup_that_is_not_markup_is_not_counted() {
        let deep = "<x>".repeat(MAX_DEPTH * 2);
        for text in [
            format!("<r><!--{deep}--></r>"),
            format!("<r><![CDATA[{deep}]]></r>"),
            format!("<r><?pi {deep}?></r>"),
            format!("<r a=\"{deep}\" b='>'/>"),
            format!(
                "<!DOCTYPE r [<!ATTLIST r a CDATA 'x'>]><r>{}</r>",
                "&gt;".repeat(10)
            ),
        ] {
            assert!(
                check("x", text.as_bytes()).is_ok(),
                "should pass: {text:.60}"
            );
        }
    }

    #[test]
    fn a_self_closing_tag_opens_nothing() {
        let text = format!("<r>{}</r>", "<e/>".repeat(MAX_DEPTH * 2));
        assert!(check("x", text.as_bytes()).is_ok());
    }

    /// An empty element is still an element at its depth: one inside 255 open ones
    /// is the 256th level, one inside 256 is past the ceiling.
    #[test]
    fn a_self_closing_tag_counts_at_its_own_depth() {
        let open = |levels: usize| format!("{}<e/>{}", "<a>".repeat(levels), "</a>".repeat(levels));
        assert!(check("x", open(MAX_DEPTH - 1).as_bytes()).is_ok());
        let refused = check("x", open(MAX_DEPTH).as_bytes()).expect_err("257 levels");
        assert_eq!(refused.depth, MAX_DEPTH + 1);
    }

    #[test]
    fn an_unclosed_run_of_tags_is_refused() {
        let text = "<a>".repeat(MAX_DEPTH + 1);
        assert!(check("x", text.as_bytes()).is_err());
    }

    #[test]
    fn the_refusal_names_the_line() {
        let text = format!("<r>\n\n{}", "<a>\n".repeat(MAX_DEPTH));
        let refused = check("x", text.as_bytes()).expect_err("too deep");
        assert_eq!(refused.line, MAX_DEPTH + 2);
    }

    /// An entity value holding markup nests the document further than its own tags
    /// say: `roxmltree` parses the value in place, at the depth of the reference.
    #[test]
    fn an_entity_carrying_markup_counts_where_it_is_expanded() {
        let value = nested(200);
        let text = format!(
            "<!DOCTYPE r [<!ENTITY e \"{value}\">]><r>{}&e;{}</r>",
            "<a>".repeat(100),
            "</a>".repeat(100)
        );
        let refused = check("x", text.as_bytes()).expect_err("100 + 200 levels");
        assert!(refused.depth > MAX_DEPTH);

        let shallow = format!("<!DOCTYPE r [<!ENTITY e \"{}\">]><r>&e;&e;</r>", nested(10));
        assert!(check("x", shallow.as_bytes()).is_ok());
    }

    #[test]
    fn entities_nested_in_entities_add_up() {
        // Each value adds 30 levels and references the next: 9 × 30 is past 256.
        let mut dtd = String::new();
        for i in 0..9 {
            let next = if i < 8 {
                format!("&e{};", i + 1)
            } else {
                String::new()
            };
            dtd.push_str(&format!(
                "<!ENTITY e{i} '{}{next}{}'>",
                "<a>".repeat(30),
                "</a>".repeat(30)
            ));
        }
        let text = format!("<!DOCTYPE r [{dtd}]><r>&e0;</r>");
        assert!(check("x", text.as_bytes()).is_err());
    }

    /// [`parse`] with a DTD allowed, minus the entity refusal: the depth scan,
    /// then `roxmltree` itself. For the tests of what the scan alone guarantees.
    fn parse_measured_only<'input>(
        part: &str,
        text: &'input str,
    ) -> Result<roxmltree::Document<'input>, XmlError> {
        check(part, text.as_bytes())?;
        on_parser_stack(move || {
            let options = roxmltree::ParsingOptions {
                allow_dtd: true,
                ..roxmltree::ParsingOptions::default()
            };
            roxmltree::Document::parse_with_options(text, options)
        })?
        .map_err(XmlError::Malformed)
    }

    /// How deep `document`'s elements go, the root counting as one.
    fn tree_depth(document: &roxmltree::Document<'_>) -> usize {
        document
            .descendants()
            .filter(roxmltree::Node::is_element)
            .map(|node| node.ancestors().filter(roxmltree::Node::is_element).count())
            .max()
            .unwrap_or(0)
    }

    /// `roxmltree` goes on building the tree where an entity's value left it, so
    /// a value that opens an element and never closes it nests everything after
    /// the reference one level further. This is the shape of a hostile
    /// `world.opml`: one value opens an `<outline>`, the other closes one, no
    /// single expansion is deep, and a thousand pairs of them are.
    #[test]
    fn elements_an_entity_leaves_open_count_after_the_reference() {
        let unbalanced = |pairs: usize| {
            format!(
                "<!DOCTYPE opml [<!ENTITY o '<outline name=\"x\">'>\
                 <!ENTITY c '<x/></outline>'>]><opml><body>{}{}</body></opml>",
                "&o;".repeat(pairs),
                "&c;".repeat(pairs)
            )
        };
        let refused = check("world.opml", unbalanced(1000).as_bytes())
            .expect_err("a thousand levels opened one entity at a time");
        assert_eq!(refused.part, "world.opml");
        assert!(refused.depth > MAX_DEPTH);

        // What passes, parsed by `roxmltree` itself, is a tree no deeper than the
        // ceiling; and the check is not simply refusing the shape, since the
        // deepest that passes comes within a level or two of it. [`parse`] would
        // refuse these documents for declaring entities before measuring them,
        // so this is the scan alone, as a caller of [`check`] relies on it.
        let mut deepest_passing = 0;
        for pairs in [1, 100, 250, 251, 252, 253, 254, 255, 256, 257] {
            let text = unbalanced(pairs);
            match parse_measured_only("world.opml", &text) {
                Ok(document) => {
                    let depth = tree_depth(&document);
                    assert!(
                        depth <= MAX_DEPTH,
                        "{pairs} pairs passed the check and parsed {depth} deep"
                    );
                    deepest_passing = deepest_passing.max(depth);
                }
                Err(XmlError::TooDeep(_)) => {}
                Err(other) => panic!("{pairs} pairs must parse or be refused, got: {other}"),
            }
        }
        assert!(
            deepest_passing >= MAX_DEPTH - 2,
            "the deepest tree passed was only {deepest_passing} levels"
        );
    }

    /// An entity's value may close what the document opened before the reference,
    /// so a value measured on its own can go below where it started, and what
    /// follows the reference is counted from there.
    #[test]
    fn an_entity_closing_what_the_document_opened_is_followed_down() {
        let text = format!(
            "<!DOCTYPE r [<!ENTITY c '<x/></a>'>]><r>{}{}{}</r>",
            "<a>".repeat(200),
            "&c;".repeat(200),
            // Back at the root's level, so another 250 fit under it.
            nested(250),
        );
        assert!(check("x", text.as_bytes()).is_ok());
    }

    /// A loop never ends in `roxmltree` either; it gives up after ten references.
    /// Everything it parsed on the way is still on the stack, so a loop is as deep
    /// as the scan can tell, and refused.
    #[test]
    fn a_reference_loop_is_refused() {
        let text = "<!DOCTYPE r [<!ENTITY a '<x>&b;</x>'><!ENTITY b '<y>&a;</y>'>]><r>&a;</r>";
        assert!(check("x", text.as_bytes()).is_err());
    }

    /// The exponential entity expansion has depth of ten and nothing more; the
    /// memo is what keeps its scan linear.
    #[test]
    fn a_billion_laughs_is_measured_once_per_entity() {
        let mut dtd = String::from("<!ENTITY l0 'lol'>");
        for i in 1..10 {
            dtd.push_str(&format!(
                "<!ENTITY l{i} '{}'>",
                format!("&l{};", i - 1).repeat(10)
            ));
        }
        let text = format!("<!DOCTYPE r [{dtd}]><r>&l9;</r>");
        assert!(check("x", text.as_bytes()).is_ok());
    }

    /// `declarations` entities, and a root naming the last one declared
    /// `references` times: the name a search in declaration order finds last.
    fn many_entities(declarations: usize, references: usize) -> String {
        let mut text = String::from("<!DOCTYPE r [");
        for i in 0..declarations {
            text.push_str(&format!("<!ENTITY e{i} 'x'>"));
        }
        let last = declarations.saturating_sub(1);
        text.push_str(&format!(
            "]><r>{}</r>",
            format!("&e{last};").repeat(references)
        ));
        text
    }

    /// What `work` returned, or `None` when it was still running after `budget`.
    /// It runs on a thread of its own, so a check that regresses fails the test
    /// when the budget runs out rather than when the work finally ends.
    fn within<T: Send + 'static>(
        budget: std::time::Duration,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> Option<T> {
        let (done, outcome) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            // The receiver is gone only once the budget ran out; nothing waits then.
            let _ = done.send(work());
        });
        outcome.recv_timeout(budget).ok()
    }

    /// Every reference is looked up among the declared names, so a lookup that
    /// walked the declarations made the scan cost declarations times references:
    /// this document, 2.6 MB, took a minute in a debug build. Each lookup now
    /// costs the same however many names were declared, and the scan a fraction
    /// of a second.
    #[test]
    fn a_lookup_costs_the_same_however_many_entities_are_declared() {
        let text = many_entities(50_000, 200_000);
        let bytes = text.len();
        match within(std::time::Duration::from_secs(5), move || {
            check("x", text.as_bytes()).is_ok()
        }) {
            Some(passed) => assert!(passed, "one level of text is not deep"),
            None => panic!(
                "checking 50,000 declarations named 200,000 times ({bytes} bytes) took \
                 more than 5 s"
            ),
        }
    }

    /// XML binds a name to its first declaration, and `roxmltree` looks names up
    /// that way too, so a later declaration of the same name is never the one
    /// measured: not when it is deeper, and not when it is shallower.
    #[test]
    fn the_first_declaration_of_a_name_is_the_one_measured() {
        let document = |first: usize, second: usize| {
            format!(
                "<!DOCTYPE r [<!ENTITY e '{}'><!ENTITY e '{}'>]><r>{}&e;{}</r>",
                nested(first),
                nested(second),
                "<a>".repeat(100),
                "</a>".repeat(100)
            )
        };
        assert!(check("x", document(200, 1).as_bytes()).is_err());

        let shallow_first = document(1, 200);
        assert!(check("x", shallow_first.as_bytes()).is_ok());
        let parsed = parse_measured_only("x", &shallow_first).expect("parses");
        assert!(tree_depth(&parsed) <= MAX_DEPTH);
    }

    #[test]
    fn predefined_and_character_references_are_text() {
        let text = "<!DOCTYPE r [<!ENTITY lt '<a><a><a>'>]><r>&lt;&#60;&#x3C;&amp;</r>";
        assert!(check("x", text.as_bytes()).is_ok());
    }

    #[test]
    fn utf16_is_scanned_as_the_text_it_encodes() {
        let text = nested(MAX_DEPTH);
        let mut le = vec![0xFF, 0xFE];
        let mut be = vec![0xFE, 0xFF];
        for unit in text.encode_utf16() {
            le.extend_from_slice(&unit.to_le_bytes());
            be.extend_from_slice(&unit.to_be_bytes());
        }
        assert!(check("le", &le).is_ok());
        assert!(check("be", &be).is_ok());

        let mut deep = vec![0xFF, 0xFE];
        for unit in nested(MAX_DEPTH + 1).encode_utf16() {
            deep.extend_from_slice(&unit.to_le_bytes());
        }
        assert!(check("deep", &deep).is_err());
    }

    #[test]
    fn a_refusal_survives_the_trip_through_a_failure_message() {
        let refused = XmlTooDeep {
            part: "odd:name.xml".to_string(),
            depth: 257,
            line: 3,
        };
        let message = refused.failure_message();
        assert_eq!(XmlTooDeep::from_failure_message(&message), Some(refused));
        assert_eq!(XmlTooDeep::from_failure_message("parsing XML: boom"), None);
    }

    /// A member sitting exactly at the ceiling passes, one folder more is refused,
    /// and the refusal survives the trip through a failure message and through
    /// context layers, like its XML sibling.
    #[test]
    fn folders_past_the_ceiling_are_refused_and_travel_as_the_typed_value() {
        let member = |folders: usize| format!("{}x.md", "0-a/".repeat(folders));
        assert!(check_folders(&member(MAX_DEPTH)).is_ok());
        let refused = check_folders(&member(MAX_DEPTH + 1)).expect_err("one folder past");
        assert_eq!(refused.depth, MAX_DEPTH + 1);
        assert_eq!(refused.part, member(MAX_DEPTH + 1));
        assert_eq!(refused.limit(), MAX_DEPTH);

        let message = refused.failure_message();
        assert_eq!(
            FoldersTooDeep::from_failure_message(&message),
            Some(refused.clone())
        );
        assert_eq!(XmlTooDeep::from_failure_message(&message), None);
        assert_eq!(
            FoldersTooDeep::from_failure_message(
                &XmlTooDeep {
                    part: "world.opml".to_string(),
                    depth: 257,
                    line: 1,
                }
                .failure_message()
            ),
            None
        );

        let wrapped = anyhow::Error::new(refused.clone()).context("opening the project");
        assert_eq!(folders_too_deep(&wrapped), Some(&refused));
        assert_eq!(too_deep(&wrapped), None);
    }

    #[test]
    fn a_refusal_is_found_through_context_layers() {
        let refused = XmlTooDeep {
            part: "tree".to_string(),
            depth: 257,
            line: 1,
        };
        let wrapped = anyhow::Error::new(XmlError::TooDeep(refused.clone()))
            .context("reading the outline")
            .context("importing");
        assert_eq!(too_deep(&wrapped), Some(&refused));
        let direct = anyhow::Error::new(refused.clone()).context("importing");
        assert_eq!(too_deep(&direct), Some(&refused));
        assert_eq!(too_deep(&anyhow::anyhow!("something else")), None);
    }

    /// The measurement behind the ceiling, kept honest: a document exactly at it
    /// parses from a thread with the stack a long operation gets, because the
    /// parse itself runs on the parser stack.
    #[test]
    fn a_document_at_the_ceiling_parses_from_a_two_mebibyte_thread() {
        let outcome = std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(|| {
                let text = nested(MAX_DEPTH);
                parse("at.xml", &text, Dtd::Refuse)
                    .map(|document| document.descendants().filter(|n| n.is_element()).count())
                    .map_err(|e| e.to_string())
            })
            .expect("spawn")
            .join()
            .expect("the parse must not unwind");
        assert_eq!(outcome, Ok(MAX_DEPTH));
    }

    #[test]
    fn a_document_past_the_ceiling_is_refused_before_it_is_parsed() {
        let text = nested(100_000);
        match parse("hostile.xml", &text, Dtd::Allow) {
            Err(XmlError::TooDeep(refused)) => assert_eq!(refused.part, "hostile.xml"),
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[test]
    fn a_malformed_document_is_reported_as_malformed() {
        assert!(matches!(
            parse("bad.xml", "<a><b></a>", Dtd::Refuse),
            Err(XmlError::Malformed(_))
        ));
    }

    #[test]
    fn a_panic_on_the_parser_stack_reaches_the_caller() {
        let caught = std::panic::catch_unwind(|| {
            on_parser_stack(|| -> usize { std::panic::panic_any("inside the parser") })
        });
        assert!(caught.is_err());
    }

    /// Progress made on the parser stack reaches the caller's reporter on the
    /// caller's own thread, every report and in order.
    #[test]
    fn reports_made_on_the_parser_stack_arrive_on_the_calling_thread_in_order() {
        let caller = std::thread::current().id();
        let seen = std::cell::RefCell::new(Vec::new());
        let report = |percent: f32, label: &str| {
            assert_eq!(std::thread::current().id(), caller);
            seen.borrow_mut().push((percent, label.to_string()));
        };
        let worker = on_parser_stack_reporting(&report, |relay| {
            for step in 0..5u8 {
                relay(f32::from(step) * 10.0, &format!("step {step}"));
            }
            std::thread::current().id()
        })
        .expect("worker");
        assert_ne!(worker, caller);
        let seen = seen.into_inner();
        assert_eq!(seen.len(), 5);
        assert_eq!(seen[4], (40.0, "step 4".to_string()));
    }

    #[test]
    fn work_already_on_the_parser_stack_runs_in_place() {
        let outer = on_parser_stack(|| {
            let outer = std::thread::current().id();
            let inner = on_parser_stack(|| std::thread::current().id()).expect("inner");
            (outer, inner)
        })
        .expect("outer");
        assert_eq!(outer.0, outer.1);
        assert_ne!(outer.0, std::thread::current().id());
    }

    // -----------------------------------------------------------------------
    // Entities a document declares
    // -----------------------------------------------------------------------

    /// One entity named from the document itself, over and over: the shape
    /// `roxmltree`'s billion-laughs guard lets through, since no reference sits
    /// inside another.
    fn amplified(value_bytes: usize, references: usize) -> String {
        format!(
            "<!DOCTYPE r [\n<!ENTITY e \"{}\">\n]><r a=\"{refs}\">{refs}</r>",
            "a".repeat(value_bytes),
            refs = "&e;".repeat(references)
        )
    }

    #[test]
    fn a_document_declaring_an_entity_is_refused_before_it_is_parsed() {
        let text = amplified(1 << 10, 1 << 10);
        match parse("tree", &text, Dtd::Allow) {
            Err(XmlError::DeclaresEntities(refused)) => {
                assert_eq!(
                    refused,
                    XmlDeclaresEntities {
                        part: "tree".to_string(),
                        line: 2,
                    }
                );
            }
            other => panic!("expected the entity refusal, got {other:?}"),
        }
    }

    /// A DOCTYPE with nothing to expand is read as before: Plume's bare one, the
    /// HTML 4 one Qt writes with an external identifier, and an internal subset
    /// declaring only attributes. The five predefined entities need no
    /// declaration.
    #[test]
    fn a_doctype_declaring_no_entity_is_read() {
        for text in [
            "<!DOCTYPE plume-tree><plume-tree version=\"0.5\"/>",
            "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" \
             \"http://www.w3.org/TR/REC-html40/strict.dtd\"><html/>",
            "<!DOCTYPE r [<!ATTLIST r a CDATA 'x'>]><r>&lt;&amp;&gt;&quot;&apos;</r>",
            "<?xml version='1.0' encoding='UTF-8'?>\n<opml version=\"1.0\"><body/></opml>",
        ] {
            assert!(
                parse("x", text, Dtd::Allow).is_ok(),
                "should be read: {text}"
            );
        }
    }

    /// `<!ENTITY` declares nothing without a DOCTYPE before it, so text that only
    /// mentions it is read.
    #[test]
    fn entity_text_with_no_doctype_before_it_is_no_declaration() {
        for text in [
            "<r><!-- <!ENTITY is not declared here --><![CDATA[<!ENTITY]]></r>",
            "<!-- <!ENTITY e 'x'> --><!DOCTYPE r><r/>",
        ] {
            assert!(check_entities("x", text.as_bytes()).is_ok(), "{text}");
            assert!(parse("x", text, Dtd::Allow).is_ok(), "{text}");
        }
    }

    /// The rule is wider than the grammar, on purpose: a parameter entity, a
    /// declaration hidden behind a quoted `>` or `]`, and `<!ENTITY` in a CDATA
    /// section after a DOCTYPE are all refused, so no disagreement about where the
    /// internal subset ends can let a declaration through.
    #[test]
    fn any_entity_after_a_doctype_is_refused() {
        for text in [
            "<!DOCTYPE r [<!ENTITY % p 'x'>]><r/>",
            "<!DOCTYPE r [<!ATTLIST r a CDATA ']>'><!ENTITY e 'x'>]><r>&e;</r>",
            "<!DOCTYPE r SYSTEM 'a>b' [<!ENTITY e 'x'>]><r>&e;</r>",
            "<!DOCTYPE r><r><![CDATA[<!ENTITY]]></r>",
        ] {
            assert!(check_entities("x", text.as_bytes()).is_err(), "{text}");
            assert!(
                matches!(
                    parse("x", text, Dtd::Allow),
                    Err(XmlError::DeclaresEntities(_))
                ),
                "{text}"
            );
        }
    }

    /// A reader that refuses a DTD is left to `roxmltree`, which refuses the whole
    /// DOCTYPE, declarations and all, exactly as it did.
    #[test]
    fn a_reader_refusing_the_dtd_is_unchanged() {
        let text = amplified(16, 4);
        assert!(matches!(
            parse("x", &text, Dtd::Refuse),
            Err(XmlError::Malformed(roxmltree::Error::DtdDetected))
        ));
    }

    #[test]
    fn a_declaration_is_found_in_utf16_and_after_a_byte_order_mark() {
        let text = amplified(16, 4);
        let mut le = vec![0xFF, 0xFE];
        for unit in text.encode_utf16() {
            le.extend_from_slice(&unit.to_le_bytes());
        }
        let mut bom = b"\xEF\xBB\xBF".to_vec();
        bom.extend_from_slice(text.as_bytes());
        for bytes in [le, bom] {
            let refused = check_entities("x", &bytes).expect_err("declares one");
            assert_eq!(refused.line, 2);
        }
    }

    #[test]
    fn an_entity_refusal_survives_the_trip_through_a_failure_message() {
        let refused = XmlDeclaresEntities {
            part: "odd:name.xml".to_string(),
            line: 3,
        };
        let message = refused.failure_message();
        assert_eq!(
            XmlDeclaresEntities::from_failure_message(&message),
            Some(refused.clone())
        );
        assert_eq!(XmlTooDeep::from_failure_message(&message), None);
        assert_eq!(FoldersTooDeep::from_failure_message(&message), None);
        let too_deep = XmlTooDeep {
            part: "tree".to_string(),
            depth: 257,
            line: 1,
        };
        assert_eq!(
            XmlDeclaresEntities::from_failure_message(&too_deep.failure_message()),
            None
        );
    }

    #[test]
    fn an_entity_refusal_is_found_through_context_layers() {
        let refused = XmlDeclaresEntities {
            part: "world.opml".to_string(),
            line: 1,
        };
        let wrapped = anyhow::Error::new(XmlError::DeclaresEntities(refused.clone()))
            .context("reading the world")
            .context("importing");
        assert_eq!(declares_entities(&wrapped), Some(&refused));
        assert_eq!(too_deep(&wrapped), None);
        let direct = anyhow::Error::new(refused.clone()).context("importing");
        assert_eq!(declares_entities(&direct), Some(&refused));
        assert_eq!(declares_entities(&anyhow::anyhow!("something else")), None);
    }
}
