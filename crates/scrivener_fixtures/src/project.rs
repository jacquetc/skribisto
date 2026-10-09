// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A synthetic project of each generation, written in the layout that generation
//! writes.
//!
//! A [`Spec`] describes the project: its generation and platform, its binder as a
//! flat list of items each with its depth, their text, notes and synopses, and the
//! project-level blocks. [`Spec::build`] writes it: the manifest with the element
//! and attribute names that generation uses (and only those: the shape fence in
//! `tests/shape.rs` checks every one against the census of real projects), each
//! item's files where that generation keeps them, and Scrivener's own
//! bookkeeping (version stamp, search index, checksum list, spare binders, lock).
//!
//! Identifiers are deterministic: UUIDs from a seed and the item's position,
//! numbers as Scrivener gives them (the Draft 0, Research 1, Trash 2, then in
//! order). Dates are fixed. The same spec always builds the same bytes.

use crate::files::Files;
use crate::rtf::{self, Dialect};
use crate::{sha1, text, xml_escape};

/// Which generation of Scrivener wrote the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generation {
    /// Scrivener 1 for Mac: `binder.scrivproj`, refused by the importer.
    ScrivenerOne,
    /// Format 1.0: Mac 2.0 to 2.6, Windows 1.0 to 1.8, the Linux beta to 1.7.
    Format10,
    /// Format 1.5: Mac 2.7 and 2.8, Windows 1.9, the Linux beta 1.9.
    Format15,
    /// Format 2.0: Scrivener 3.
    Format20,
    /// A project created on an iPhone or iPad, never opened on a desktop.
    IosStub,
}

/// The platform that wrote it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Mac,
    Windows,
    Linux,
    Ios,
}

/// A binder item's type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Draft,
    Research,
    Trash,
    Folder,
    Text,
    Image,
    Pdf,
    Other,
}

impl Kind {
    fn type_name(&self) -> &'static str {
        match self {
            Kind::Draft => "DraftFolder",
            Kind::Research => "ResearchFolder",
            Kind::Trash => "TrashFolder",
            Kind::Folder => "Folder",
            Kind::Text => "Text",
            Kind::Image => "Image",
            Kind::Pdf => "PDF",
            Kind::Other => "Other",
        }
    }
}

/// One binder item.
#[derive(Debug, Clone)]
pub struct Item {
    pub depth: u16,
    pub kind: Kind,
    pub title: Option<String>,
    /// Paragraphs; none means the document is empty and has no file.
    pub text: Vec<String>,
    pub notes: Vec<String>,
    pub synopsis: Option<String>,
    /// A media item's extension and bytes.
    pub media: Option<(String, Vec<u8>)>,
    pub label: Option<i64>,
    pub status: Option<i64>,
    pub include_in_compile: bool,
    pub keywords: Vec<i64>,
    /// Snapshots Scrivener took of the item.
    pub snapshots: u32,
    /// Linked comments on the text, in its sidecar.
    pub comments: u32,
    /// An explicit section type (format 2.0), by index into the spec's types.
    pub section_type: Option<usize>,
    /// Custom metadata values: field id, value.
    pub custom: Vec<(String, String)>,
    /// A document target, in words unless `target_kind` names another unit.
    pub target: Option<u32>,
    /// The document target's `Type` (`Words`, `Characters`); `None` writes `Words`.
    pub target_kind: Option<String>,
    /// Document bookmarks (format 2.0) or references (format 1.x).
    pub bookmarks: Vec<Bookmark>,
    /// Written in script mode (`TextMode` = `Script`; formats 1.5 and 2.0).
    pub script_mode: bool,
    /// `Created`, as written; `None` writes the fixed [`DATE`].
    pub created: Option<String>,
    /// `Modified`, as written; `None` writes the fixed [`DATE`].
    pub modified: Option<String>,
    /// The binder icon (`IconFileName`), a built-in icon's name.
    pub icon: Option<String>,
}

impl Item {
    pub fn new(depth: u16, kind: Kind, title: &str) -> Self {
        Item {
            depth,
            kind,
            title: Some(title.to_string()),
            text: Vec::new(),
            notes: Vec::new(),
            synopsis: None,
            media: None,
            label: None,
            status: None,
            include_in_compile: false,
            keywords: Vec::new(),
            snapshots: 0,
            comments: 0,
            section_type: None,
            custom: Vec::new(),
            target: None,
            target_kind: None,
            bookmarks: Vec::new(),
            script_mode: false,
            created: None,
            modified: None,
            icon: None,
        }
    }

    /// A text of `paragraphs` invented paragraphs.
    pub fn with_text(mut self, seed: usize, paragraphs: usize) -> Self {
        self.text = text::paragraphs(seed, paragraphs, 3);
        self
    }

    pub fn with_notes(mut self, seed: usize) -> Self {
        self.notes = text::paragraphs(seed, 1, 2);
        self
    }

    pub fn with_synopsis(mut self, seed: usize) -> Self {
        self.synopsis = Some(text::sentence(seed).to_string());
        self
    }

    pub fn compiled(mut self) -> Self {
        self.include_in_compile = true;
        self
    }
}

/// A bookmark (format 2.0) or a reference (format 1.x): to an item of the project, or to a
/// destination outside it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bookmark {
    /// The item it points at, by index.
    pub target: Option<usize>,
    /// It points at an item the binder does not hold.
    pub dangling: bool,
    /// A URL, as written.
    pub destination: Option<String>,
    /// A format 1.x reference's description, or a bookmark's label.
    pub text: Option<String>,
}

impl Bookmark {
    /// A bookmark to item `index`.
    pub fn to(index: usize) -> Self {
        Bookmark {
            target: Some(index),
            ..Bookmark::default()
        }
    }

    /// A bookmark to a web page.
    pub fn web(url: &str, label: Option<&str>) -> Self {
        Bookmark {
            destination: Some(url.to_string()),
            text: label.map(str::to_string),
            ..Bookmark::default()
        }
    }
}

/// A custom metadata field of a type format 2.0 writes: a list or a checkbox.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedField {
    pub id: String,
    pub title: String,
    /// `List` or `Checkbox`.
    pub kind: String,
    /// A list's options, id and name, in display order.
    pub options: Vec<(i64, String)>,
    /// A list's name for "no option".
    pub none: Option<String>,
    /// A checkbox's `Default`, as written (`Yes` on Scrivener 3 for Mac when it is ticked
    /// by default); `None` writes no attribute.
    pub default: Option<String>,
}

/// A collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionSpec {
    /// `Binder`, `RecentSearch`, `SavedSearch` or `Arbitrary`.
    pub kind: String,
    pub title: String,
    /// Its members, by item index.
    pub members: Vec<usize>,
    /// A search's words, `Type`, `Operator` and `CustomIDs` (format 2.0 for a saved one).
    pub search: Option<(String, String, String, Option<String>)>,
}

/// The project targets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Targets {
    pub draft: u32,
    /// `Words`, `Characters`.
    pub draft_kind: String,
    pub deadline: Option<String>,
    pub ignore_deadline: bool,
    pub count_included_only: bool,
    pub compile_group_only: bool,
    pub session: u32,
    pub session_kind: String,
    /// `WritingDays`, as written.
    pub writing_days: String,
    pub can_write_on_deadline: bool,
    /// `DeterminedFromDeadline`: Scrivener calculates the session target from the deadline
    /// and the writing days.
    pub determined_from_deadline: bool,
}

impl Default for Targets {
    /// What a novel's targets hold: an 80,000-word draft target with its deadline ignored,
    /// a 1,000-word session target, every day a writing day.
    fn default() -> Self {
        Targets {
            draft: 80_000,
            draft_kind: "Words".to_string(),
            deadline: Some(LATER.to_string()),
            ignore_deadline: true,
            count_included_only: true,
            compile_group_only: false,
            session: 1_000,
            session_kind: "Words".to_string(),
            writing_days: String::new(),
            can_write_on_deadline: false,
            determined_from_deadline: false,
        }
    }
}

/// A compile replacement (format 2.0 `compile.xml`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replacement {
    pub replace: String,
    pub with: String,
    pub regex: bool,
    pub ignore: bool,
}

/// Scrivener's spare binder copy, `Files/binder.backup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backup {
    None,
    /// A copy of the manifest.
    Copy,
    /// The empty `NewProject="Yes"` placeholder real projects carry.
    Placeholder,
}

/// A whole project.
#[derive(Debug, Clone)]
pub struct Spec {
    /// The project folder's stem: the folder is `<name>.scriv`.
    pub name: String,
    /// The manifest's stem, when it is not the folder's.
    pub manifest_stem: Option<String>,
    pub generation: Generation,
    pub platform: Platform,
    pub seed: u32,
    pub items: Vec<Item>,
    pub labels: Vec<(i64, String)>,
    pub statuses: Vec<(i64, String)>,
    /// Keywords: depth, id, title.
    pub keywords: Vec<(u16, i64, String)>,
    /// Text custom fields: id, title.
    pub custom_fields: Vec<(String, String)>,
    /// Section types (format 2.0): names; ids are derived.
    pub section_types: Vec<String>,
    /// The Document Templates folder, by item index.
    pub template_folder: Option<usize>,
    pub version_txt: bool,
    pub search_index: bool,
    pub checksum: bool,
    pub autosave: bool,
    pub backup: Backup,
    /// A lock file naming this host.
    pub lock_host: Option<String>,
    pub settings: bool,
    /// Format 1.x project notes.
    pub project_notes: bool,
    /// The label field's title: `Label` unless the writer renamed it.
    pub label_title: String,
    /// The status field's title: `Status` unless the writer renamed it.
    pub status_title: String,
    /// Format 1.x `ProjectProperties`, when set.
    pub properties: Option<Properties>,
    /// The compile settings, written in the generation's own file (`settings` on).
    pub compile: Compile,
    /// The days of `Files/writing.history` (format 2.0), as written.
    pub writing_days: Vec<String>,
    /// List and checkbox fields (format 2.0).
    pub typed_fields: Vec<TypedField>,
    /// The collections; `None` writes the novel's (the binder, and one holding the first
    /// two texts).
    pub collections: Option<Vec<CollectionSpec>>,
    pub targets: Targets,
    /// Project bookmarks (format 2.0).
    pub project_bookmarks: Vec<Bookmark>,
    /// Favourites, by item index (format 1.x).
    pub favorites: Vec<usize>,
    /// The auto-complete list (format 2.0), as written.
    pub auto_complete: Vec<String>,
}

