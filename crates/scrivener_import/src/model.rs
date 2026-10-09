// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One version-neutral model of a Scrivener project, whatever generation wrote it.
//!
//! Every reader normalises into these types, so the layers above (structure,
//! metadata, prose) map one shape and never ask which generation they are looking
//! at. Two rules decide what is here:
//!
//! * **Nothing is resolved.** A label is still the number Scrivener stored, a
//!   section type is still its UUID, a custom field value is still the option id.
//!   Mapping them onto Skribisto happens in one place, where the vocabularies they
//!   resolve against are built (the Manuskript importer learned this the hard way).
//! * **Every generation's conventions are normalised.** The key scheme (a number in
//!   format 1.0, a number and a UUID in 1.5, a UUID in 2.0), where each item's files
//!   live, the `-1` that means "no label", the `Yes` that is the only value
//!   `IncludeInCompile` ever takes, the date form: the readers settle these, so a
//!   mapper never meets them.
//!
//! # Flat, never recursive
//!
//! The binder and the keyword tree are lists of rows, each carrying its depth, in
//! the order Scrivener lists them. No type here contains itself. A recursive type
//! deserialised from JSON recurses once per level of nesting in the input, and a
//! crafted or corrupt project nests as deep as it likes: the model crosses a long
//! operation's boundary as JSON, and a recursive one failed to deserialise from
//! depth 63. A row's parent is the nearest row above it one level shallower
//! ([`Project::parents`]); [`Project::check_shape`] refuses rows whose depth jumps.
//!
//! # Prose stays where it is
//!
//! An item's text, notes and synopsis are [`MemberRef`]s: where the bytes live in the
//! project, how large they are, when they last changed. The model never holds a
//! document's bytes, so it stays small enough to send to the interface for a
//! preview, and pictures and research files are never read unless they are imported.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A whole Scrivener project, however it was stored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Which container, generation and manifest this model was read from.
    pub source: SourceFacts,
    /// The binder in Scrivener's order, one row per item, each with its depth.
    pub binder: Vec<BinderRow>,
    /// The label list, in menu order, the "no label" row included.
    pub labels: Vocabulary,
    /// The status list, in menu order, the "no status" row included.
    pub statuses: Vocabulary,
    /// The project's keywords, in display order, each with its depth.
    pub keywords: Vec<KeywordRow>,
    /// Custom metadata field definitions, in display order.
    pub custom_fields: Vec<CustomFieldDef>,
    /// Section types (format 2.0 only; empty elsewhere).
    pub section_types: SectionTypes,
    /// Collections, in their tab order.
    pub collections: Vec<Collection>,
    /// Project bookmarks (2.0) or project references (1.x), in order.
    pub project_links: Vec<Link>,
    /// Format 1.x project notes (`Files/ProjectNotes`).
    pub project_notes: Vec<ProjectNote>,
    /// The draft and session targets, as written.
    pub targets: ProjectTargets,
    /// The Document Templates folder, when one is designated.
    pub template_folder: Option<ItemKey>,
    /// Where new project bookmarks go (2.0), when designated.
    pub bookmarks_folder: Option<ItemKey>,
    /// Format 1.x favourites, in order.
    pub favorites: Vec<ItemKey>,
    /// The auto-complete list, as written.
    pub auto_complete: Vec<String>,
    /// Format 1.x `ProjectProperties` (title and author, when set).
    pub properties: ProjectProperties,
    /// Settings files the metadata step reads, where they are.
    pub settings: SettingsFiles,
    /// What the compile settings state that reaches the import: the title, the authors
    /// and the language a writer set there, and what a compile leaves out.
    #[serde(default)]
    pub compile: CompileFacts,
    /// `Files/styles.xml`, when present.
    pub styles: Option<MemberRef>,
    /// `Files/writing.history` (or the iOS copy), when present.
    pub writing_history: Option<MemberRef>,
    /// The day of the manifest's own cache of the latest writing-history day
    /// (`RecentWritingHistory/@Date`, format 2.0), as written.
    #[serde(default)]
    pub recent_writing_day: Option<Stamp>,
    /// Custom binder icons in `Icons/`, by file name.
    pub custom_icons: Vec<MemberRef>,
    /// Text on disk that no binder item references: sync-conflict copies, copies
    /// whose original is gone, and documents the binder does not list.
    pub recovered: Vec<RecoveredText>,
    /// Counts the report needs.
    pub counts: Counts,
}

