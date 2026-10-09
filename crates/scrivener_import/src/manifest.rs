// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a manifest (`.scrivx`, or an iOS `binder.mob`) into the model.
//!
//! One reader for formats 1.0, 1.5 and 2.0 and the iOS binder: they share one XML
//! family, and what differs (the key attributes, `References` against `Bookmarks`,
//! `BinderIDs` against `BinderUUIDs`, a custom field's title as text against a
//! `Title` child, the template folder by number or by UUID) is settled here, so
//! nothing downstream sees it. Blocks a manifest does not carry are simply empty:
//! an iOS binder has no keywords, collections, section types, custom fields or
//! bookmarks at all, and a desktop manifest may lack any of them too.
//!
//! The binder is walked with a stack, never by recursion, and comes out flat,
//! each row carrying its depth (see `model`).
//!
//! # Identifiers
//!
//! An identifier becomes part of a path (`Files/Docs/<ID>.rtf`,
//! `Files/Data/<UUID>/`), so only what Scrivener writes is accepted: a UUID as
//! 8-4-4-4-12 hexadecimal digits, a number of one to nine digits, an extension of
//! one to sixteen letters and digits. Anything else is dropped and counted. Not one
//! of 30,774 real binder items breaks that grammar.

use std::collections::BTreeMap;

use uuid::Uuid;

use crate::dates::{stamp, stamp_opt};
use crate::model::{
    BinderRow, Collection, CustomFieldDef, CustomValue, DocTarget, ItemKey, ItemKind, ItemMeta,
    KeywordRow, Link, ListOption, Payload, ProjectProperties, ProjectTargets, RawTarget, Rgb,
    SearchSettings, SectionTypeDef, SectionTypes, Stamp, VocabEntry, Vocabulary,
};
use crate::names::{is_extension, is_item_number, is_uuid};
use crate::outcome::ManifestDamage;
use crate::xml::{self, XmlProblem, child, child_text, children, text};

/// The most binder items one manifest is read with.
///
/// A row of the model takes about a kilobyte, and an item as little as twenty
/// bytes of XML (`<BinderItem ID="1"/>`), so a 40 MB manifest within every other
/// ceiling would come out as two gigabytes of rows. The largest real binder seen
/// lists 2,509 items; a hundred thousand is forty times that, and the same
/// ceiling a project's files are held to.
pub(crate) const MAX_ITEMS: usize = crate::index::MAX_MEMBERS as usize;

/// A manifest read whole.
#[derive(Debug, Clone)]
pub(crate) struct ManifestRead {
    /// The root element's attributes.
    pub(crate) attrs: BTreeMap<String, String>,
    pub(crate) rows: Vec<BinderRow>,
    /// Every item's raw numeric ID, beside its row (format 1.x payload key).
    pub(crate) blocks: Blocks,
    /// Identifier attributes and extensions dropped for breaking the grammar.
    pub(crate) invalid_identifiers: u32,
}

/// The project-level blocks of a manifest.
#[derive(Debug, Clone, Default)]
pub(crate) struct Blocks {
    pub(crate) labels: Vocabulary,
    pub(crate) statuses: Vocabulary,
    pub(crate) keywords: Vec<KeywordRow>,
    pub(crate) custom_fields: Vec<CustomFieldDef>,
    pub(crate) section_types: SectionTypes,
    pub(crate) collections: Vec<Collection>,
    pub(crate) project_links: Vec<Link>,
    pub(crate) targets: ProjectTargets,
    pub(crate) template_folder: Option<ItemKey>,
    pub(crate) bookmarks_folder: Option<ItemKey>,
    pub(crate) favorites: Vec<ItemKey>,
    pub(crate) auto_complete: Vec<String>,
    pub(crate) properties: ProjectProperties,
    /// `RecentWritingHistory/@Date`, read as the latest day of writing history the manifest
    /// caches. Unverified: in the Mac ground truth (3.5.2) it is the project's creation
    /// instant, in both projects holding one day of history, so it may be when the history
    /// began rather than its latest day (M20).
    pub(crate) recent_writing_day: Option<Stamp>,
}