/// Format 1.x `ProjectProperties`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Properties {
    pub title: Option<String>,
    pub abbreviated_title: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// One author of a format 2.0 compile's metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: String,
    pub role: Option<String>,
    pub file_as: Option<String>,
}

/// A compile format saved inside a format 2.0 project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Format {
    /// Its name, which is also its file's name.
    pub name: String,
    pub id: String,
    /// Styles, by name, its compile deletes.
    pub deleted: Vec<String>,
    /// Styles, by name, it passes through as raw markup.
    pub raw: Vec<String>,
}

/// Compile settings. Each generation writes what it keeps: format 2.0 `Settings/compile.xml`
/// (title, authors, names, language, the struck-through switch, the last format used) and
/// its compile formats; Scrivener 2 for Mac `Settings/compile.plist` (language, switch);
/// Scrivener for Windows 1.x and the Linux beta `Settings/compile.ini` (language, as a
/// language's English name).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Compile {
    pub title: Option<String>,
    pub authors: Vec<Author>,
    pub surname: Option<String>,
    pub forename: Option<String>,
    /// `None` writes the language Scrivener ships (`en`, or `English` in `compile.ini`).
    pub language: Option<String>,
    pub delete_struck_through: bool,
    pub formats: Vec<Format>,
    /// The format last compiled to rich text, by id.
    pub last_format: Option<String>,
    /// Format 2.0's `ProjectAbbreviatedTitle`, `Subjects` and `Company`.
    pub abbreviated_title: Option<String>,
    pub subjects: Vec<String>,
    pub company: Option<String>,
    /// Format 2.0's project replacements.
    pub replacements: Vec<Replacement>,
    /// Scrivener 2 for Mac's variable replacements (`compile.plist`): variable, replacement.
    pub variable_replacements: Vec<(String, String)>,
}

/// A fixed date in Scrivener's form.
pub const DATE: &str = "2024-03-14 10:15:00 +0100";
/// A later one.
pub const LATER: &str = "2024-06-02 18:40:12 +0200";

impl Spec {
    /// A small novel: a Draft with a part, two chapters and four scenes (one
    /// empty), Research with a picture, a PDF and a character sheet, a Trash
    /// holding a deleted scene, a notes folder and a Document Templates folder.
    pub fn novel(generation: Generation, platform: Platform) -> Self {
        let items = vec![
            Item::new(0, Kind::Draft, "Manuscript").compiled(),
            Item::new(1, Kind::Folder, "Part One").compiled(),
            Item::new(2, Kind::Folder, "Chapter 1")
                .compiled()
                .with_text(40, 1),
            Item {
                label: Some(1),
                status: Some(2),
                keywords: vec![2],
                snapshots: 2,
                comments: 1,
                section_type: Some(1),
                custom: vec![("setting".to_string(), "Harbour".to_string())],
                target: Some(1500),
                ..Item::new(3, Kind::Text, "Arrival")
                    .compiled()
                    .with_text(0, 3)
                    .with_notes(5)
                    .with_synopsis(7)
            },
            Item::new(3, Kind::Text, "The Harbour")
                .compiled()
                .with_text(3, 2),
            Item::new(2, Kind::Folder, "Chapter 2").compiled(),
            Item {
                keywords: vec![2, 3],
                ..Item::new(3, Kind::Text, "Night Watch")
                    .compiled()
                    .with_text(6, 2)
                    .with_synopsis(1)
            },
            Item {
                title: None,
                ..Item::new(3, Kind::Text, "").compiled()
            },
            Item::new(0, Kind::Research, "Research"),
            Item {
                media: Some(("png".to_string(), crate::TINY_PNG.to_vec())),
                ..Item::new(1, Kind::Image, "Harbour map")
            },
            Item {
                media: Some(("pdf".to_string(), crate::TINY_PDF.to_vec())),
                ..Item::new(1, Kind::Pdf, "Tide tables")
            },
            Item::new(1, Kind::Text, "Mara")
                .with_text(9, 1)
                .with_notes(10),
            Item::new(0, Kind::Folder, "Notes"),
            Item::new(1, Kind::Text, "Ideas").with_text(11, 1),
            Item::new(0, Kind::Folder, "Templates"),
            Item::new(1, Kind::Text, "Scene sheet").with_text(2, 1),
            Item::new(0, Kind::Trash, "Trash"),
            Item::new(1, Kind::Text, "Deleted scene").with_text(4, 1),
        ];
        // The one project made on an iPad that real evidence shows holds no
        // document notes and no research files; the fixture holds none either,
        // rather than a shape no real project was seen with.
        let items: Vec<Item> = if generation == Generation::IosStub {
            items
                .into_iter()
                .filter(|item| item.media.is_none())
                .map(|item| Item {
                    notes: Vec::new(),
                    ..item
                })
                .collect()
        } else {
            items
        };
        let mut spec = Spec {
            name: "Lighthouse".to_string(),
            manifest_stem: None,
            generation,
            platform: match generation {
                Generation::IosStub => Platform::Ios,
                _ => platform,
            },
            seed: 0x5C1F_0001,
            items,
            labels: vec![
                (-1, "No Label".to_string()),
                (1, "Mara".to_string()),
                (2, "Keeper".to_string()),
            ],
            statuses: vec![
                (-1, "No Status".to_string()),
                (1, "To Do".to_string()),
                (2, "First Draft".to_string()),
                (3, "Final".to_string()),
            ],
            keywords: vec![
                (0, 1, "People".to_string()),
                (1, 2, "Mara".to_string()),
                (1, 3, "The inspector".to_string()),
            ],
            custom_fields: vec![("setting".to_string(), "Setting".to_string())],
            section_types: vec!["Chapter Heading".to_string(), "Scene".to_string()],
            template_folder: None,
            version_txt: generation != Generation::IosStub,
            search_index: true,
            checksum: true,
            autosave: generation != Generation::IosStub,
            backup: if generation == Generation::IosStub {
                Backup::None
            } else {
                Backup::Copy
            },
            lock_host: None,
            settings: true,
            project_notes: matches!(generation, Generation::Format10 | Generation::Format15),
            label_title: "Label".to_string(),
            status_title: "Status".to_string(),
            properties: None,
            compile: Compile::default(),
            writing_days: vec!["2024-03-14".to_string()],
            typed_fields: Vec::new(),
            collections: None,
            targets: Targets::default(),
            project_bookmarks: Vec::new(),
            favorites: Vec::new(),
            auto_complete: Vec::new(),
        };
        spec.template_folder = spec.index_of("Templates");
        spec
    }

    /// The same project with everything the metadata mapping reads set: a stated title
    /// and authors (format 2.0 compile settings, format 1.x project properties), a
    /// language the writer chose, a compile that removes struck-through text, a compile
    /// format leaving a style out (format 2.0), dates written in other time zones (one
    /// just after midnight east of Greenwich, one unreadable), labels and statuses worn,
    /// and a writing history of several days.
    pub fn with_metadata(mut self) -> Self {
        let g = self.generation;
        self.compile = Compile {
            title: Some("The Lighthouse Keepers".to_string()),
            authors: vec![Author {
                name: "Holt, Mara".to_string(),
                role: Some("aut".to_string()),
                file_as: None,
            }],
            surname: Some("Holt".to_string()),
            forename: Some("Mara".to_string()),
            language: Some(match (g, self.platform) {
                (
                    Generation::Format10 | Generation::Format15,
                    Platform::Windows | Platform::Linux,
                ) => "French".to_string(),
                _ => "fr".to_string(),
            }),
            delete_struck_through: true,
            formats: vec![Format {
                name: "Manuscript".to_string(),
                id: uuid_for(self.seed ^ 0xF0, 1),
                deleted: vec!["Private Note".to_string()],
                raw: Vec::new(),
            }],
            last_format: Some(uuid_for(self.seed ^ 0xF0, 1)),
            ..Compile::default()
        };
        if matches!(g, Generation::Format10 | Generation::Format15) {
            self.properties = Some(Properties {
                title: Some("The Lighthouse Keepers".to_string()),
                abbreviated_title: None,
                full_name: Some("Mara Holt".to_string()),
                first_name: None,
                last_name: None,
            });
        }
        self.labels = vec![
            (-1, "No Label".to_string()),
            (0, "Scene".to_string()),
            (1, "Mara".to_string()),
            (2, "Keeper".to_string()),
            (3, "Unused".to_string()),
        ];
        self.statuses = vec![
            (-1, "No Status".to_string()),
            (1, "To Do".to_string()),
            (2, "First Draft".to_string()),
            (3, "Revised Draft".to_string()),
            (4, "Done".to_string()),
        ];
        self.writing_days = vec![
            "2024-03-14".to_string(),
            "2024-03-02".to_string(),
            "2024-04-01".to_string(),
            "not a day".to_string(),
        ];
        for (title, label, status, created, modified) in [
            (
                "Arrival",
                Some(1),
                Some(2),
                Some("2024-03-14 00:30:00 +0200"),
                Some("2024-06-02 18:40:12 -0700"),
            ),
            (
                "The Harbour",
                Some(0),
                Some(4),
                None,
                Some("2024-06-31 10:00:00 +0200"),
            ),
            ("Chapter 1", Some(2), Some(3), None, None),
            ("Mara", Some(1), None, None, None),
        ] {
            if let Some(index) = self.index_of(title) {
                let item = &mut self.items[index];
                item.label = label;
                item.status = status;
                item.created = created.map(str::to_string);
                item.modified = modified.map(str::to_string);
            }
        }
        self
    }