/// Which container the project came in, which generation wrote it, and which
/// manifest was read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFacts {
    pub container: Container,
    pub generation: Generation,
    /// The platform and version that last saved the project, from its `Creator`.
    pub writer: Writer,
    /// The manifest read, by file name, and how it came to be the one.
    pub manifest: ManifestOrigin,
    /// The project folder's own name (`Novel.scriv`), a title fallback.
    pub folder_name: String,
    /// The project's name as a title: the folder's name (the archive's, for a project at
    /// the top of one) without its extension and without what a backup or a sync service
    /// added to it (` Backup`, `.bak3`, a date stamp, a conflicted-copy note, a computer's
    /// name). A conflict note naming a person goes whatever it names; a computer's name
    /// OneDrive joined on goes when the import can tell it is one (this computer's, the
    /// lock's or the device's, or a name Windows gave a computer itself): a name the writer
    /// gave another computer reads as a word, and the Save page shows it.
    #[serde(default)]
    pub folder_title: String,
    /// The format 2.0 root `Identifier`. Provenance only: distinct novels share
    /// one, so it never identifies a project.
    pub identifier: Option<String>,
    /// When the manifest was last saved, as written. Absent from format 1.0
    /// manifests saved before Scrivener for Mac 2.5 and from Windows 1.x ones.
    pub modified: Option<Stamp>,
    /// `Files/version.txt`, when present.
    pub version_stamp: Option<u32>,
    /// The identifier scheme the binder actually uses, which decides where each
    /// item's files are looked for.
    pub id_scheme: IdScheme,
    /// The title of the template a `.scrivtemplate` carries.
    pub template_title: Option<String>,
    /// The title of the template the project was created from
    /// (`Settings/templateinfo.xml`), when it records one.
    pub created_from_template: Option<String>,
}

/// What the writer handed over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Container {
    /// A project folder (a `.scriv` package on macOS, a plain folder elsewhere).
    Folder,
    /// A zip archive: a Scrivener backup, an iOS export, a Finder "Compress".
    Zip,
    /// A `.scrivtemplate`: a project zip written into an XML wrapper.
    Template(TemplateEncoding),
}

/// How a `.scrivtemplate` spells the project zip inside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateEncoding {
    /// `Type="Base64"` (Scrivener 3).
    Base64,
    /// No `Type`, hexadecimal (Scrivener 2).
    Hex,
    /// No `Type`, and not hexadecimal: read as base64.
    UntypedBase64,
}

/// The on-disk generation the project was read as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Generation {
    /// `Version="1.0"`: Scrivener for Mac 2.0 to 2.6, Windows 1.0 to 1.8, the
    /// Linux beta up to 1.7.
    Format10,
    /// `Version="1.5"`: Scrivener for Mac 2.7 and 2.8, Windows 1.9, the Linux beta
    /// 1.9.
    Format15,
    /// `Version="2.0"`: Scrivener 3 on Mac and Windows, and iOS.
    Format20,
    /// A project created on an iPhone or iPad and never opened on a desktop: an
    /// empty manifest beside a `Mobile/` folder holding the real binder.
    IosStub,
}

/// The platform that last saved the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Mac,
    Windows,
    Linux,
    Ios,
    /// A `Creator` this reader does not recognise, or none.
    Unknown,
}

/// The platform and version that last saved the project. Informational only:
/// the generation is decided by the manifest's `Version`, never by this (the
/// Scrivener 3 betas wrote `2.9` creators).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Writer {
    pub platform: Platform,
    /// The version as the `Creator` spells it (`3.5.2`, `1.9.16.0`).
    pub version: Option<String>,
    /// The `Creator` attribute as written.
    pub creator: Option<String>,
}

/// Which manifest was read, and why that one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestOrigin {
    /// Its file name, inside the project folder.
    pub name: String,
    pub chosen: ManifestChoice,
}