/// The root element's attributes, and whether it is a manifest at all, from a
/// manifest's bytes. `None` when it is not one.
#[derive(Debug, Clone, Default)]
pub(crate) struct Header {
    pub(crate) attrs: BTreeMap<String, String>,
}

impl Header {
    pub(crate) fn get(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(String::as_str)
    }
    pub(crate) fn is_mobile_stub(&self) -> bool {
        self.get("MobileStub") == Some("Yes")
    }
}

/// Read the start tag of a manifest from the first bytes of it, without parsing
/// the document: enough to tell an iOS stub, a version and a date, for a folder
/// the analysis does not otherwise read.
pub(crate) fn header(bytes: &[u8]) -> Option<Header> {
    let text = String::from_utf8_lossy(bytes);
    let start = text.find("<ScrivenerProject")?;
    let rest = &text[start + "<ScrivenerProject".len()..];
    let end = rest.find('>')?;
    let mut attrs = BTreeMap::new();
    let mut s = &rest[..end];
    // `name="value"` pairs, the value up to its closing quote.
    while let Some(eq) = s.find('=') {
        let name = s[..eq].trim().trim_start_matches('/').to_string();
        let after = s[eq + 1..].trim_start();
        let Some(quote) = after.chars().next().filter(|c| *c == '"' || *c == '\'') else {
            break;
        };
        let body = &after[1..];
        let Some(close) = body.find(quote) else {
            break;
        };
        if !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
        {
            attrs.insert(name, unescape(&body[..close]));
        }
        s = &body[close + 1..];
    }
    Some(Header { attrs })
}