    /// The same project with every piece of metadata the second half of the mapping reads
    /// set, as far as the generation writes it: a story-bible folder of two entries, nested
    /// keywords (one unused), text fields with a value on a scene, a note and a document
    /// template, list and checkbox fields (format 2.0), collections of every kind (a saved
    /// search in format 2.0), bookmarks and references of every kind (to a note, to a
    /// story-bible entry, to the web, to itself, to nothing, to a template), project
    /// bookmarks (format 2.0) or favourites (format 1.x), document targets on a scene and on
    /// notes, a future deadline Scrivener would plan towards, an auto-complete list (format
    /// 2.0), compile replacements, an abbreviated title, subjects and a company, icons, and a
    /// document in script mode (formats 1.5 and 2.0).
    pub fn with_s2(mut self) -> Self {
        let g = self.generation;
        let v3 = g == Generation::Format20;
        let x = matches!(g, Generation::Format10 | Generation::Format15);
        // A story-bible folder before the Trash.
        if let Some(trash) = self.items.iter().position(|i| i.kind == Kind::Trash) {
            let bible = [
                Item::new(0, Kind::Folder, "Characters"),
                Item::new(1, Kind::Text, "Mara Holt").with_text(12, 1),
                Item::new(1, Kind::Text, "The Inspector").with_text(13, 1),
            ];
            for (k, item) in bible.into_iter().enumerate() {
                self.items.insert(trash + k, item);
            }
        }
        let at = |spec: &Spec, title: &str| spec.index_of(title).unwrap_or(0);
        let (arrival, harbour, night, mara, ideas, sheet, inspector, holt) = (
            at(&self, "Arrival"),
            at(&self, "The Harbour"),
            at(&self, "Night Watch"),
            at(&self, "Mara"),
            at(&self, "Ideas"),
            at(&self, "Scene sheet"),
            at(&self, "The Inspector"),
            at(&self, "Mara Holt"),
        );
        self.keywords = vec![
            (0, 1, "People".to_string()),
            (1, 2, "Mara".to_string()),
            (1, 3, "The inspector".to_string()),
            (0, 4, "Places".to_string()),
            (1, 5, "Harbour town".to_string()),
            (0, 6, "Unused".to_string()),
        ];
        self.custom_fields = vec![
            ("setting".to_string(), "Setting".to_string()),
            ("eyes".to_string(), "Eyes".to_string()),
        ];
        if v3 {
            self.typed_fields = vec![
                TypedField {
                    id: "platform".to_string(),
                    title: "Platform".to_string(),
                    kind: "List".to_string(),
                    options: vec![(14, "Mac".to_string()), (0, "Windows".to_string())],
                    none: Some("All".to_string()),
                    default: None,
                },
                TypedField {
                    id: "done".to_string(),
                    title: "Done".to_string(),
                    kind: "Checkbox".to_string(),
                    options: Vec::new(),
                    none: None,
                    default: None,
                },
            ];
        }
        let description = |text: &str| x.then(|| text.to_string());
        for (index, edit) in [
            (arrival, 0usize),
            (harbour, 1),
            (night, 2),
            (mara, 3),
            (ideas, 4),
            (sheet, 5),
        ] {
            let item = &mut self.items[index];
            match edit {
                0 => {
                    item.custom.push(("platform".to_string(), "14".to_string()));
                    item.custom.push(("done".to_string(), "No".to_string()));
                    item.bookmarks = vec![
                        Bookmark {
                            text: description("Where she grew up"),
                            ..Bookmark::to(mara)
                        },
                        Bookmark::to(inspector),
                        Bookmark::web("https://example.org/tides", Some("Tide chart")),
                        Bookmark::to(arrival),
                        Bookmark {
                            dangling: true,
                            ..Bookmark::default()
                        },
                        Bookmark::to(sheet),
                    ];
                    item.icon = Some("Book (Blue Book)".to_string());
                }
                1 => {
                    item.keywords = vec![5];
                    item.custom = vec![
                        ("setting".to_string(), "Quay: 10:30:45".to_string()),
                        ("platform".to_string(), "0".to_string()),
                        ("done".to_string(), "Yes".to_string()),
                    ];
                    item.target = Some(900);
                    item.icon = Some("Book (Blue Book)".to_string());
                }
                2 => {
                    item.custom.push(("platform".to_string(), "-1".to_string()));
                    item.script_mode = true;
                }
                3 => {
                    item.custom.push((
                        "eyes".to_string(),
                        "Grey\nAt 10:30:45 she said \"no\"".to_string(),
                    ));
                    item.bookmarks = vec![
                        Bookmark::to(ideas),
                        Bookmark::web("https://example.org/map", Some("Map")),
                    ];
                    item.target = Some(600);
                }
                4 => {
                    item.custom = vec![
                        ("platform".to_string(), "99".to_string()),
                        ("ghost".to_string(), "Boo".to_string()),
                    ];
                    item.target = Some(2000);
                    item.target_kind = Some("Characters".to_string());
                    item.icon = Some("To Do (Ticked)".to_string());
                }
                _ => {
                    item.keywords = vec![3];
                    item.custom = vec![("setting".to_string(), "Anywhere".to_string())];
                    item.bookmarks = vec![Bookmark::to(ideas)];
                    item.target = Some(100);
                }
            }
        }
        // The text and the platform values only where the generation writes them.
        if !v3 {
            for item in &mut self.items {
                item.custom
                    .retain(|(field, _)| field != "platform" && field != "done");
            }
        }
        let mut collections = vec![
            CollectionSpec {
                kind: "Binder".to_string(),
                title: "Binder".to_string(),
                members: Vec::new(),
                search: None,
            },
            CollectionSpec {
                kind: "Arbitrary".to_string(),
                title: "To revise".to_string(),
                members: vec![arrival, harbour],
                search: None,
            },
            CollectionSpec {
                kind: "Arbitrary".to_string(),
                title: "Cast".to_string(),
                members: vec![holt],
                search: None,
            },
            CollectionSpec {
                kind: "Arbitrary".to_string(),
                title: "Empty".to_string(),
                members: Vec::new(),
                search: None,
            },
            CollectionSpec {
                kind: "Arbitrary".to_string(),
                title: "Templates only".to_string(),
                members: vec![sheet],
                search: None,
            },
            CollectionSpec {
                kind: "RecentSearch".to_string(),
                title: "Search Results".to_string(),
                members: Vec::new(),
                search: Some((
                    "harbour".to_string(),
                    "All".to_string(),
                    "Any".to_string(),
                    None,
                )),
            },
        ];
        if v3 {
            collections.push(CollectionSpec {
                kind: "SavedSearch".to_string(),
                title: "Settings".to_string(),
                members: Vec::new(),
                search: Some((
                    "Quay".to_string(),
                    "CustomMetaData".to_string(),
                    "Exact".to_string(),
                    Some("setting".to_string()),
                )),
            });
            self.project_bookmarks = vec![
                Bookmark::to(ideas),
                Bookmark::web("https://example.org", Some("Site")),
            ];
            self.auto_complete = vec![
                "Mara".to_string(),
                "Mara".to_string(),
                "LUKE".to_string(),
                " Harbour ".to_string(),
                "Harbour master".to_string(),
                String::new(),
            ];
            self.compile.abbreviated_title = Some("Keepers".to_string());
            self.compile.subjects = vec!["Sea".to_string(), "Lights".to_string()];
            self.compile.company = Some("Harbour Press".to_string());
            self.compile.replacements = vec![
                Replacement {
                    replace: r"\s+\[@".to_string(),
                    with: "[@".to_string(),
                    regex: true,
                    ignore: true,
                },
                Replacement {
                    replace: "{Name}".to_string(),
                    with: "Mara".to_string(),
                    regex: false,
                    ignore: false,
                },
            ];
        }
        if x {
            self.favorites = vec![ideas];
            let mut properties = self.properties.clone().unwrap_or_default();
            properties.abbreviated_title = Some("Keepers".to_string());
            self.properties = Some(properties);
            if self.platform == Platform::Mac {
                self.compile.variable_replacements =
                    vec![("!fig($@)".to_string(), "<$n:figure:$@>".to_string())];
            }
        }
        self.collections = Some(collections);
        self.targets = Targets {
            deadline: Some("2099-12-01 10:00:00 +0100".to_string()),
            ignore_deadline: false,
            ..Targets::default()
        };
        self
    }

    /// The index of the item titled `title`.
    pub fn index_of(&self, title: &str) -> Option<usize> {
        self.items
            .iter()
            .position(|i| i.title.as_deref() == Some(title))
    }

    /// The folder name: `<name>.scriv`.
    pub fn folder(&self) -> String {
        format!("{}.scriv", self.name)
    }

    /// The manifest's file name.
    pub fn manifest_name(&self) -> String {
        format!(
            "{}.scrivx",
            self.manifest_stem.as_deref().unwrap_or(&self.name)
        )
    }