/// How the manifest read came to be chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestChoice {
    /// The only one there.
    Only,
    /// The file the writer picked.
    Picked,
    /// The one the writer chose among several.
    Answered,
    /// Several byte-identical copies; any is the project.
    Identical,
    /// Chosen on converging evidence; the others are sync copies of it.
    Automatic,
    /// An iOS stub: the binder was read from `Mobile/binder.mob`.
    MobileBinder,
}

/// Which identifier scheme a binder uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdScheme {
    /// Numbers only (format 1.0): files under `Files/Docs/<ID>`.
    Id,
    /// Numbers and UUIDs (format 1.5): files under `Files/Docs/<ID>`, identity by
    /// UUID.
    IdAndUuid,
    /// UUIDs only (format 2.0): files under `Files/Data/<UUID>/`.
    Uuid,
}

/// An item's identity inside the model. Format 1.5 items carry both a number and
/// a UUID; the UUID is their identity, because a 1.5 to 2.0 upgrade keeps it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ItemKey {
    Uuid(Uuid),
    Number(u32),
}

/// A date as Scrivener wrote it (`2025-03-14 22:15:28 -0600`), and what it means.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stamp {
    /// The text as written, kept so an unreadable date is reported rather than
    /// silently lost.
    pub raw: String,
    /// The instant, in seconds since the Unix epoch, when the text could be read.
    pub unix_seconds: Option<i64>,
    /// The offset from UTC it was written with, in minutes. Kept because the same
    /// instant written in two time zones is how a writer's travel shows up.
    pub offset_minutes: Option<i16>,
}

/// Where a member of the project is, and what it looked like when indexed.
///
/// The path is relative to the project folder, `/`-separated, as Scrivener names
/// it: `Files/Data/<UUID>/content.rtf`. Never a path outside the project.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MemberRef {
    pub path: String,
    pub size: u64,
    /// Last modification, in nanoseconds since the Unix epoch, when known.
    pub modified_ns: Option<i64>,
}

/// One binder item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderRow {
    /// 0 for the items at the binder's top level (the Draft, Research and Trash
    /// folders and whatever sits beside them).
    pub depth: u16,
    /// The item's identity, when it carries a valid one.
    pub key: Option<ItemKey>,
    /// The number format 1.x gave it: what its files and its range links name.
    pub number: Option<u32>,
    /// False for an item whose key an earlier item already owns: it keeps its
    /// place and its metadata, and gets no files, since the files belong to the
    /// first.
    pub owns_key: bool,
    pub kind: ItemKind,
    /// As written: may be absent, empty, or hold a line break.
    pub title: Option<String>,
    pub created: Option<Stamp>,
    pub modified: Option<Stamp>,
    pub meta: ItemMeta,
    /// Keyword ids, as written (into [`Project::keywords`]).
    pub keyword_ids: Vec<i64>,
    /// Document bookmarks (2.0) or references (1.x), in order.
    pub links: Vec<Link>,
    /// The document's own target, as written.
    pub target: Option<DocTarget>,
    /// The document is in script mode (`TextMode` = `Script`).
    pub script_mode: bool,
    /// The folder's default document template.
    pub default_child_template: Option<ItemKey>,
    /// Drawn as a separator in Scrivener's binder.
    pub binder_separator: bool,
    /// Where the item's text, notes, synopsis and other files are.
    pub payload: Payload,
    /// How many snapshots Scrivener keeps of this item. Snapshots are not
    /// imported; the report says how many stay in the Scrivener project.
    pub snapshots: u32,
}

/// What kind of binder item this is, from its `Type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemKind {
    DraftFolder,
    ResearchFolder,
    TrashFolder,
    Folder,
    Text,
    Image,
    Pdf,
    WebArchive,
    Media,
    Other,
    /// A type this reader does not know, as written.
    Unknown(String),
}

impl ItemKind {
    /// Read a `Type` attribute.
    pub fn from_type(raw: &str) -> Self {
        match raw {
            "DraftFolder" => ItemKind::DraftFolder,
            "ResearchFolder" => ItemKind::ResearchFolder,
            "TrashFolder" => ItemKind::TrashFolder,
            "Folder" => ItemKind::Folder,
            "Text" => ItemKind::Text,
            "Image" => ItemKind::Image,
            "PDF" => ItemKind::Pdf,
            "WebArchive" => ItemKind::WebArchive,
            "Media" => ItemKind::Media,
            "Other" => ItemKind::Other,
            other => ItemKind::Unknown(other.to_string()),
        }
    }