fn unescape(value: &str) -> String {
    value
        .replace("&apos;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// Read a manifest whole, with at most `max_items` binder items (never more than
/// [`MAX_ITEMS`]). `part` names it in a refusal.
pub(crate) fn read(
    part: &str,
    bytes: &[u8],
    max_items: usize,
) -> Result<ManifestRead, ManifestDamage> {
    let text = xml::text_of(bytes).map_err(damage_of)?;
    let doc = xml::parse(part, text).map_err(damage_of)?;
    let root = doc.root_element();
    if root.tag_name().name() != "ScrivenerProject" {
        return Err(ManifestDamage::NotAManifest);
    }
    let attrs: BTreeMap<String, String> = root
        .attributes()
        .map(|a| (a.name().to_string(), a.value().to_string()))
        .collect();
    if attrs.get("NewProject").map(String::as_str) == Some("Yes") {
        return Err(ManifestDamage::NewProjectPlaceholder);
    }
    let binder = child(root, "Binder").ok_or(ManifestDamage::EmptyBinder)?;
    let (rows, invalid_identifiers) = read_binder(binder, max_items.min(MAX_ITEMS))?;
    if rows.is_empty() {
        return Err(ManifestDamage::EmptyBinder);
    }
    Ok(ManifestRead {
        attrs,
        rows,
        blocks: read_blocks(root),
        invalid_identifiers,
    })
}

fn damage_of(problem: XmlProblem) -> ManifestDamage {
    match problem {
        XmlProblem::NotText => ManifestDamage::NotText,
        XmlProblem::TooDeep { depth, line } => ManifestDamage::TooDeep { depth, line },
        XmlProblem::DeclaresEntities => ManifestDamage::DeclaresEntities,
        XmlProblem::Malformed(reason) => ManifestDamage::NotXml { reason },
    }
}

/// The binder, flat, in Scrivener's order, and how many identifiers were dropped;
/// refused at the first item past `max_items`, so its rows never exist.
fn read_binder(
    binder: roxmltree::Node<'_, '_>,
    max_items: usize,
) -> Result<(Vec<BinderRow>, u32), ManifestDamage> {
    let mut rows = Vec::new();
    let mut invalid = 0u32;
    let mut stack: Vec<(roxmltree::Node<'_, '_>, u16)> =
        children(binder, "BinderItem").map(|n| (n, 0)).collect();
    stack.reverse();
    while let Some((node, depth)) = stack.pop() {
        if rows.len() >= max_items {
            return Err(ManifestDamage::TooManyItems {
                limit: u32::try_from(MAX_ITEMS).unwrap_or(u32::MAX),
            });
        }
        rows.push(read_item(node, depth, &mut invalid));
        if let Some(kids) = child(node, "Children") {
            let mut below: Vec<_> = children(kids, "BinderItem")
                .map(|n| (n, depth.saturating_add(1)))
                .collect();
            below.reverse();
            stack.extend(below);
        }
    }
    Ok((rows, invalid))
}

/// A number or UUID identifier checked against the grammar; `invalid` counts a
/// value present and refused.
fn number_attr(raw: Option<&str>, invalid: &mut u32) -> Option<u32> {
    let raw = raw?;
    if is_item_number(raw) {
        raw.parse().ok()
    } else {
        *invalid += 1;
        None
    }
}

fn uuid_attr(raw: Option<&str>, invalid: &mut u32) -> Option<Uuid> {
    let raw = raw?;
    if is_uuid(raw) {
        Uuid::parse_str(raw).ok()
    } else {
        *invalid += 1;
        None
    }
}

/// An item key written as element text (a template folder, a default child
/// template): a UUID or a number, whichever it is.
pub(crate) fn key_text(raw: &str) -> Option<ItemKey> {
    let raw = raw.trim();
    if is_uuid(raw) {
        Uuid::parse_str(raw).ok().map(ItemKey::Uuid)
    } else if is_item_number(raw) {
        raw.parse().ok().map(ItemKey::Number)
    } else {
        None
    }
}

fn read_item(node: roxmltree::Node<'_, '_>, depth: u16, invalid: &mut u32) -> BinderRow {
    let number = number_attr(node.attribute("ID"), invalid);
    let uuid = uuid_attr(node.attribute("UUID"), invalid);
    let key = uuid.map(ItemKey::Uuid).or(number.map(ItemKey::Number));
    let kind = ItemKind::from_type(node.attribute("Type").unwrap_or_default());
    let meta = child(node, "MetaData")
        .map(|m| read_meta(m, invalid))
        .unwrap_or_default();
    let keyword_ids = child(node, "Keywords")
        .map(|k| {
            children(k, "KeywordID")
                .filter_map(|id| text(id).and_then(|t| t.trim().parse().ok()))
                .collect()
        })
        .unwrap_or_default();
    let mut links = Vec::new();
    if let Some(bookmarks) = child(node, "Bookmarks") {
        links.extend(children(bookmarks, "Bookmark").map(read_link));
    }
    if let Some(references) = child(node, "References") {
        links.extend(children(references, "Reference").map(read_link));
    }
    let settings = child(node, "TextSettings");
    let target = settings
        .and_then(|s| child(s, "Target"))
        .map(|t| DocTarget {
            kind: t.attribute("Type").map(str::to_string),
            value: text(t).unwrap_or_default().trim().to_string(),
            notify: t.attribute("Notify") == Some("Yes"),
        });
    let script_mode = settings
        .and_then(|s| child_text(s, "TextMode"))
        .is_some_and(|mode| mode.trim() == "Script");
    let default_child_template = child_text(node, "DefaultChildTemplateUUID")
        .or_else(|| child_text(node, "DefaultChildTemplateID"))
        .and_then(|raw| key_text(&raw));
    BinderRow {
        depth,
        key,
        number,
        owns_key: true,
        kind,
        title: child_text(node, "Title"),
        created: stamp_opt(node.attribute("Created")),
        modified: stamp_opt(node.attribute("Modified")),
        meta,
        keyword_ids,
        links,
        target,
        script_mode,
        default_child_template,
        binder_separator: child_text(node, "BinderSeparator").is_some_and(|v| v.trim() == "Yes"),
        payload: Payload::default(),
        snapshots: 0,
    }
}

fn read_meta(meta: roxmltree::Node<'_, '_>, invalid: &mut u32) -> ItemMeta {
    let yes = |name: &str| child_text(meta, name).is_some_and(|v| v.trim() == "Yes");
    let id = |name: &str| {
        child_text(meta, name)
            .and_then(|v| v.trim().parse::<i64>().ok())
            // -1 is Scrivener's "none", written explicitly by iOS.
            .filter(|v| *v != -1)
    };
    let extension = |name: &str, invalid: &mut u32| {
        let raw = child_text(meta, name)?;
        let raw = raw.trim();
        if raw.is_empty() {
            None
        } else if is_extension(raw) {
            Some(raw.to_string())
        } else {
            *invalid += 1;
            None
        }
    };
    let section = child(meta, "SectionType");
    ItemMeta {
        label: id("LabelID"),
        status: id("StatusID"),
        include_in_compile: yes("IncludeInCompile"),
        section_type: section
            .and_then(text)
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty()),
        section_child_default: section
            .and_then(|s| s.attribute("ChildDefault"))
            .map(str::to_string)
            .filter(|t| !t.is_empty()),
        page_break_before: yes("PageBreakBefore"),
        compile_as_is: yes("CompileAsIs"),
        custom: child(meta, "CustomMetaData")
            .map(|c| {
                children(c, "MetaDataItem")
                    .filter_map(|item| {
                        Some(CustomValue {
                            field: child_text(item, "FieldID")?,
                            value: child_text(item, "Value").unwrap_or_default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        icon: child_text(meta, "IconFileName").filter(|t| !t.is_empty()),
        file_extension: extension("FileExtension", invalid),
        card_image_extension: extension("IndexCardImageFileExtension", invalid),
        show_synopsis_image: yes("ShowSynopsisImage"),
    }
}

fn read_link(node: roxmltree::Node<'_, '_>) -> Link {
    let target = node
        .attribute("BinderUUID")
        .or_else(|| node.attribute("BinderID"))
        .and_then(key_text);
    // "[Internal Link]" is how an upgraded reference says it points inside.
    let raw = node
        .attribute("Destination")
        .filter(|d| *d != "[Internal Link]");
    let (destination, local_file) = match raw {
        Some(raw) => match local_file_name(raw) {
            Some(name) => (Some(name), true),
            None => (Some(raw.to_string()), false),
        },
        None => (None, false),
    };
    Link {
        target,
        destination,
        local_file,
        text: text(node).filter(|t| !t.is_empty()),
    }
}

/// The schemes whose address is a place on a disk, the writer's or a shared one, rather
/// than a page or a mailbox: a link to another Scrivener project (`x-scrivener-item:`,
/// whose path holds the writer's home folder), and the network shares. What follows the
/// scheme is a path, and its last step is all a line keeps.
const PATH_SCHEMES: &[&str] = &["x-scrivener-item:", "smb:", "afp:", "nfs:", "cifs:"];

/// When `destination` names a file on the writer's disk (a `file:` URL, an
/// absolute or home-relative path, a Windows drive or network path) or on a shared one
/// ([`PATH_SCHEMES`]), its file name alone: a path names the writer's folders, and a
/// project is a file the writer sends to others.
pub(crate) fn local_file_name(destination: &str) -> Option<String> {
    let trimmed = destination.trim();
    let lower = trimmed.to_ascii_lowercase();
    let bytes = trimmed.as_bytes();
    let drive = bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'/' | b'\\');
    let local = lower.starts_with("file:")
        || trimmed.starts_with('/')
        || trimmed.starts_with('~')
        || trimmed.starts_with("\\\\")
        || drive;
    let shared = PATH_SCHEMES.iter().any(|scheme| lower.starts_with(scheme));
    if !local && !shared {
        return None;
    }
    // What follows a shared place's scheme is its path; its query and fragment (`?id=…`)
    // name what is inside the file, never the file.
    let path = if shared {
        let rest = trimmed.split_once(':').map_or("", |(_, rest)| rest);
        rest.split(['?', '#']).next().unwrap_or_default()
    } else {
        trimmed
    };
    let name = path
        .trim_end_matches(['/', '\\'])
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default();
    Some(name.to_string())
}

/// Three numbers between 0 and 1, separated by spaces or commas: `0.993500 0.701213
/// 0.732586` as Scrivener writes them (six decimals, or one to three from iOS), `1 1 1`
/// whole, `0.5,0.5,0.5`, `0.5, 0.5, 0.5`. When spaces alone separate the numbers and none
/// holds a decimal point, a comma inside one is its decimal mark (`0,5 0,5 0,5`, as a
/// French system would write it). A fourth value (an alpha) is ignored; anything that is
/// not three finite numbers is no colour.
pub(crate) fn rgb(raw: &str) -> Option<Rgb> {
    let spaced = raw.trim().contains(char::is_whitespace);
    let parts: Vec<String> = if raw.contains('.') {
        // A decimal point is written: every comma separates (`0.5, 0.25, 1`).
        raw.split(|c: char| c == ',' || c.is_whitespace())
            .filter(|p| !p.is_empty())
            .map(str::to_string)
            .collect()
    } else if spaced {
        raw.split_whitespace()
            .map(|p| p.trim_end_matches(',').replace(',', "."))
            .collect()
    } else {
        raw.split(',').map(|p| p.trim().to_string()).collect()
    };
    let mut parts = parts
        .iter()
        .map(|p| p.parse::<f32>().ok().filter(|v| v.is_finite()));
    let r = parts.next()??;
    let g = parts.next()??;
    let b = parts.next()??;
    Some(Rgb { r, g, b })
}

fn read_blocks(root: roxmltree::Node<'_, '_>) -> Blocks {
    let mut blocks = Blocks::default();
    if let Some(labels) = child(root, "LabelSettings") {
        blocks.labels = read_vocabulary(labels, "DefaultLabelID", "Labels", "Label");
    }
    if let Some(statuses) = child(root, "StatusSettings") {
        blocks.statuses = read_vocabulary(statuses, "DefaultStatusID", "StatusItems", "Status");
    }
    if let Some(keywords) = child(root, "Keywords") {
        blocks.keywords = read_keywords(keywords);
    }
    if let Some(fields) = child(root, "CustomMetaDataSettings") {
        blocks.custom_fields = children(fields, "MetaDataField").map(read_field).collect();
    }
    if let Some(types) = child(root, "SectionTypes") {
        blocks.section_types = read_section_types(types);
    }
    if let Some(collections) = child(root, "Collections") {
        blocks.collections = children(collections, "Collection")
            .map(read_collection)
            .collect();
    }
    if let Some(bookmarks) = child(root, "ProjectBookmarks") {
        blocks
            .project_links
            .extend(children(bookmarks, "Bookmark").map(read_link));
    }
    if let Some(references) = child(root, "ProjectReferences") {
        blocks
            .project_links
            .extend(children(references, "Reference").map(read_link));
    }
    if let Some(targets) = child(root, "ProjectTargets") {
        blocks.targets = ProjectTargets {
            draft: child(targets, "DraftTarget").map(read_target),
            session: child(targets, "SessionTarget").map(read_target),
        };
    }
    blocks.template_folder = child_text(root, "TemplateFolderUUID")
        .or_else(|| child_text(root, "TemplateFolderID"))
        .and_then(|raw| key_text(&raw));
    blocks.bookmarks_folder =
        child_text(root, "BookmarksFolderUUID").and_then(|raw| key_text(&raw));
    if let Some(favorites) = child(root, "Favorites") {
        blocks.favorites = favorites
            .children()
            .filter(|c| c.is_element())
            .filter_map(|c| text(c).and_then(|t| key_text(&t)))
            .collect();
    }
    if let Some(list) = child(root, "AutoCompleteList") {
        blocks.auto_complete = children(list, "Completion").filter_map(text).collect();
    }
    blocks.recent_writing_day =
        child(root, "RecentWritingHistory").and_then(|r| stamp_opt(r.attribute("Date")));
    if let Some(props) = child(root, "ProjectProperties") {
        blocks.properties = ProjectProperties {
            title: child_text(props, "ProjectTitle"),
            abbreviated_title: child_text(props, "AbbreviatedTitle"),
            full_name: child_text(props, "FullName"),
            last_name: child_text(props, "LastName"),
            first_name: child_text(props, "FirstName"),
        };
    }
    blocks
}

fn read_vocabulary(
    node: roxmltree::Node<'_, '_>,
    default: &str,
    list: &str,
    entry: &'static str,
) -> Vocabulary {
    Vocabulary {
        title: child_text(node, "Title"),
        default: child_text(node, default).and_then(|v| v.trim().parse().ok()),
        entries: child(node, list)
            .map(|l| {
                children(l, entry)
                    .filter_map(|e| {
                        Some(VocabEntry {
                            id: e.attribute("ID")?.trim().parse().ok()?,
                            name: text(e).unwrap_or_default(),
                            color: e.attribute("Color").and_then(rgb),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn read_keywords(node: roxmltree::Node<'_, '_>) -> Vec<KeywordRow> {
    let mut rows = Vec::new();
    let mut stack: Vec<(roxmltree::Node<'_, '_>, u16)> =
        children(node, "Keyword").map(|n| (n, 0)).collect();
    stack.reverse();
    while let Some((keyword, depth)) = stack.pop() {
        if let Some(id) = keyword.attribute("ID").and_then(|v| v.trim().parse().ok()) {
            rows.push(KeywordRow {
                depth,
                id,
                title: child_text(keyword, "Title"),
                color: child_text(keyword, "Color").and_then(|c| rgb(&c)),
            });
        } else {
            // A keyword without an id cannot be worn; its children still can,
            // and keep their place one level up.
        }
        if let Some(kids) = child(keyword, "Children") {
            let mut below: Vec<_> = children(kids, "Keyword")
                .map(|n| (n, depth.saturating_add(1)))
                .collect();
            below.reverse();
            stack.extend(below);
        }
    }
    // A dropped keyword leaves its children one level too deep; bring every row
    // back within one level of the row above it so the list stays a tree.
    let mut previous: Option<u16> = None;
    for row in &mut rows {
        let allowed = previous.map_or(0, |p| p.saturating_add(1));
        row.depth = row.depth.min(allowed);
        previous = Some(row.depth);
    }
    rows
}

fn read_field(node: roxmltree::Node<'_, '_>) -> CustomFieldDef {
    let yes_no = |name: &str| node.attribute(name).map(|v| v == "Yes");
    // Format 2.0 gives the title as a child element; format 1.x as the element's
    // own text.
    let title = child_text(node, "Title").or_else(|| {
        text(node)
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
    });
    let options = child(node, "ListOptions");
    let checkbox_default = match node.attribute("Default").map(str::trim) {
        Some(v) if v.eq_ignore_ascii_case("yes") => Some(true),
        Some(v) if v.eq_ignore_ascii_case("no") => Some(false),
        _ => None,
    };
    CustomFieldDef {
        id: node.attribute("ID").unwrap_or_default().to_string(),
        title,
        kind: node.attribute("Type").map(str::to_string),
        wraps: yes_no("Wraps"),
        align: node.attribute("Align").map(str::to_string),
        color: node.attribute("Color").and_then(rgb),
        none_label: options
            .and_then(|o| o.attribute("None"))
            .map(str::to_string),
        options: options
            .map(|o| {
                children(o, "Option")
                    .filter_map(|opt| {
                        Some(ListOption {
                            id: opt.attribute("ID")?.trim().parse().ok()?,
                            name: text(opt).unwrap_or_default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        checkbox_default,
        date_type: node.attribute("DateType").map(str::to_string),
        date_format: child(node, "DateFormat").map(|f| text(f).unwrap_or_default()),
        other_attributes: node
            .attributes()
            .map(|a| a.name())
            .filter(|name| {
                !matches!(
                    *name,
                    "ID" | "Type" | "Wraps" | "Align" | "Color" | "DateType"
                )
            })
            .filter(|name| !(*name == "Default" && checkbox_default.is_some()))
            .map(str::to_string)
            .collect(),
    }
}

fn read_section_types(node: roxmltree::Node<'_, '_>) -> SectionTypes {
    let level = |name: &str| {
        child(node, "LevelTypes")
            .and_then(|l| child(l, name))
            .map(|l| {
                children(l, "Type")
                    .filter_map(text)
                    .map(|t| t.trim().to_string())
                    .collect()
            })
            .unwrap_or_default()
    };
    SectionTypes {
        definitions: child(node, "TypeDefinitions")
            .map(|d| {
                children(d, "Type")
                    .filter_map(|t| {
                        Some(SectionTypeDef {
                            id: t.attribute("ID")?.to_string(),
                            name: text(t).unwrap_or_default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        folders: level("Folders"),
        containers: level("Containers"),
        files: level("Files"),
    }
}

fn read_collection(node: roxmltree::Node<'_, '_>) -> Collection {
    let mut members = Vec::new();
    // Format 2.0: one comma-separated list of UUIDs.
    if let Some(list) = child_text(node, "BinderUUIDs") {
        members.extend(list.split(',').filter_map(key_text));
    }
    // Format 1.x: one element per member.
    if let Some(list) = child(node, "BinderIDs") {
        members.extend(
            children(list, "BinderID").filter_map(|id| text(id).and_then(|t| key_text(&t))),
        );
    }
    Collection {
        kind: node.attribute("Type").map(str::to_string),
        id: node.attribute("ID").map(str::to_string),
        title: child_text(node, "Title"),
        color: node.attribute("Color").and_then(rgb),
        members,
        search: child(node, "SearchSettings").map(|settings| SearchSettings {
            query: text(settings).unwrap_or_default(),
            kind: settings.attribute("Type").map(str::to_string),
            operator: settings.attribute("Operator").map(str::to_string),
            fields: settings
                .attribute("CustomIDs")
                .map(|ids| {
                    ids.split(',')
                        .map(str::trim)
                        .filter(|id| !id.is_empty())
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default(),
        }),
    }
}

fn read_target(node: roxmltree::Node<'_, '_>) -> RawTarget {
    RawTarget {
        value: text(node).unwrap_or_default().trim().to_string(),
        attributes: node
            .attributes()
            .map(|a| (a.name().to_string(), a.value().to_string()))
            .collect(),
    }
}

/// A manifest's `Modified` date as a Unix timestamp, when it has a readable one.
pub(crate) fn modified_seconds(attrs: &BTreeMap<String, String>) -> Option<i64> {
    attrs.get("Modified").and_then(|m| stamp(m).unix_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_header_is_read_without_parsing_the_document() {
        let bytes = br#"<?xml version="1.0" encoding="UTF-8"?>
<ScrivenerProject Template="No" Version="1.5" Creator="SCRWIN-1.9.16.0" Device="PC" Modified="2016-01-01 10:00:00 +0100" ModID="X">
<Binder>"#;
        let header = header(bytes).map(|h| h.attrs);
        assert_eq!(
            header
                .as_ref()
                .and_then(|a| a.get("Version"))
                .map(String::as_str),
            Some("1.5")
        );
        assert_eq!(
            header
                .as_ref()
                .and_then(|a| a.get("Creator"))
                .map(String::as_str),
            Some("SCRWIN-1.9.16.0")
        );
    }

    #[test]
    fn an_ios_stub_header_says_so() {
        let bytes = br#"<ScrivenerProject MobileStub="Yes" Device="Pad"></ScrivenerProject>"#;
        assert!(header(bytes).is_some_and(|h| h.is_mobile_stub()));
    }

    #[test]
    fn a_placeholder_manifest_is_damage_not_a_project() {
        let bytes = br#"<ScrivenerProject Version="1.0" Creator="SCRMAC/2.5/25238" NewProject="Yes"></ScrivenerProject>"#;
        assert_eq!(
            read("p.scrivx", bytes, MAX_ITEMS).err(),
            Some(ManifestDamage::NewProjectPlaceholder)
        );
    }

    #[test]
    fn a_binder_with_no_item_is_damage() {
        let bytes = br#"<ScrivenerProject Version="2.0"><Binder></Binder></ScrivenerProject>"#;
        assert_eq!(
            read("p.scrivx", bytes, MAX_ITEMS).err(),
            Some(ManifestDamage::EmptyBinder)
        );
    }

    #[test]
    fn identifiers_outside_the_grammar_are_dropped_and_counted() {
        let bytes = br#"<ScrivenerProject Version="2.0"><Binder>
<BinderItem UUID="../../../etc/passwd" Type="Text"><MetaData><FileExtension>../x</FileExtension></MetaData></BinderItem>
<BinderItem UUID="4A1D6B18-0952-42EB-A487-D17915DD3BA1" Type="Text"/>
</Binder></ScrivenerProject>"#;
        let read = read("p.scrivx", bytes, MAX_ITEMS);
        let read = match read {
            Ok(read) => read,
            Err(e) => panic!("{e:?}"),
        };
        assert_eq!(read.invalid_identifiers, 2);
        assert_eq!(read.rows[0].key, None);
        assert!(read.rows[1].key.is_some());
    }

    #[test]
    fn a_link_to_a_file_on_disk_keeps_its_name_only() {
        assert_eq!(
            local_file_name("file:///Users/tav/Research/map.png").as_deref(),
            Some("map.png")
        );
        assert_eq!(
            local_file_name("/home/tav/notes.txt").as_deref(),
            Some("notes.txt")
        );
        assert_eq!(
            local_file_name("C:\\Users\\tav\\draft.docx").as_deref(),
            Some("draft.docx")
        );
        assert_eq!(
            local_file_name("\\\\server\\share\\a.pdf").as_deref(),
            Some("a.pdf")
        );
        assert_eq!(
            local_file_name("x-scrivener-item:///Users/jane/Documents/Series.scriv?id=0F2A-11#top")
                .as_deref(),
            Some("Series.scriv")
        );
        assert_eq!(
            local_file_name("smb://host/share/jane/draft.docx").as_deref(),
            Some("draft.docx")
        );
        assert_eq!(
            local_file_name("AFP://host/jane/Notes/").as_deref(),
            Some("Notes")
        );
        assert_eq!(local_file_name("x-scrivener-item:").as_deref(), Some(""));
        assert_eq!(local_file_name("https://example.org/page"), None);
        assert_eq!(local_file_name("mailto:someone@example.org"), None);
    }

    fn field_of(xml: &str) -> CustomFieldDef {
        match roxmltree::Document::parse(xml) {
            Ok(doc) => read_field(doc.root_element()),
            Err(e) => panic!("{e}"),
        }
    }

    /// Scrivener 3.5.2 for Mac spells a checkbox's default `Default="Yes"` (M9); a
    /// spelling this import cannot read stays listed, for the report to say so.
    #[test]
    fn a_checkbox_default_is_read_and_an_unreadable_one_kept_as_written() {
        let ticked = field_of(
            r#"<MetaDataField ID="checked" Type="Checkbox" Default="Yes"><Title>Checked</Title></MetaDataField>"#,
        );
        assert_eq!(ticked.checkbox_default, Some(true));
        assert!(ticked.other_attributes.is_empty());
        let unticked = field_of(r#"<MetaDataField ID="c" Type="Checkbox" Default="No"/>"#);
        assert_eq!(unticked.checkbox_default, Some(false));
        assert!(unticked.other_attributes.is_empty());
        let unread = field_of(r#"<MetaDataField ID="c" Type="Checkbox" Default="Maybe"/>"#);
        assert_eq!(unread.checkbox_default, None);
        assert_eq!(unread.other_attributes, ["Default"]);
    }

    /// A date field's style and its format pattern, as Scrivener 3.5.2 for Mac writes them.
    #[test]
    fn a_date_fields_style_and_format_are_read() {
        let when = field_of(
            r#"<MetaDataField ID="when" Type="Date" DateType="Short"><Title>When</Title><DateFormat></DateFormat></MetaDataField>"#,
        );
        assert_eq!(when.date_type.as_deref(), Some("Short"));
        assert_eq!(when.date_format.as_deref(), Some(""));
        assert!(when.other_attributes.is_empty());
    }

    #[test]
    fn nested_items_come_out_flat_with_their_depth() {
        let bytes = br#"<ScrivenerProject Version="1.0"><Binder>
<BinderItem ID="0" Type="DraftFolder"><Title>Draft</Title><Children>
  <BinderItem ID="3" Type="Folder"><Children><BinderItem ID="4" Type="Text"/></Children></BinderItem>
  <BinderItem ID="5" Type="Text"/>
</Children></BinderItem>
<BinderItem ID="1" Type="ResearchFolder"/>
</Binder></ScrivenerProject>"#;
        let rows = match read("p.scrivx", bytes, MAX_ITEMS) {
            Ok(read) => read.rows,
            Err(e) => panic!("{e:?}"),
        };
        let shape: Vec<(Option<u32>, u16)> = rows.iter().map(|r| (r.number, r.depth)).collect();
        assert_eq!(
            shape,
            vec![
                (Some(0), 0),
                (Some(3), 1),
                (Some(4), 2),
                (Some(5), 1),
                (Some(1), 0)
            ]
        );
    }
}