    /// Item `index`'s UUID, upper-case as Scrivener writes it.
    pub fn uuid(&self, index: usize) -> String {
        uuid_for(self.seed, index as u32 + 1)
    }

    /// Item `index`'s number: Draft 0, Research 1, Trash 2, then in order.
    pub fn number(&self, index: usize) -> u32 {
        let mut next = 3;
        for (i, item) in self.items.iter().enumerate() {
            let n = match item.kind {
                Kind::Draft => 0,
                Kind::Research => 1,
                Kind::Trash => 2,
                _ => {
                    next += 1;
                    next - 1
                }
            };
            if i == index {
                return n;
            }
        }
        0
    }

    /// Where item `index`'s files are, relative to the project folder, with the
    /// file name to follow: `Files/Data/<UUID>/` or `Files/Docs/<ID>`.
    pub fn payload_prefix(&self, index: usize) -> String {
        match self.generation {
            Generation::Format20 => format!("Files/Data/{}/", self.uuid(index)),
            Generation::IosStub => format!("Mobile/Data/{}/", self.uuid(index)),
            Generation::Format10 | Generation::Format15 => {
                format!("Files/Docs/{}", self.number(index))
            }
            Generation::ScrivenerOne => format!("{}", self.number(index)),
        }
    }

    /// Where item `index`'s main text is.
    pub fn text_path(&self, index: usize) -> String {
        match self.generation {
            Generation::Format20 | Generation::IosStub => {
                format!("{}content.rtf", self.payload_prefix(index))
            }
            Generation::Format10 | Generation::Format15 => {
                format!("{}.rtf", self.payload_prefix(index))
            }
            Generation::ScrivenerOne => format!("{}.rtfd/TXT.rtf", self.number(index)),
        }
    }

    fn dialect(&self) -> Dialect {
        match self.platform {
            Platform::Mac | Platform::Ios => Dialect::Cocoa,
            Platform::Windows | Platform::Linux => Dialect::Qt,
        }
    }