    /// Items whose writing is RTF: texts and every kind of folder (folders carry
    /// text too). An unknown kind is treated as one. A text file is looked for
    /// with an item of any kind; this says whether the search index's text and
    /// the checksum list speak for its writing (for a PDF, the index holds the
    /// PDF's own words, not a missing document).
    pub fn holds_text(&self) -> bool {
        matches!(
            self,
            ItemKind::DraftFolder
                | ItemKind::ResearchFolder
                | ItemKind::TrashFolder
                | ItemKind::Folder
                | ItemKind::Text
                | ItemKind::Unknown(_)
        )
    }

    /// Imported files: pictures, PDFs, web archives, sound and anything else.
    pub fn is_media(&self) -> bool {
        matches!(
            self,
            ItemKind::Image
                | ItemKind::Pdf
                | ItemKind::WebArchive
                | ItemKind::Media
                | ItemKind::Other
        )
    }
}

/// An item's `MetaData`, as written, with the generations' conventions settled.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemMeta {
    /// The label id, or `None` for no label (`-1`, or none written).
    pub label: Option<i64>,
    /// The status id, or `None` for no status.
    pub status: Option<i64>,
    /// `IncludeInCompile` is only ever written as `Yes`; an excluded item simply
    /// lacks it.
    pub include_in_compile: bool,
    /// An explicit section type (2.0), its UUID as written.
    pub section_type: Option<String>,
    /// The section type this item's descendants default to (2.0 `ChildDefault`).
    pub section_child_default: Option<String>,
    /// Format 1.x "page break before".
    pub page_break_before: bool,
    /// Format 1.x "compile as-is".
    pub compile_as_is: bool,
    /// Custom metadata values, field id and value as written (a list field's value
    /// is still the option id).
    pub custom: Vec<CustomValue>,
    /// The binder icon, as written: a built-in icon's name, or a file in `Icons/`.
    pub icon: Option<String>,
    /// The extension of a media item's file, checked against the identifier
    /// grammar.
    pub file_extension: Option<String>,
    /// The extension of the index-card image, checked the same way.
    pub card_image_extension: Option<String>,
    /// The index card shows the image instead of the synopsis.
    pub show_synopsis_image: bool,
}

/// One custom metadata value of one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomValue {
    pub field: String,
    pub value: String,
}

/// Where an item's files are. Each is `None` when the file is absent, which for
/// text is normal: an empty document has no file at all.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payload {
    /// The main text (`content.rtf`, `<ID>.rtf`), for an item of any kind: a
    /// picture or research item has one when Scrivener left text with it.
    pub text: Option<MemberRef>,
    /// Document notes (`notes.rtf`, `<ID>_notes.rtf`).
    pub notes: Option<MemberRef>,
    /// The synopsis, plain UTF-8 (`synopsis.txt`, `<ID>_synopsis.txt`).
    pub synopsis: Option<MemberRef>,
    /// Linked comments and footnotes (`content.comments`, `<ID>.comments`).
    pub comments: Option<MemberRef>,
    /// Format 1.0 range-anchored comments, footnotes and links (`<ID>.links`).
    pub links: Option<MemberRef>,
    /// The style table for the text's style markers (`content.styles`).
    pub text_styles: Option<MemberRef>,
    /// The style table for the notes (`notes.styles`).
    pub notes_styles: Option<MemberRef>,
    /// A media item's file (`content.<ext>`, `<ID>.<ext>`).
    pub file: Option<MemberRef>,
    /// The index-card image (`card-image.<ext>`, `<ID>_icImage.<ext>`).
    pub card_image: Option<MemberRef>,
    /// Plain text Scrivener's search index still holds for a file that is missing.
    pub salvaged: Salvaged,
}

/// Text rescued from Scrivener's search index for a file that is missing on disk.
/// Plain text without its formatting, and possibly an older version: the index
/// can lag behind the files, and has been seen holding the template text a
/// writer had replaced.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Salvaged {
    pub text: Option<String>,
    pub notes: Option<String>,
    pub synopsis: Option<String>,
}

/// Which of an item's writings a file holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextRole {
    Text,
    Notes,
    Synopsis,
}

/// A document bookmark, reference or project bookmark.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    /// The item it points at, when it points inside the project.
    pub target: Option<ItemKey>,
    /// A destination outside the project, as written (a URL), never followed.
    /// A file on the writer's own disk (a `file:` URL or a path) keeps its file
    /// name only: the rest names their folders, and the model feeds what the
    /// imported project keeps.
    pub destination: Option<String>,
    /// The destination named a file on the writer's disk, reduced to its name.
    pub local_file: bool,
    /// The description or label, as written.
    pub text: Option<String>,
}

/// A document target (`TextSettings/Target`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocTarget {
    /// `Words` or `Characters`, as written.
    pub kind: Option<String>,
    /// The count, as written.
    pub value: String,
    pub notify: bool,
}

/// A label or status list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Vocabulary {
    /// The field's own name, which writers rename ("Label" to "Time", "Status" to
    /// "Rating").
    pub title: Option<String>,
    /// The default id for new items, as written.
    pub default: Option<i64>,
    /// In menu order, which is not id order.
    pub entries: Vec<VocabEntry>,
}

/// One label or status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VocabEntry {
    pub id: i64,
    pub name: String,
    pub color: Option<Rgb>,
}

/// A colour Scrivener wrote as three floats between 0 and 1.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// One keyword, flattened: its depth says which group it sits in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeywordRow {
    pub depth: u16,
    pub id: i64,
    pub title: Option<String>,
    pub color: Option<Rgb>,
}

/// A custom metadata field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldDef {
    pub id: String,
    pub title: Option<String>,
    /// `Text`, `List`, `Checkbox` or `Date` (2.0); absent in 1.x, where every
    /// field is text.
    pub kind: Option<String>,
    pub wraps: Option<bool>,
    pub align: Option<String>,
    pub color: Option<Rgb>,
    /// A list field's label for "no option".
    pub none_label: Option<String>,
    /// A list field's options, in display order.
    pub options: Vec<ListOption>,
}

/// One option of a list field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListOption {
    pub id: i64,
    pub name: String,
}

/// Section types (format 2.0).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionTypes {
    /// Every type defined, id and name, in order.
    pub definitions: Vec<SectionTypeDef>,
    /// The structure-based defaults, per kind: index 0 is the binder's top level,
    /// and the last entry applies to every deeper level.
    pub folders: Vec<String>,
    pub containers: Vec<String>,
    pub files: Vec<String>,
}

/// One section type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionTypeDef {
    pub id: String,
    pub name: String,
}

/// A collection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    /// `Binder`, `RecentSearch`, `SavedSearch` or `Arbitrary`, as written.
    pub kind: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub color: Option<Rgb>,
    /// Its members, in the collection's own order.
    pub members: Vec<ItemKey>,
}

/// A format 1.x project note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectNote {
    pub id: Option<u32>,
    /// From `index.xml`; `None` for a note file the index does not list.
    pub title: Option<String>,
    pub modified: Option<Stamp>,
    /// `Files/ProjectNotes/Note-<ID>.rtf`, absent for an empty note.
    pub file: Option<MemberRef>,
}

/// The project targets, as written.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectTargets {
    pub draft: Option<RawTarget>,
    pub session: Option<RawTarget>,
}

/// A target element: its count and its attributes, as written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawTarget {
    pub value: String,
    pub attributes: Vec<(String, String)>,
}

/// Format 1.x `ProjectProperties`: the project's title and author, when set.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectProperties {
    pub title: Option<String>,
    pub abbreviated_title: Option<String>,
    pub full_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
}

/// The settings files the metadata step reads (title, author, language live in
/// `compile.xml` in format 2.0).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsFiles {
    pub compile_xml: Option<MemberRef>,
    pub compile_plist: Option<MemberRef>,
    pub compile_ini: Option<MemberRef>,
    pub template_info: Option<MemberRef>,
    pub project_preferences: Option<MemberRef>,
    /// The compile formats saved inside the project (`Settings/Compile Formats/*.scrformat`).
    #[serde(default)]
    pub compile_formats: Vec<MemberRef>,
}