    fn creator(&self) -> &'static str {
        match (self.generation, self.platform) {
            (Generation::Format20, Platform::Windows | Platform::Linux) => "SCRWIN-3.1.6.0",
            (Generation::Format20, _) => "SCRMAC-3.5.2-17487",
            (Generation::Format15, Platform::Mac) => "SCRMAC-2.8.1-29111",
            (Generation::Format15, Platform::Linux) => "SCRLIN-1.9.0.1",
            (Generation::Format15, _) => "SCRWIN-1.9.16.0",
            (Generation::Format10, Platform::Mac) => "SCRMAC-2.5-25238",
            (Generation::Format10, Platform::Linux) => "SCRLIN-1.7.2.3",
            (Generation::Format10, _) => "SCRWIN-1.8.6.0",
            (Generation::IosStub, _) => "SCRIOS-1.1.4-1281",
            (Generation::ScrivenerOne, _) => "",
        }
    }

    /// Write the project.
    pub fn build(&self) -> Files {
        let mut files = Files::new();
        match self.generation {
            Generation::ScrivenerOne => self.build_scrivener_one(&mut files),
            Generation::IosStub => self.build_ios_stub(&mut files),
            _ => self.build_desktop(&mut files),
        }
        files
    }

    fn build_desktop(&self, files: &mut Files) {
        let manifest = self.manifest();
        files.put(self.manifest_name(), manifest.clone());
        if self.version_txt {
            let stamp = match self.generation {
                Generation::Format10 => "16",
                Generation::Format15 => "18",
                _ => "23",
            };
            files.put("Files/version.txt", stamp);
        }
        self.write_payloads(files);
        if self.search_index {
            files.put("Files/search.indexes", self.search_index_xml());
        }
        if self.checksum {
            self.write_checksum(files);
        }
        if self.autosave {
            files.put(
                "Files/binder.autosave",
                binder_copy(&self.manifest_name(), manifest.as_bytes()),
            );
        }
        match self.backup {
            Backup::None => {}
            Backup::Copy => files.put(
                "Files/binder.backup",
                binder_copy(&self.manifest_name(), manifest.as_bytes()),
            ),
            Backup::Placeholder => {
                let placeholder = format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"1.0\" Creator=\"SCRMAC/2.5/25238\" Modified=\"{DATE}\" NewProject=\"Yes\"></ScrivenerProject>\n"
                );
                files.put(
                    "Files/binder.backup",
                    binder_copy(&self.manifest_name(), placeholder.as_bytes()),
                );
            }
        }
        if let Some(host) = &self.lock_host {
            files.put("Files/user.lock", self.lock_file(host));
        }
        if self.generation == Generation::Format20 {
            files.put(
                "Files/styles.xml",
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Styles>\n</Styles>\n",
            );
            let days: String = self
                .writing_days
                .iter()
                .map(|day| {
                    format!(
                        "    <Day dwc=\"120\" dcc=\"610\" owc=\"0\" occ=\"0\">{}</Day>\n",
                        xml_escape(day)
                    )
                })
                .collect();
            files.put(
                "Files/writing.history",
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<WritingHistory>\n{days}</WritingHistory>\n"
                ),
            );
            if self.platform == Platform::Mac {
                files.put("QuickLook/Preview.html", "<html><body></body></html>\n");
            }
        }
        if self.settings {
            self.write_compile(files);
            files.put(
                "Settings/templateinfo.xml",
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<TemplateSettings Version=\"1.0\">\n    <Title>Novel</Title>\n    <Category>Fiction</Category>\n</TemplateSettings>\n",
            );
        }
        if self.project_notes {
            files.put(
                "Files/ProjectNotes/index.xml",
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ProjectNotes Version=\"1.0\">\n    <Note ID=\"10\" Modified=\"{DATE}\">\n        <Title>General</Title>\n    </Note>\n</ProjectNotes>\n"
                ),
            );
            files.put(
                "Files/ProjectNotes/Note-10.rtf",
                rtf::document(self.dialect(), &[text::sentence(8).to_string()]),
            );
        }
        self.write_snapshots(files);
    }

    fn build_ios_stub(&self, files: &mut Files) {
        files.put(
            self.manifest_name(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n<ScrivenerProject MobileStub=\"Yes\" Device=\"Test Pad\"></ScrivenerProject>\n",
        );
        files.put("Mobile/binder.mob", self.manifest());
        self.write_payloads(files);
        if self.search_index {
            files.put("Mobile/search.indexes", self.search_index_xml());
        }
        files.put(
            "Mobile/writing.history",
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<WritingHistory>\n</WritingHistory>\n",
        );
        files.put(
            format!("Mobile/Data/{}.checksums", uuid_for(self.seed, 999)),
            "",
        );
    }

    fn build_scrivener_one(&self, files: &mut Files) {
        // Enough of a Scrivener 1 project to be recognised; the importer refuses it
        // without reading the archive.
        files.put("binder.scrivproj", b"bplist00\xd4\x01\x02\x03\x04".to_vec());
        files.put(
            "info.plist",
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n\t<key>SCRProjectVersion</key>\n\t<integer>4</integer>\n</dict>\n</plist>\n",
        );
        files.put(
            "3.rtfd/TXT.rtf",
            rtf::document(Dialect::Cocoa, &text::paragraphs(0, 1, 2)),
        );
        files.put("3_synopsis.txt", text::sentence(3));
    }

    fn write_payloads(&self, files: &mut Files) {
        let dialect = self.dialect();
        for (i, item) in self.items.iter().enumerate() {
            let prefix = self.payload_prefix(i);
            let (text, notes, synopsis) = match self.generation {
                Generation::Format20 | Generation::IosStub => {
                    ("content.rtf", "notes.rtf", "synopsis.txt")
                }
                _ => (".rtf", "_notes.rtf", "_synopsis.txt"),
            };
            if !item.text.is_empty() {
                files.put(
                    format!("{prefix}{text}"),
                    rtf::document(dialect, &item.text),
                );
            }
            if !item.notes.is_empty() {
                files.put(
                    format!("{prefix}{notes}"),
                    rtf::document(dialect, &item.notes),
                );
            }
            if let Some(synopsis_text) = &item.synopsis {
                files.put(format!("{prefix}{synopsis}"), synopsis_text.clone());
            }
            if let Some((ext, bytes)) = &item.media {
                let name = match self.generation {
                    Generation::Format20 | Generation::IosStub => format!("{prefix}content.{ext}"),
                    _ => format!("{prefix}.{ext}"),
                };
                files.put(name, bytes.clone());
            }
            if item.comments > 0 {
                let (name, body) = match self.generation {
                    Generation::Format20 | Generation::IosStub => (
                        format!("{prefix}content.comments"),
                        self.comments_xml(i, true),
                    ),
                    Generation::Format15 => {
                        (format!("{prefix}.comments"), self.comments_xml(i, false))
                    }
                    _ => (format!("{prefix}.links"), self.links_xml(i)),
                };
                files.put(name, body);
            }
        }
    }

    fn comments_xml(&self, index: usize, v3: bool) -> String {
        let item = &self.items[index];
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        out.push_str(
            if v3 && matches!(self.platform, Platform::Windows | Platform::Linux) {
                "<Comments Version=\"1.0\">\n"
            } else {
                "<Comments>\n"
            },
        );
        for c in 0..item.comments {
            let body = rtf::document(
                Dialect::Cocoa,
                &[text::sentence(index + c as usize).to_string()],
            );
            out.push_str(&format!(
                "    <Comment ID=\"{}\" Color=\"0.992 0.929 0.525\"><![CDATA[{body}]]></Comment>\n",
                uuid_for(self.seed ^ 0xC0, (index * 16) as u32 + c)
            ));
        }
        out.push_str("</Comments>\n");
        out
    }

    fn links_xml(&self, index: usize) -> String {
        let item = &self.items[index];
        let mut out = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Links Version=\"1.0\">\n    <CommentsAndFootnotes>\n",
        );
        for c in 0..item.comments {
            let body = rtf::document(
                Dialect::Cocoa,
                &[text::sentence(index + c as usize).to_string()],
            );
            out.push_str(&format!(
                "        <Comment IsFootnote=\"No\" Collapsed=\"No\">\n            <Range>{},{}</Range>\n            <Color>0.992 0.929 0.525</Color>\n            <RTF><![CDATA[{body}]]></RTF>\n        </Comment>\n",
                4 + c * 10,
                6
            ));
        }
        out.push_str("    </CommentsAndFootnotes>\n</Links>\n");
        out
    }

    fn search_index_xml(&self) -> String {
        let mut out = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<SearchIndexes Version=\"1.0\">\n    <Documents>\n",
        );
        for (i, item) in self.items.iter().enumerate() {
            let id = match self.generation {
                Generation::Format20 | Generation::IosStub => self.uuid(i),
                _ => self.number(i).to_string(),
            };
            out.push_str(&format!("        <Document ID=\"{id}\">\n"));
            // In Scrivener's own order: title, synopsis, text, comments, notes.
            out.push_str(&format!(
                "            <Title>{}</Title>\n",
                xml_escape(item.title.as_deref().unwrap_or_default())
            ));
            if let Some(synopsis) = &item.synopsis {
                out.push_str(&format!(
                    "            <Synopsis>{}</Synopsis>\n",
                    xml_escape(synopsis)
                ));
            }
            if !item.text.is_empty() {
                out.push_str(&format!(
                    "            <Text>{}</Text>\n",
                    xml_escape(&item.text.join("\n"))
                ));
            }
            if item.comments > 0 {
                let comments: Vec<&str> = (0..item.comments)
                    .map(|c| text::sentence(i + c as usize))
                    .collect();
                out.push_str(&format!(
                    "            <Comments>{}</Comments>\n",
                    xml_escape(&comments.join("\n"))
                ));
            }
            if !item.notes.is_empty() {
                out.push_str(&format!(
                    "            <Notes>{}</Notes>\n",
                    xml_escape(&item.notes.join("\n"))
                ));
            }
            out.push_str("        </Document>\n");
        }
        out.push_str("    </Documents>\n</SearchIndexes>\n");
        out
    }

    fn write_checksum(&self, files: &mut Files) {
        let (path, strip) = match self.generation {
            Generation::Format20 => ("Files/Data/docs.checksum", "Files/Data/"),
            Generation::Format10 | Generation::Format15 => {
                ("Files/Docs/docs.checksum", "Files/Docs/")
            }
            _ => return,
        };
        let mut lines = String::new();
        for (member, bytes) in files.iter() {
            let Some(name) = member.strip_prefix(strip) else {
                continue;
            };
            let lower = name.to_lowercase();
            let listed = lower.ends_with(".rtf") || lower.ends_with(".txt");
            if !listed {
                continue;
            }
            // Scrivener for Mac writes the UUID folders in lower case here.
            let name = if self.platform == Platform::Mac {
                lower
            } else {
                name.to_string()
            };
            lines.push_str(&format!("{name}={}\n", sha1::hex(bytes)));
        }
        files.put(path, lines);
    }

    fn write_snapshots(&self, files: &mut Files) {
        for (i, item) in self.items.iter().enumerate() {
            if item.snapshots == 0 {
                continue;
            }
            let key = match self.generation {
                Generation::Format20 => self.uuid(i),
                _ => self.number(i).to_string(),
            };
            let folder = format!("Snapshots/{key}.snapshots");
            let mut index = String::from(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Snapshots Version=\"1.0\">\n",
            );
            for s in 0..item.snapshots {
                let date = format!("2024-03-{:02} 09:00:00 +0100", 10 + s);
                index.push_str(&format!(
                    "    <Snapshot>\n        <Title>Untitled Snapshot</Title>\n        <Date>{date}</Date>\n    </Snapshot>\n"
                ));
                let file = format!("2024-03-{:02}-09-00-00+0100.rtf", 10 + s);
                files.put(
                    format!("{folder}/{file}"),
                    rtf::document(self.dialect(), &text::paragraphs(i + s as usize, 1, 2)),
                );
            }
            index.push_str("</Snapshots>\n");
            files.put(format!("{folder}/index.xml"), index);
        }
    }

    /// The compile settings, in the file the generation keeps them in.
    fn write_compile(&self, files: &mut Files) {
        let c = &self.compile;
        match (self.generation, self.platform) {
            (Generation::Format20, _) => {
                let mut meta = String::new();
                if let Some(title) = &c.title {
                    meta.push_str(&format!(
                        "            <ProjectTitle>{}</ProjectTitle>\n",
                        xml_escape(title)
                    ));
                }
                if !c.authors.is_empty() {
                    meta.push_str("            <Authors>\n");
                    for author in &c.authors {
                        let file_as = author
                            .file_as
                            .as_ref()
                            .map(|f| format!(" FileAs=\"{}\"", xml_escape(f)))
                            .unwrap_or_default();
                        let role = author
                            .role
                            .as_ref()
                            .map(|r| format!(" Role=\"{}\"", xml_escape(r)))
                            .unwrap_or_default();
                        meta.push_str(&format!(
                            "                <Author{file_as}{role}>{}</Author>\n",
                            xml_escape(&author.name)
                        ));
                    }
                    meta.push_str("            </Authors>\n");
                }
                if let Some(surname) = &c.surname {
                    meta.push_str(&format!(
                        "            <Surname>{}</Surname>\n",
                        xml_escape(surname)
                    ));
                }
                if let Some(forename) = &c.forename {
                    meta.push_str(&format!(
                        "            <Forename>{}</Forename>\n",
                        xml_escape(forename)
                    ));
                }
                if let Some(abbreviated) = &c.abbreviated_title {
                    meta.push_str(&format!(
                        "            <ProjectAbbreviatedTitle>{}</ProjectAbbreviatedTitle>\n",
                        xml_escape(abbreviated)
                    ));
                }
                if !c.subjects.is_empty() {
                    meta.push_str("            <Subjects>\n");
                    for subject in &c.subjects {
                        meta.push_str(&format!(
                            "                <Subject>{}</Subject>\n",
                            xml_escape(subject)
                        ));
                    }
                    meta.push_str("            </Subjects>\n");
                }
                if let Some(company) = &c.company {
                    meta.push_str(&format!(
                        "            <Company>{}</Company>\n",
                        xml_escape(company)
                    ));
                }
                meta.push_str(&format!(
                    "            <EbookLanguage>{}</EbookLanguage>\n",
                    xml_escape(c.language.as_deref().unwrap_or("en"))
                ));
                let mut replacements = String::new();
                if !c.replacements.is_empty() {
                    replacements.push_str("        <Replacements>\n");
                    for r in &c.replacements {
                        let mut attributes = String::new();
                        if r.ignore {
                            attributes.push_str(" Ignore=\"Yes\"");
                        }
                        if r.regex {
                            attributes.push_str(" RegEx=\"Yes\"");
                        }
                        replacements.push_str(&format!(
                            "            <Replacement{attributes}>\n                <Replace>{}</Replace>\n                <With>{}</With>\n            </Replacement>\n",
                            xml_escape(&r.replace),
                            xml_escape(&r.with)
                        ));
                    }
                    replacements.push_str("        </Replacements>\n");
                }
                // An untouched Scrivener 3 project writes these four options and no
                // `DeleteStruckThroughText`, which appears once the writer sets it.
                let struck = if c.delete_struck_through {
                    "            <DeleteStruckThroughText>Yes</DeleteStruckThroughText>\n"
                } else {
                    ""
                };
                let last = c
                    .last_format
                    .as_ref()
                    .map(|id| {
                        format!(
                            "    <CurrentFileType>rtf</CurrentFileType>\n    <LastUsedFormats>\n        <rtf>{}</rtf>\n    </LastUsedFormats>\n",
                            xml_escape(id)
                        )
                    })
                    .unwrap_or_default();
                files.put(
                    "Settings/compile.xml",
                    format!(
                        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<CompileSettings>\n    <ProjectSettings>\n        <Options>\n            <RemoveComments>Yes</RemoveComments>\n            <RemoveAnnotations>Yes</RemoveAnnotations>\n            <ResampleImages DPI=\"72\">No</ResampleImages>\n            <ConvertTablesAndListToMMD>Yes</ConvertTablesAndListToMMD>\n{struck}        </Options>\n        <MetaData>\n{meta}        </MetaData>\n{replacements}    </ProjectSettings>\n{last}</CompileSettings>\n"
                    ),
                );
                for format in &c.formats {
                    let mut styles = String::new();
                    for (names, element) in
                        [(&format.deleted, "DeleteText"), (&format.raw, "RawMarkup")]
                    {
                        for name in names {
                            styles.push_str(&format!(
                                "        <Style Name=\"{}\" ID=\"{}\" Type=\"Para+Char\">\n            <{element}>Yes</{element}>\n        </Style>\n",
                                xml_escape(name),
                                uuid_for(self.seed ^ 0xF0, name.len() as u32)
                            ));
                        }
                    }
                    files.put(
                        format!("Settings/Compile Formats/{}.scrformat", format.name),
                        format!(
                            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<CompileFormat Name=\"{}\" ID=\"{}\">\n    <SupportedTypes>\n        <Type>rtf</Type>\n    </SupportedTypes>\n    <Styles>\n{styles}    </Styles>\n</CompileFormat>\n",
                            xml_escape(&format.name),
                            xml_escape(&format.id)
                        ),
                    );
                }
            }
            (Generation::Format10 | Generation::Format15, Platform::Mac) => {
                let struck = if c.delete_struck_through {
                    "<true/>"
                } else {
                    "<false/>"
                };
                let mut variables =
                    String::from("\t<key>SCRCompileVariableReplacementsKey</key>\n");
                if c.variable_replacements.is_empty() {
                    variables.push_str("\t<array/>\n");
                } else {
                    variables.push_str("\t<array>\n");
                    for (variable, replacement) in &c.variable_replacements {
                        variables.push_str(&format!(
                            "\t\t<dict>\n\t\t\t<key>replacement</key>\n\t\t\t<string>{}</string>\n\t\t\t<key>variable</key>\n\t\t\t<string>{}</string>\n\t\t</dict>\n",
                            xml_escape(replacement),
                            xml_escape(variable)
                        ));
                    }
                    variables.push_str("\t</array>\n");
                }
                files.put(
                    "Settings/compile.plist",
                    format!(
                        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n\t<key>SCRCompileDeleteStruckThroughTextKey</key>\n\t{struck}\n\t<key>SCRCompileEBookAuthorsKey</key>\n\t<array/>\n\t<key>SCRCompileEBookLanguageKey</key>\n\t<string>{}</string>\n{variables}</dict>\n</plist>\n",
                        xml_escape(c.language.as_deref().unwrap_or("en"))
                    ),
                );
            }
            (Generation::Format10 | Generation::Format15, _) => {
                files.put(
                    "Settings/compile.ini",
                    format!(
                        "[General]\nversion=1\n\n[MetaData]\ntitle=\nauthors=\nlanguageCode={}\n",
                        c.language.as_deref().unwrap_or("English")
                    ),
                );
            }
            _ => {}
        }
    }

    fn lock_file(&self, host: &str) -> String {
        match self.platform {
            Platform::Mac | Platform::Ios => format!(
                "platform=mac\nhost={host}\nuser=Test Writer\nuuid={}\napp=Scrivener\nproject_path=/Users/test/{}\napp_path=/Applications/Scrivener 3.app\n",
                uuid_for(self.seed, 777),
                self.folder()
            ),
            Platform::Windows | Platform::Linux => format!(
                "[General]\r\nplatform=win\r\nhost={host}\r\nuser=Test Writer\r\nuuid={}\r\nproject_path=C:/Users/test/{}\r\napp_path=C:/Program Files/Scrivener3/Scrivener.exe\r\n",
                uuid_for(self.seed, 777),
                self.folder()
            ),
        }
    }

    /// The manifest (or, for an iOS stub, `Mobile/binder.mob`).
    pub fn manifest(&self) -> String {
        let g = self.generation;
        let v3 = matches!(g, Generation::Format20 | Generation::IosStub);
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let creator = self.creator();
        let win = matches!(self.platform, Platform::Windows | Platform::Linux);
        let mod_id = uuid_for(self.seed, 555);
        match g {
            Generation::Format20 if win => out.push_str(&format!(
                "<ScrivenerProject Template=\"No\" Version=\"2.0\" Identifier=\"{}\" Creator=\"{creator}\" Device=\"TEST-PC\" Modified=\"{LATER}\" ModID=\"{mod_id}\">\n",
                uuid_for(self.seed, 0)
            )),
            Generation::Format20 => out.push_str(&format!(
                "<ScrivenerProject Identifier=\"{}\" Version=\"2.0\" Creator=\"{creator}\" Device=\"Test-Mac\" Author=\"\" Modified=\"{LATER}\" ModID=\"{mod_id}\">\n",
                uuid_for(self.seed, 0)
            )),
            Generation::IosStub => out.push_str(&format!(
                "<ScrivenerProject Version=\"2.0\" Creator=\"{creator}\" Modified=\"{LATER}\" Device=\"Test Pad\" Checksums=\"{}\">\n",
                uuid_for(self.seed, 999)
            )),
            Generation::Format15 if win => out.push_str(&format!(
                "<ScrivenerProject Template=\"No\" Version=\"1.5\" Creator=\"{creator}\" Device=\"TEST-PC\" Modified=\"{LATER}\" ModID=\"{mod_id}\">\n"
            )),
            Generation::Format15 => out.push_str(&format!(
                "<ScrivenerProject Version=\"1.5\" Creator=\"{creator}\" Device=\"Test-Mac\" Modified=\"{LATER}\" ModID=\"{mod_id}\">\n"
            )),
            Generation::Format10 if win => out.push_str(&format!(
                "<ScrivenerProject Template=\"No\" Version=\"1.0\" Creator=\"{creator}\">\n"
            )),
            Generation::Format10 => out.push_str(&format!(
                "<ScrivenerProject Version=\"1.0\" Creator=\"{creator}\" Modified=\"{LATER}\">\n"
            )),
            Generation::ScrivenerOne => return String::new(),
        }
        out.push_str("    <Binder>\n");
        self.write_binder(&mut out);
        out.push_str("    </Binder>\n");
        // An iOS binder has no collections.
        if g != Generation::IosStub {
            out.push_str(&self.collections_xml());
        }
        out.push_str(&self.label_status_xml());
        if g != Generation::IosStub {
            out.push_str(&self.keywords_xml());
            out.push_str(&self.custom_fields_xml(v3));
            if g == Generation::Format20 {
                out.push_str(&self.section_types_xml());
            }
        }
        out.push_str(&self.targets_xml(g));
        out.push_str(&self.project_lists_xml());
        if g == Generation::Format20 {
            out.push_str(
                "    <RecentWritingHistory Date=\"2024-06-02 00:00:00 +0200\">\n        <DraftWordCount>0</DraftWordCount>\n        <DraftCharCount>0</DraftCharCount>\n        <OtherWordCount>0</OtherWordCount>\n        <OtherCharCount>0</OtherCharCount>\n    </RecentWritingHistory>\n",
            );
        }
        if let Some(t) = self.template_folder {
            match g {
                Generation::Format10 => out.push_str(&format!(
                    "    <TemplateFolderID>{}</TemplateFolderID>\n",
                    self.number(t)
                )),
                Generation::Format15 | Generation::Format20 => out.push_str(&format!(
                    "    <TemplateFolderUUID>{}</TemplateFolderUUID>\n",
                    self.uuid(t)
                )),
                _ => {}
            }
        }
        if matches!(g, Generation::Format10 | Generation::Format15) {
            out.push_str(&self.properties_xml());
        }
        if g != Generation::IosStub {
            out.push_str(
                "    <PrintSettings PaperSize=\"612.0,792.0\" LeftMargin=\"72.0\" RightMargin=\"72.0\" TopMargin=\"72.0\" BottomMargin=\"72.0\" PaperType=\"na-letter\" Orientation=\"Portrait\" Collates=\"Yes\"/>\n",
            );
        }
        out.push_str("</ScrivenerProject>\n");
        out
    }

    /// The binder, written without recursion: an item opens `<Children>` when the
    /// next one is deeper, and closes as many levels as the next one climbs.
    fn write_binder(&self, out: &mut String) {
        let g = self.generation;
        let win = matches!(self.platform, Platform::Windows | Platform::Linux);
        let mut open: Vec<u16> = Vec::new();
        for (i, item) in self.items.iter().enumerate() {
            while open.last().is_some_and(|d| *d >= item.depth) {
                open.pop();
                let pad = indent(open.len() * 2 + 2);
                out.push_str(&format!("{pad}    </Children>\n{pad}</BinderItem>\n"));
            }
            let pad = indent(open.len() * 2 + 2);
            let kind = item.kind.type_name();
            let created = xml_escape(item.created.as_deref().unwrap_or(DATE));
            let modified = xml_escape(item.modified.as_deref().unwrap_or(DATE));
            let ids = match g {
                Generation::Format20 | Generation::IosStub => {
                    format!("UUID=\"{}\" Type=\"{kind}\"", self.uuid(i))
                }
                Generation::Format15 if win => format!(
                    "ID=\"{}\" UUID=\"{}\" Created=\"{created}\" Modified=\"{modified}\" Type=\"{kind}\"",
                    self.number(i),
                    self.uuid(i)
                ),
                Generation::Format15 => format!(
                    "ID=\"{}\" UUID=\"{}\" Type=\"{kind}\"",
                    self.number(i),
                    self.uuid(i)
                ),
                Generation::Format10 if win => {
                    format!(
                        "ID=\"{}\" Created=\"{created}\" Modified=\"{modified}\" Type=\"{kind}\"",
                        self.number(i)
                    )
                }
                _ => format!("ID=\"{}\" Type=\"{kind}\"", self.number(i)),
            };
            let dates = if ids.contains("Created=") {
                String::new()
            } else {
                format!(" Created=\"{created}\" Modified=\"{modified}\"")
            };
            out.push_str(&format!("{pad}<BinderItem {ids}{dates}>\n"));
            if let Some(title) = &item.title {
                out.push_str(&format!("{pad}    <Title>{}</Title>\n", xml_escape(title)));
            }
            out.push_str(&self.metadata_xml(i, &pad));
            out.push_str(&format!(
                "{pad}    <TextSettings>\n{pad}        <TextSelection>0,0</TextSelection>\n"
            ));
            if let (Some(target), false) = (item.target, g == Generation::IosStub) {
                // Format 1.0 targets carry their type only; an iOS binder was
                // seen with none.
                let notify = if g == Generation::Format10 {
                    ""
                } else {
                    " Notify=\"No\""
                };
                let kind = xml_escape(item.target_kind.as_deref().unwrap_or("Words"));
                out.push_str(&format!(
                    "{pad}        <Target Type=\"{kind}\"{notify}>{target}</Target>\n"
                ));
            }
            if item.script_mode && matches!(g, Generation::Format15 | Generation::Format20) {
                out.push_str(&format!("{pad}        <TextMode>Script</TextMode>\n"));
            }
            out.push_str(&format!("{pad}    </TextSettings>\n"));
            out.push_str(&self.bookmarks_xml(&item.bookmarks, &pad));
            if !item.keywords.is_empty() && g != Generation::IosStub {
                out.push_str(&format!("{pad}    <Keywords>\n"));
                for k in &item.keywords {
                    out.push_str(&format!("{pad}        <KeywordID>{k}</KeywordID>\n"));
                }
                out.push_str(&format!("{pad}    </Keywords>\n"));
            }
            let next_deeper = self.items.get(i + 1).is_some_and(|n| n.depth > item.depth);
            if next_deeper {
                out.push_str(&format!("{pad}    <Children>\n"));
                open.push(item.depth);
            } else {
                out.push_str(&format!("{pad}</BinderItem>\n"));
            }
        }
        while !open.is_empty() {
            open.pop();
            let pad = indent(open.len() * 2 + 2);
            out.push_str(&format!("{pad}    </Children>\n{pad}</BinderItem>\n"));
        }
    }

    fn metadata_xml(&self, index: usize, pad: &str) -> String {
        let g = self.generation;
        let item = &self.items[index];
        let mut meta = String::new();
        // An iOS binder writes it too (`binder.mob`), as `Yes` only.
        if item.include_in_compile {
            meta.push_str(&format!(
                "{pad}        <IncludeInCompile>Yes</IncludeInCompile>\n"
            ));
        }
        if let (Some(t), Generation::Format20) = (item.section_type, g) {
            meta.push_str(&format!(
                "{pad}        <SectionType>{}</SectionType>\n",
                uuid_for(self.seed ^ 0x57, t as u32)
            ));
        }
        let none = if g == Generation::IosStub {
            Some(-1)
        } else {
            None
        };
        if let Some(label) = item.label.or(none) {
            meta.push_str(&format!("{pad}        <LabelID>{label}</LabelID>\n"));
        }
        if let Some(status) = item.status.or(none) {
            meta.push_str(&format!("{pad}        <StatusID>{status}</StatusID>\n"));
        }
        if let Some((ext, _)) = &item.media {
            meta.push_str(&format!(
                "{pad}        <FileExtension>{ext}</FileExtension>\n"
            ));
        }
        if let Some(icon) = &item.icon {
            meta.push_str(&format!(
                "{pad}        <IconFileName>{}</IconFileName>\n",
                xml_escape(icon)
            ));
        }
        if !item.custom.is_empty() && g != Generation::IosStub {
            meta.push_str(&format!("{pad}        <CustomMetaData>\n"));
            for (field, value) in &item.custom {
                meta.push_str(&format!(
                    "{pad}            <MetaDataItem>\n{pad}                <FieldID>{}</FieldID>\n{pad}                <Value>{}</Value>\n{pad}            </MetaDataItem>\n",
                    xml_escape(field),
                    xml_escape(value)
                ));
            }
            meta.push_str(&format!("{pad}        </CustomMetaData>\n"));
        }
        if matches!(g, Generation::Format10 | Generation::Format15) {
            meta.push_str(&format!(
                "{pad}        <NotesTextSelection>0,0</NotesTextSelection>\n"
            ));
        }
        if meta.is_empty() {
            return String::new();
        }
        format!("{pad}    <MetaData>\n{meta}{pad}    </MetaData>\n")
    }

    /// Document bookmarks (format 2.0) or references (format 1.x).
    fn bookmarks_xml(&self, bookmarks: &[Bookmark], pad: &str) -> String {
        let g = self.generation;
        if bookmarks.is_empty() || g == Generation::IosStub {
            return String::new();
        }
        let v3 = g == Generation::Format20;
        let (list, entry) = if v3 {
            ("Bookmarks", "Bookmark")
        } else {
            ("References", "Reference")
        };
        let mut out = format!("{pad}    <{list}>\n");
        for line in bookmarks.iter().filter_map(|b| self.link_xml(b, entry)) {
            out.push_str(&format!("{pad}        {line}\n"));
        }
        out.push_str(&format!("{pad}    </{list}>\n"));
        out
    }

    /// One bookmark or reference element.
    fn link_xml(&self, bookmark: &Bookmark, entry: &str) -> Option<String> {
        let v3 = self.generation == Generation::Format20;
        let mut attributes = String::new();
        if bookmark.target.is_some() || bookmark.dangling {
            let target = if v3 {
                let uuid = match bookmark.target {
                    Some(t) => self.uuid(t),
                    None => uuid_for(self.seed ^ 0xDA, 1),
                };
                format!("BinderUUID=\"{uuid}\"")
            } else {
                let number = match bookmark.target {
                    Some(t) => self.number(t),
                    None => 9_999,
                };
                format!("BinderID=\"{number}\" Destination=\"[Internal Link]\"")
            };
            attributes.push_str(&target);
        } else if let Some(url) = &bookmark.destination {
            attributes.push_str(&format!("Destination=\"{}\"", xml_escape(url)));
        } else {
            return None;
        }
        Some(match &bookmark.text {
            Some(text) => format!("<{entry} {attributes}>{}</{entry}>", xml_escape(text)),
            None => format!("<{entry} {attributes}/>"),
        })
    }

    fn collections_xml(&self) -> String {
        let v3 = self.generation == Generation::Format20;
        if let Some(collections) = &self.collections {
            let mut out = String::from("    <Collections>\n");
            for (k, c) in collections.iter().enumerate() {
                out.push_str(&format!(
                    "        <Collection Type=\"{}\" ID=\"{}\" Color=\"0.8 0.4 0.4\">\n            <Title>{}</Title>\n",
                    xml_escape(&c.kind),
                    uuid_for(self.seed ^ 0xC1, k as u32 + 10),
                    xml_escape(&c.title)
                ));
                if !c.members.is_empty() {
                    if v3 {
                        let list: Vec<String> = c.members.iter().map(|&i| self.uuid(i)).collect();
                        out.push_str(&format!(
                            "            <BinderUUIDs>{}</BinderUUIDs>\n",
                            list.join(",")
                        ));
                    } else {
                        out.push_str("            <BinderIDs>\n");
                        for &i in &c.members {
                            out.push_str(&format!(
                                "                <BinderID>{}</BinderID>\n",
                                self.number(i)
                            ));
                        }
                        out.push_str("            </BinderIDs>\n");
                    }
                }
                if let Some((query, kind, operator, ids)) = &c.search {
                    let ids = ids
                        .as_ref()
                        .map(|ids| format!(" CustomIDs=\"{}\"", xml_escape(ids)))
                        .unwrap_or_default();
                    out.push_str(&format!(
                        "            <SearchSettings Operator=\"{}\" Type=\"{}\"{ids} CaseSensitive=\"No\">{}</SearchSettings>\n",
                        xml_escape(operator),
                        xml_escape(kind),
                        xml_escape(query)
                    ));
                }
                out.push_str("        </Collection>\n");
            }
            out.push_str("    </Collections>\n");
            return out;
        }
        let mut out = String::from("    <Collections>\n");
        out.push_str(&format!(
            "        <Collection Type=\"Binder\" ID=\"{}\" Color=\"0.5 0.5 0.5\">\n            <Title>Binder</Title>\n        </Collection>\n",
            uuid_for(self.seed ^ 0xC1, 1)
        ));
        // One collection holding the first two scenes.
        let members: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, it)| it.kind == Kind::Text)
            .map(|(i, _)| i)
            .take(2)
            .collect();
        out.push_str(&format!(
            "        <Collection Type=\"Arbitrary\" ID=\"{}\" Color=\"0.8 0.4 0.4\">\n            <Title>To revise</Title>\n",
            uuid_for(self.seed ^ 0xC1, 2)
        ));
        if v3 {
            let list: Vec<String> = members.iter().map(|&i| self.uuid(i)).collect();
            out.push_str(&format!(
                "            <BinderUUIDs>{}</BinderUUIDs>\n",
                list.join(",")
            ));
        } else {
            out.push_str("            <BinderIDs>\n");
            for &i in &members {
                out.push_str(&format!(
                    "                <BinderID>{}</BinderID>\n",
                    self.number(i)
                ));
            }
            out.push_str("            </BinderIDs>\n");
        }
        out.push_str("        </Collection>\n    </Collections>\n");
        out
    }

    fn label_status_xml(&self) -> String {
        let mut out = format!(
            "    <LabelSettings>\n        <Title>{}</Title>\n        <DefaultLabelID>-1</DefaultLabelID>\n        <Labels>\n",
            xml_escape(&self.label_title)
        );
        for (id, name) in &self.labels {
            // Each label its own colour, as Scrivener writes them: six decimals.
            let colour = if *id == -1 {
                String::new()
            } else {
                let k = id.rem_euclid(7) as f32;
                format!(
                    " Color=\"{:.6} {:.6} {:.6}\"",
                    0.993_5 - k * 0.1,
                    0.701_213,
                    0.2 + k * 0.1
                )
            };
            out.push_str(&format!(
                "            <Label ID=\"{id}\"{colour}>{}</Label>\n",
                xml_escape(name)
            ));
        }
        out.push_str(&format!(
            "        </Labels>\n    </LabelSettings>\n    <StatusSettings>\n        <Title>{}</Title>\n        <DefaultStatusID>-1</DefaultStatusID>\n        <StatusItems>\n",
            xml_escape(&self.status_title)
        ));
        for (id, name) in &self.statuses {
            out.push_str(&format!(
                "            <Status ID=\"{id}\">{}</Status>\n",
                xml_escape(name)
            ));
        }
        out.push_str("        </StatusItems>\n    </StatusSettings>\n");
        out
    }

    /// Format 1.x `ProjectProperties`, when the spec sets them.
    fn properties_xml(&self) -> String {
        let Some(props) = &self.properties else {
            return String::new();
        };
        let mut out = String::from("    <ProjectProperties>\n");
        for (element, value) in [
            ("ProjectTitle", &props.title),
            ("AbbreviatedTitle", &props.abbreviated_title),
            ("FullName", &props.full_name),
            ("FirstName", &props.first_name),
            ("LastName", &props.last_name),
        ] {
            if let Some(value) = value {
                out.push_str(&format!(
                    "        <{element}>{}</{element}>\n",
                    xml_escape(value)
                ));
            }
        }
        out.push_str("    </ProjectProperties>\n");
        out
    }

    fn keywords_xml(&self) -> String {
        if self.keywords.is_empty() {
            return String::new();
        }
        let mut out = String::from("    <Keywords>\n");
        let mut open: Vec<u16> = Vec::new();
        for (i, (depth, id, title)) in self.keywords.iter().enumerate() {
            while open.last().is_some_and(|d| d >= depth) {
                open.pop();
                let pad = indent(open.len() * 2 + 2);
                out.push_str(&format!("{pad}    </Children>\n{pad}</Keyword>\n"));
            }
            let pad = indent(open.len() * 2 + 2);
            out.push_str(&format!(
                "{pad}<Keyword ID=\"{id}\">\n{pad}    <Title>{}</Title>\n{pad}    <Color>0.9 0.8 0.6</Color>\n",
                xml_escape(title)
            ));
            let deeper = self.keywords.get(i + 1).is_some_and(|(d, _, _)| d > depth);
            if deeper {
                out.push_str(&format!("{pad}    <Children>\n"));
                open.push(*depth);
            } else {
                out.push_str(&format!("{pad}</Keyword>\n"));
            }
        }
        while !open.is_empty() {
            open.pop();
            let pad = indent(open.len() * 2 + 2);
            out.push_str(&format!("{pad}    </Children>\n{pad}</Keyword>\n"));
        }
        out.push_str("    </Keywords>\n");
        out
    }

    fn custom_fields_xml(&self, v3: bool) -> String {
        if self.custom_fields.is_empty() && !(v3 && !self.typed_fields.is_empty()) {
            return String::new();
        }
        let mut out = String::from("    <CustomMetaDataSettings>\n");
        if v3 {
            for field in &self.typed_fields {
                let default = field
                    .default
                    .as_ref()
                    .map(|d| format!(" Default=\"{}\"", xml_escape(d)))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "        <MetaDataField Type=\"{}\" ID=\"{}\"{default}>\n            <Title>{}</Title>\n",
                    xml_escape(&field.kind),
                    xml_escape(&field.id),
                    xml_escape(&field.title)
                ));
                if field.kind == "List" {
                    let none = field
                        .none
                        .as_ref()
                        .map(|n| format!(" None=\"{}\"", xml_escape(n)))
                        .unwrap_or_default();
                    out.push_str(&format!("            <ListOptions{none}>\n"));
                    for (id, name) in &field.options {
                        out.push_str(&format!(
                            "                <Option ID=\"{id}\">{}</Option>\n",
                            xml_escape(name)
                        ));
                    }
                    out.push_str("            </ListOptions>\n");
                }
                out.push_str("        </MetaDataField>\n");
            }
        }
        for (id, title) in &self.custom_fields {
            if v3 {
                out.push_str(&format!(
                    "        <MetaDataField Type=\"Text\" ID=\"{}\" Wraps=\"No\" Align=\"Left\">\n            <Title>{}</Title>\n        </MetaDataField>\n",
                    xml_escape(id),
                    xml_escape(title)
                ));
            } else {
                out.push_str(&format!(
                    "        <MetaDataField ID=\"{}\" Wraps=\"Yes\">{}</MetaDataField>\n",
                    xml_escape(id),
                    xml_escape(title)
                ));
            }
        }
        out.push_str("    </CustomMetaDataSettings>\n");
        out
    }

    fn section_types_xml(&self) -> String {
        let mut out = String::from("    <SectionTypes>\n        <TypeDefinitions>\n");
        for (i, name) in self.section_types.iter().enumerate() {
            out.push_str(&format!(
                "            <Type ID=\"{}\">{}</Type>\n",
                uuid_for(self.seed ^ 0x57, i as u32),
                xml_escape(name)
            ));
        }
        out.push_str("        </TypeDefinitions>\n        <LevelTypes>\n");
        let first = uuid_for(self.seed ^ 0x57, 0);
        let second = uuid_for(self.seed ^ 0x57, 1);
        out.push_str(&format!(
            "            <Folders>\n                <Type>{first}</Type>\n            </Folders>\n            <Containers>\n                <Type>{first}</Type>\n            </Containers>\n            <Files>\n                <Type>{second}</Type>\n            </Files>\n"
        ));
        out.push_str("        </LevelTypes>\n    </SectionTypes>\n");
        out
    }

    fn targets_xml(&self, g: Generation) -> String {
        if g == Generation::IosStub {
            return "    <ProjectTargets>\n        <DraftTarget Type=\"Words\">0</DraftTarget>\n        <PreviousSession Words=\"0\" Characters=\"0\"/>\n    </ProjectTargets>\n".to_string();
        }
        let t = &self.targets;
        let yes_no = |b: bool| if b { "Yes" } else { "No" };
        let deadline = t
            .deadline
            .as_ref()
            .map(|d| {
                format!(
                    " Deadline=\"{}\" IgnoreDeadline=\"{}\"",
                    xml_escape(d),
                    yes_no(t.ignore_deadline)
                )
            })
            .unwrap_or_default();
        // Scrivener 3 resets the session at a time of day and writes when it next will;
        // the older formats say only that it resets at midnight.
        let reset = if g == Generation::Format20 {
            format!("NextResetDate=\"{DATE}\" ResetType=\"Time\" ResetTime=\"00:00\"")
        } else {
            "ResetType=\"Midnight\"".to_string()
        };
        format!(
            "    <ProjectTargets Notify=\"No\">\n        <DraftTarget Type=\"{}\" CountIncludedOnly=\"{}\" CurrentCompileGroupOnly=\"{}\"{deadline}>{}</DraftTarget>\n        <SessionTarget Type=\"{}\" CountDraftOnly=\"Yes\" AllowNegatives=\"Yes\" {reset} DeterminedFromDeadline=\"{}\" WritingDays=\"{}\" CanWriteOnDeadlineDate=\"{}\">{}</SessionTarget>\n        <PreviousSession Words=\"0\" Characters=\"0\" Date=\"{DATE}\"/>\n    </ProjectTargets>\n",
            xml_escape(&t.draft_kind),
            yes_no(t.count_included_only),
            yes_no(t.compile_group_only),
            t.draft,
            xml_escape(&t.session_kind),
            yes_no(t.determined_from_deadline),
            xml_escape(&t.writing_days),
            yes_no(t.can_write_on_deadline),
            t.session,
        )
    }

    /// Project bookmarks (format 2.0), favourites (format 1.x) and the auto-complete list
    /// (format 2.0).
    fn project_lists_xml(&self) -> String {
        let g = self.generation;
        let mut out = String::new();
        if g == Generation::Format20 {
            let lines: Vec<String> = self
                .project_bookmarks
                .iter()
                .filter_map(|b| self.link_xml(b, "Bookmark"))
                .collect();
            if !lines.is_empty() {
                out.push_str("    <ProjectBookmarks>\n");
                for line in lines {
                    out.push_str(&format!("        {line}\n"));
                }
                out.push_str("    </ProjectBookmarks>\n");
            }
            if !self.auto_complete.is_empty() {
                out.push_str("    <AutoCompleteList>\n");
                for entry in &self.auto_complete {
                    out.push_str(&format!(
                        "        <Completion Scope=\"0\">{}</Completion>\n",
                        xml_escape(entry)
                    ));
                }
                out.push_str("    </AutoCompleteList>\n");
            }
        }
        if matches!(g, Generation::Format10 | Generation::Format15) && !self.favorites.is_empty() {
            out.push_str("    <Favorites>\n");
            for &i in &self.favorites {
                out.push_str(&format!(
                    "        <BinderID>{}</BinderID>\n",
                    self.number(i)
                ));
            }
            out.push_str("    </Favorites>\n");
        }
        out
    }
}

/// A deterministic UUID, upper-case.
pub fn uuid_for(seed: u32, n: u32) -> String {
    format!(
        "{seed:08X}-{:04X}-4{:03X}-8{:03X}-{:012X}",
        (n >> 16) & 0xFFFF,
        n & 0xFFF,
        (n >> 12) & 0xFFF,
        u64::from(n)
    )
}

/// A spare binder copy: a zip holding one manifest, as Scrivener writes
/// `Files/binder.autosave` and `Files/binder.backup`.
pub fn binder_copy(name: &str, manifest: &[u8]) -> Vec<u8> {
    let mut files = Files::new();
    files.put(name, manifest.to_vec());
    files.zip(None).unwrap_or_default()
}

fn indent(n: usize) -> String {
    " ".repeat(n * 2)
}