/// What the compile settings state, as written, from whichever of Scrivener's compile
/// files the project holds: `Settings/compile.xml` (format 2.0), `Settings/compile.plist`
/// (Scrivener 2 for Mac) or `Settings/compile.ini` (Scrivener for Windows 1.x and the Linux
/// beta), and the compile formats saved inside the project (`Settings/Compile Formats/`).
///
/// Nothing here is resolved: a placeholder title is still `<$template_projectName>`, the
/// language is still the default Scrivener ships. What the import proposes from them is
/// [`crate::metadata::propose`]'s to decide.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileFacts {
    /// `ProjectSettings/MetaData/ProjectTitle` (format 2.0).
    pub title: Option<String>,
    /// `ProjectSettings/MetaData/ProjectAbbreviatedTitle` (format 2.0).
    pub abbreviated_title: Option<String>,
    /// `ProjectSettings/MetaData/Authors/Author`, in order (format 2.0).
    pub authors: Vec<CompileAuthor>,
    /// `ProjectSettings/MetaData/Surname` and `Forename` (format 2.0).
    pub surname: Option<String>,
    pub forename: Option<String>,
    /// The language a compile writes, from each file that states one, in the order
    /// `compile.xml`, `compile.plist`, `compile.ini`.
    pub languages: Vec<CompileLanguage>,
    /// Whether the compile removes struck-through text: `compile.xml`'s
    /// `DeleteStruckThroughText` when the project has a readable one (absent is off), else
    /// `compile.plist`'s `SCRCompileDeleteStruckThroughTextKey`.
    pub delete_struck_through: bool,
    /// The compile format Scrivener last compiled with, by id: `LastUsedFormats`
    /// under the `CurrentFileType` (format 2.0).
    pub last_format: Option<String>,
    /// The compile formats saved inside the project, in file-name order.
    pub formats: Vec<CompileFormat>,
}

/// One author of a format 2.0 compile's metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileAuthor {
    /// The element's text, as written.
    pub name: String,
    /// `Role` (`aut` for an author), when written.
    pub role: Option<String>,
    /// `FileAs` (`Surname, Forename` for sorting), when written.
    pub file_as: Option<String>,
}

/// A compile's language, as written in one file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileLanguage {
    pub file: CompileFile,
    /// An ISO code (`en`, `fr`) in `compile.xml` and `compile.plist`; a language's
    /// English name (`English`) in `compile.ini`.
    pub value: String,
}

/// Which of Scrivener's compile files a value comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompileFile {
    Xml,
    Plist,
    Ini,
}

/// A compile format saved inside the project, reduced to what the import needs: the styles
/// its compile leaves out or passes through untouched.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileFormat {
    /// `CompileFormat/@ID`.
    pub id: String,
    /// Styles, by name, whose text this format deletes (`DeleteText`).
    pub deleted_styles: Vec<String>,
    /// Styles, by name, whose text this format passes through as raw markup
    /// (`RawMarkup`): meant for another program, never read as prose.
    pub raw_styles: Vec<String>,
}

/// Text found on disk that no binder item references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveredText {
    pub origin: RecoveredOrigin,
    /// Which of an item's writings the file would be.
    pub role: TextRole,
    pub member: MemberRef,
    /// The item the file's name or folder names, when it names one.
    pub names: Option<ItemKey>,
}

/// Why a text counts as recovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveredOrigin {
    /// A sync service's copy of a live item's file: a divergent version of it.
    SyncConflict(ConflictScheme),
    /// A copy whose original is gone: it may be the only copy of that text.
    OrphanCopy(ConflictScheme),
    /// A document file whose item the binder does not list.
    Unreferenced,
}

/// How a sync service or tool named its copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictScheme {
    /// Dropbox: `name (… conflicted copy 2016-03-22).ext`, in any language.
    Dropbox,
    /// `name-conflicted-2018-02-14T09-31-27-0500.ext`.
    IsoConflicted,
    /// `name[Conflict].ext`, `name[Conflict 1].ext`.
    BracketConflict,
    /// Syncthing: `name.sync-conflict-<date>-<time>-<device>.ext`.
    Syncthing,
    /// Resilio: `name.Conflict.ext`.
    Resilio,
    /// A git merge leftover: `name.ext~HEAD`.
    GitMerge,
    /// `name (1).ext`.
    Numbered,
    /// `name_2.ext`.
    Underscore,
    /// `name 2.ext` (iCloud).
    SpaceNumber,
    /// Any other name that starts with a document file's stem inside an item's
    /// folder: OneDrive's `content-MACHINE.rtf`, for one.
    Stem,
    /// An item's own file name spelled otherwise (in case, Unicode form or
    /// leading zeros) beside the one read: two copies merged onto a disk that
    /// tells the spellings apart.
    Spelling,
}

/// Counts the report needs about what the project holds.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    /// Binder items.
    pub items: u32,
    /// Items of kind Text.
    pub text_items: u32,
    /// Items with a main text file.
    pub texts_present: u32,
    /// Folders with a text of their own.
    pub folders_with_text: u32,
    /// Media items (pictures, PDFs, web archives, sound, other files).
    pub media_items: u32,
    /// Media items whose file is missing.
    pub media_missing: u32,
    /// Research files present that are not pictures (PDFs, web archives, sound,
    /// anything else): what is copied beside the imported project.
    pub research_files: u32,
    /// Items in the Trash, at any depth.
    pub trashed_items: u32,
    /// Snapshots Scrivener keeps, over every item.
    pub snapshots: u32,
    /// Items with at least one snapshot.
    pub items_with_snapshots: u32,
    /// Snapshot folders naming no item in the binder.
    pub orphan_snapshot_folders: u32,
    /// Snapshots in those folders: kept by Scrivener for documents since deleted.
    #[serde(default)]
    pub orphan_snapshots: u32,
    /// Files, by count, that no item references and that are not text (pictures
    /// and other research left behind by deleted items).
    pub unreferenced_files: u32,
    /// Empty document files no item references (counted, never carried).
    pub unreferenced_empty_files: u32,
}

impl Project {
    /// The index of each row's parent row, or `None` at the top level.
    ///
    /// A row's parent is the nearest row above it one level shallower. Computed
    /// with one stack of open ancestors, never by recursion.
    pub fn parents(&self) -> Vec<Option<usize>> {
        parents_of(self.binder.iter().map(|row| row.depth))
    }

    /// Refuse a model whose depths do not describe a tree: the first row deeper
    /// than the top level, or a row more than one level deeper than the row above
    /// it. A reader never builds one; a model deserialised from elsewhere might.
    pub fn check_shape(&self) -> Result<(), ShapeError> {
        check_depths(self.binder.iter().map(|row| row.depth))
            .map_err(|row| ShapeError::Binder { row })?;
        check_depths(self.keywords.iter().map(|row| row.depth))
            .map_err(|row| ShapeError::Keywords { row })
    }

    /// The row an item key names, when an item owns it.
    pub fn row_of(&self, key: ItemKey) -> Option<&BinderRow> {
        self.binder
            .iter()
            .find(|row| row.owns_key && row.key == Some(key))
    }

    /// How the writer knew each thing of the project a notice can name ([`SourceNames`]).
    /// One walk over the flat binder with a stack of open ancestors, never recursion.
    pub fn source_names(&self) -> SourceNames {
        let mut open: Vec<(u16, String)> = Vec::new();
        let mut items = Vec::with_capacity(self.binder.len());
        for row in &self.binder {
            while open.last().is_some_and(|(depth, _)| *depth >= row.depth) {
                open.pop();
            }
            open.push((row.depth, row.title.clone().unwrap_or_default()));
            items.push(open.iter().map(|(_, title)| title.clone()).collect());
        }
        let mut names = SourceNames {
            items,
            keys: self.binder.iter().map(|row| row.key).collect(),
            owns_key: self.binder.iter().map(|row| row.owns_key).collect(),
            project_notes: self
                .project_notes
                .iter()
                .map(|note| note.title.clone().unwrap_or_default())
                .collect(),
            recovered: Vec::with_capacity(self.recovered.len()),
        };
        for text in &self.recovered {
            let path = text
                .names
                .and_then(|key| names.item_by_key(key, true))
                .and_then(|item| names.items.get(item as usize).cloned())
                .unwrap_or_else(|| {
                    // The number the file's name gives, as the recovered row is titled,
                    // else nothing: never the name itself, which in a sync service's copy
                    // carries a person's or a computer's name.
                    let number = crate::names::recovered_number(&text.member.path);
                    if number.is_empty() {
                        Vec::new()
                    } else {
                        vec![number]
                    }
                });
            names.recovered.push(path);
        }
        names
    }
}

/// How the writer knew each thing of a project a notice can name: where they will recognise
/// it (`Draft › Part One › Scene 3`), never where Scrivener keeps it
/// (`Files/Data/<UUID>/content.rtf`). A report names an item by these; nothing here is a
/// sentence, and nothing personal is in it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceNames {
    /// One per binder row, in the binder's order: the titles from the top of Scrivener's
    /// binder down to the item's own. An untitled item's title is empty.
    pub items: Vec<Vec<String>>,
    /// Each binder row's key, as the binder gives it.
    pub keys: Vec<Option<ItemKey>>,
    /// Whether each binder row owns its key (an earlier row may already carry it).
    pub owns_key: Vec<bool>,
    /// One per format 1.x project note: its title, empty when the notes' index gives none.
    pub project_notes: Vec<String>,
    /// One per recovered text: the path of the item its file names, else the Scrivener
    /// document number its file's name gives, which is the title of the row that holds it,
    /// else nothing. Never the file's own name, which a sync service writes a person's or a
    /// computer's name into.
    pub recovered: Vec<Vec<String>>,
}

impl SourceNames {
    /// The binder row carrying `key`: the one that owns it when `owner`, else one that
    /// carries it without owning it; either way, any row carrying it when no row fits.
    pub fn item_by_key(&self, key: ItemKey, owner: bool) -> Option<u32> {
        let carries = |i: &usize| self.keys.get(*i).copied().flatten() == Some(key);
        (0..self.keys.len())
            .filter(carries)
            .find(|i| self.owns_key.get(*i).copied().unwrap_or(false) == owner)
            .or_else(|| (0..self.keys.len()).find(carries))
            .map(|i| i as u32)
    }
}

/// Why a flat list of rows does not describe a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeError {
    /// The binder row at this index jumps more than one level deeper.
    Binder { row: usize },
    /// The keyword row at this index does.
    Keywords { row: usize },
}

impl std::fmt::Display for ShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeError::Binder { row } => write!(f, "binder row {row} jumps more than one level"),
            ShapeError::Keywords { row } => {
                write!(f, "keyword row {row} jumps more than one level")
            }
        }
    }
}

impl std::error::Error for ShapeError {}

/// Parent indices for a sequence of depths.
pub(crate) fn parents_of(depths: impl Iterator<Item = u16>) -> Vec<Option<usize>> {
    let mut open: Vec<(u16, usize)> = Vec::new();
    let mut parents = Vec::new();
    for (index, depth) in depths.enumerate() {
        while open.last().is_some_and(|(d, _)| *d >= depth) {
            open.pop();
        }
        parents.push(open.last().map(|(_, i)| *i));
        open.push((depth, index));
    }
    parents
}

/// The index of the first depth that does not follow from the one before it.
fn check_depths(depths: impl Iterator<Item = u16>) -> Result<(), usize> {
    let mut previous: Option<u16> = None;
    for (index, depth) in depths.enumerate() {
        let allowed = previous.map_or(0, |p| p.saturating_add(1));
        if depth > allowed {
            return Err(index);
        }
        previous = Some(depth);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parents_follow_the_depths() {
        let depths = [0u16, 1, 2, 2, 1, 0, 1];
        assert_eq!(
            parents_of(depths.into_iter()),
            vec![None, Some(0), Some(1), Some(1), Some(0), None, Some(5)]
        );
    }

    #[test]
    fn a_jump_of_two_levels_is_not_a_tree() {
        assert_eq!(check_depths([0u16, 1, 3].into_iter()), Err(2));
        assert_eq!(check_depths([1u16].into_iter()), Err(0));
        assert_eq!(check_depths([0u16, 1, 2, 0, 1].into_iter()), Ok(()));
    }
}
