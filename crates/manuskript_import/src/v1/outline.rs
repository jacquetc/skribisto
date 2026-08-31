// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Rebuilding the outline tree from `outline/`.
//!
//! Manuskript keeps **no index file**. The tree is the directory tree, and sibling
//! order is a zero-padded numeric prefix on each name — the row index, padded to
//! the width of the sibling count, so `9` siblings give `0-`… and `10` give `00-`.
//! A folder is a directory holding `folder.txt`; a text item is a `.md` file, and
//! is always a leaf, because Manuskript refuses to store prose on a folder.
//!
//! The title comes from the `title:` metadata and never from the file name.
//! `slugify` keeps only ASCII letters and digits, maps whitespace to `_` and
//! *everything else* to `-`, so a Japanese or Cyrillic title becomes a run of
//! dashes on disk. That is intentional upstream — it keeps the tree portable — and
//! it means the name is not reversible.
//!
//! # Three places this reads a project Manuskript would not
//!
//! Each is a case where its own loader gives up, and where giving up loses the
//! writer's work for no gain:
//!
//! - a `folder.txt` with no `ID:` **aborts** its whole load (`md.pop('ID')`, with
//!   no guard on that branch). Here the folder is kept and an id is synthesised.
//! - a `.md` with no `ID:` is dropped silently. Here it is kept.
//! - a directory with no `folder.txt` is skipped **with everything beneath it**.
//!   Here it becomes a plain folder titled from its slug, so the scenes inside it
//!   still arrive.
//!
//! All three are reported rather than done quietly.

use std::collections::BTreeMap;

use crate::mmd;
use crate::model::{OutlineItem, OutlineKind};
use crate::source::ManuskriptSource;

/// The `outline/` prefix every member of the tree carries.
pub const OUTLINE_DIR: &str = "outline/";
/// The member naming a directory's own metadata.
const FOLDER_FILE: &str = "folder.txt";

/// Every metadata key an outline item may carry.
///
/// The live set is the `Outline` enum's member names. `text` is here although it
/// is never written as a key — the body carries it — because a hand-edited file
/// might, and naming it as known keeps it out of the unknown-key report.
/// `summarySentance` is the pre-0.3.0 misspelling, still present in real
/// format-0 data.
const KNOWN_KEYS: &[&str] = &[
    "title",
    "ID",
    "type",
    "summarySentence",
    "summarySentance",
    "summaryFull",
    "POV",
    "notes",
    "label",
    "status",
    "compile",
    "text",
    "wordCount",
    "goal",
    "goalPercentage",
    "setGoal",
    "textFormat",
    "revisions",
    "customIcon",
    "charCount",
];

/// A directory in the reconstructed tree.
#[derive(Default)]
struct Dir<'a> {
    /// Child directories, by name.
    dirs: BTreeMap<&'a str, Dir<'a>>,
    /// Child files, by name, valued by their full member key.
    files: BTreeMap<&'a str, &'a str>,
}

impl<'a> Dir<'a> {
    fn insert(&mut self, parts: &[&'a str], member: &'a str) {
        match parts {
            [] => {}
            [name] => {
                self.files.insert(name, member);
            }
            [head, rest @ ..] => {
                self.dirs.entry(head).or_default().insert(rest, member);
            }
        }
    }
}

/// Read the whole outline. Never fails: an unreadable corner is reported and the
/// rest of the manuscript still arrives.
pub fn read(src: &ManuskriptSource, notices: &mut Vec<String>) -> Vec<OutlineItem> {
    let mut root = Dir::default();
    for member in src.members_under(OUTLINE_DIR) {
        let Some(rest) = member.strip_prefix(OUTLINE_DIR) else {
            continue;
        };
        let parts: Vec<&str> = rest.split('/').collect();
        root.insert(&parts, member);
    }
    let mut synthetic_ids = 0usize;
    let items = read_dir(src, &root, notices, &mut synthetic_ids);
    if synthetic_ids > 0 {
        notices.push(format!(
            "{synthetic_ids} outline {} no ID of its own and was given one. Manuskript drops \
             such a row, or stops loading the project entirely when it is a folder.",
            plural(synthetic_ids, "row had", "rows had")
        ));
    }
    items
}

/// Read one directory's children, in sibling order.
fn read_dir(
    src: &ManuskriptSource,
    dir: &Dir<'_>,
    notices: &mut Vec<String>,
    synthetic_ids: &mut usize,
) -> Vec<OutlineItem> {
    // One ordered list of both kinds, so a folder and a file interleave by their
    // numeric prefix exactly as they do in the binder.
    let mut entries: Vec<(&str, Entry<'_>)> = Vec::new();
    for (name, sub) in &dir.dirs {
        entries.push((name, Entry::Folder(sub)));
    }
    for (name, member) in &dir.files {
        if *name == FOLDER_FILE {
            continue; // this directory's own metadata, not a child
        }
        entries.push((name, Entry::File(member)));
    }
    entries.sort_by(|a, b| sibling_order(a.0, b.0));

    let mut out = Vec::new();
    for (name, entry) in entries {
        match entry {
            Entry::Folder(sub) => {
                let mut item = match sub.files.get(FOLDER_FILE) {
                    Some(member) => {
                        read_item(src, member, OutlineKind::Folder, notices, synthetic_ids)
                    }
                    None => {
                        notices.push(format!(
                            "'{OUTLINE_DIR}{name}' has no {FOLDER_FILE}, so it was imported as a \
                             plain folder named after the directory. Manuskript skips such a \
                             directory and everything inside it."
                        ));
                        *synthetic_ids += 1;
                        OutlineItem {
                            id: None,
                            title: title_from_slug(name),
                            kind: OutlineKind::Folder,
                            ..OutlineItem::default()
                        }
                    }
                };
                item.children = read_dir(src, sub, notices, synthetic_ids);
                out.push(item);
            }
            Entry::File(member) => {
                if !name.ends_with(".md") {
                    notices.push(format!(
                        "'{member}' is not a Manuskript text item and was left out of the outline."
                    ));
                    continue;
                }
                out.push(read_item(
                    src,
                    member,
                    OutlineKind::Text,
                    notices,
                    synthetic_ids,
                ));
            }
        }
    }
    out
}

enum Entry<'a> {
    Folder(&'a Dir<'a>),
    File(&'a str),
}

/// Parse one item file.
fn read_item(
    src: &ManuskriptSource,
    member: &str,
    kind: OutlineKind,
    notices: &mut Vec<String>,
    synthetic_ids: &mut usize,
) -> OutlineItem {
    let Some(text) = src.text(member) else {
        *synthetic_ids += 1;
        return OutlineItem {
            kind,
            title: String::new(),
            ..OutlineItem::default()
        };
    };
    let file = mmd::parse(&text);
    if file.recovered_trailing_key {
        notices.push(format!(
            "'{member}' ended without a blank line after its metadata. Its last field was kept; \
             Manuskript loses it."
        ));
    }
    for key in file.unknown_keys(KNOWN_KEYS) {
        notices.push(format!(
            "'{member}' carries a field this importer does not know, '{key}'. It was not \
             imported."
        ));
    }

    let id = file.get("ID").map(str::to_string).filter(|s| !s.is_empty());
    if id.is_none() {
        *synthetic_ids += 1;
    }

    // A folder is a folder because of where it sits, whatever `type:` claims: the
    // file name is the authority Manuskript itself uses when rebuilding the tree.
    let declared = file.get("type").unwrap_or_default();
    let converted = crate::prose::to_djot(declared, &file.body, member);
    if let Some(notice) = converted.notice {
        notices.push(notice);
    }
    let body = converted.djot;

    OutlineItem {
        id,
        title: file.get("title").unwrap_or_default().to_string(),
        kind,
        // Accept the 2016 misspelling as well as the current key.
        summary_sentence: file
            .get("summarySentence")
            .or_else(|| file.get("summarySentance"))
            .unwrap_or_default()
            .to_string(),
        summary_full: file.get("summaryFull").unwrap_or_default().to_string(),
        pov: file
            .get("POV")
            .map(str::to_string)
            .filter(|s| !s.is_empty()),
        notes: file.get("notes").unwrap_or_default().to_string(),
        label: file.get("label").and_then(parse_index),
        status: file.get("status").and_then(parse_index),
        compile: file
            .get("compile")
            .and_then(|v| v.trim().parse::<i64>().ok()),
        set_goal: file
            .get("setGoal")
            .and_then(|v| v.trim().parse::<i64>().ok())
            .filter(|n| *n > 0),
        custom_icon: file.get("customIcon").unwrap_or_default().to_string(),
        text: if kind == OutlineKind::Folder {
            // Manuskript refuses to store prose on a folder, so anything after the
            // header of a `folder.txt` is not manuscript text.
            String::new()
        } else {
            body
        },
        children: Vec::new(),
    }
}

/// Read a stored vocabulary index.
///
/// Deliberately **not** defaulting a missing or unreadable value to `0` or to `1`.
/// Manuskript omits a falsy value when writing, so an absent key and a stored `0`
/// are the same state — no label, no status — and inventing one would mark rows
/// the writer never marked. `Some(0)` is filtered out here so the model's 1-based
/// lookup never has to think about it.
fn parse_index(raw: &str) -> Option<usize> {
    raw.trim().parse::<usize>().ok().filter(|n| *n > 0)
}

/// Compare two sibling names by their leading number, then by name.
///
/// Manuskript sorts full path strings and relies on the zero padding to make that
/// numeric. The padding is correct in anything Manuskript wrote, so this agrees
/// with it there; where a hand edit or a merge has left `1-`, `2-`, `10-`
/// unpadded, a plain string sort would put `10` between `1` and `2` and this does
/// not.
pub(crate) fn sibling_order(a: &str, b: &str) -> std::cmp::Ordering {
    match (leading_number(a), leading_number(b)) {
        (Some(x), Some(y)) if x != y => x.cmp(&y),
        _ => a.cmp(b),
    }
}

fn leading_number(name: &str) -> Option<u64> {
    let digits: String = name.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

/// Recover a readable title from a slug, for a directory that has no metadata.
///
/// The row-index prefix goes, `_` becomes a space again, and `-` stays: it is what
/// `slugify` writes for every character it could not keep, so it is not reversible
/// and guessing would be worse than showing it.
fn title_from_slug(name: &str) -> String {
    let without_index = match name.split_once('-') {
        Some((head, rest)) if !head.is_empty() && head.chars().all(|c| c.is_ascii_digit()) => rest,
        _ => name,
    };
    without_index.replace('_', " ")
}

fn plural(n: usize, one: &'static str, many: &'static str) -> &'static str {
    if n == 1 { one } else { many }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `.md` or `folder.txt` body, in the shape `outlineToMMD` writes.
    fn item(pairs: &[(&str, &str)], body: &str) -> String {
        let header: String = pairs
            .iter()
            .map(|(k, v)| format!("{k}:{}{v}\n", " ".repeat(15usize.saturating_sub(k.len()))))
            .collect();
        format!("{header}\n\n{body}")
    }

    fn read_members(members: &[(&str, &str)]) -> (Vec<OutlineItem>, Vec<String>) {
        let src = ManuskriptSource::for_tests(members);
        let mut notices = Vec::new();
        let items = read(&src, &mut notices);
        (items, notices)
    }

    fn titles(items: &[OutlineItem]) -> Vec<&str> {
        items.iter().map(|i| i.title.as_str()).collect()
    }

    #[test]
    fn a_directory_with_folder_txt_is_a_folder_and_a_md_is_a_leaf() {
        let folder = item(
            &[("title", "Chapter 1"), ("ID", "5"), ("type", "folder")],
            "",
        );
        let scene = item(
            &[("title", "Introduction"), ("ID", "1"), ("type", "md")],
            "First words.",
        );
        let (items, notices) = read_members(&[
            ("outline/0-Chapter_1/folder.txt", &folder),
            ("outline/0-Chapter_1/0-Introduction.md", &scene),
        ]);
        assert!(notices.is_empty(), "{notices:?}");
        assert_eq!(titles(&items), ["Chapter 1"]);
        assert!(items[0].is_folder());
        assert_eq!(titles(&items[0].children), ["Introduction"]);
        assert!(!items[0].children[0].is_folder());
        assert_eq!(items[0].children[0].text, "First words.");
    }

    /// Sibling order lives only in the numeric prefix. Manuskript pads it to the
    /// sibling-count width so a plain string sort is numeric; where a hand edit has
    /// left the padding inconsistent, a string sort would put 10 between 1 and 2.
    #[test]
    fn siblings_order_by_their_number_even_when_the_padding_is_inconsistent() {
        let mk = |t: &str, id: &str| item(&[("title", t), ("ID", id), ("type", "md")], "");
        let (items, _) = read_members(&[
            ("outline/1-One.md", &mk("One", "1")),
            ("outline/2-Two.md", &mk("Two", "2")),
            ("outline/10-Ten.md", &mk("Ten", "10")),
        ]);
        assert_eq!(titles(&items), ["One", "Two", "Ten"]);
    }

    #[test]
    fn a_folder_and_a_file_interleave_by_their_number() {
        let f = item(&[("title", "Second"), ("ID", "2"), ("type", "folder")], "");
        let mk = |t: &str, id: &str| item(&[("title", t), ("ID", id), ("type", "md")], "");
        let (items, _) = read_members(&[
            ("outline/0-First.md", &mk("First", "1")),
            ("outline/1-Second/folder.txt", &f),
            ("outline/2-Third.md", &mk("Third", "3")),
        ]);
        assert_eq!(titles(&items), ["First", "Second", "Third"]);
    }

    /// Manuskript skips such a directory and everything under it. Here the scenes
    /// inside still arrive.
    #[test]
    fn a_directory_with_no_folder_txt_keeps_its_children_and_is_reported() {
        let scene = item(&[("title", "Kept"), ("ID", "1"), ("type", "md")], "Prose.");
        let (items, notices) = read_members(&[("outline/0-Lost_Part/0-Kept.md", &scene)]);
        assert_eq!(titles(&items), ["Lost Part"]);
        assert!(items[0].is_folder());
        assert_eq!(titles(&items[0].children), ["Kept"]);
        assert!(
            notices.iter().any(|n| n.contains("no folder.txt")),
            "{notices:?}"
        );
    }

    /// Manuskript drops a `.md` with no ID and aborts on a `folder.txt` with none.
    #[test]
    fn a_row_with_no_id_is_kept_and_counted() {
        let scene = item(&[("title", "Nameless"), ("type", "md")], "Prose.");
        let (items, notices) = read_members(&[("outline/0-Nameless.md", &scene)]);
        assert_eq!(titles(&items), ["Nameless"]);
        assert!(items[0].id.is_none());
        assert_eq!(items[0].text, "Prose.");
        assert!(
            notices.iter().any(|n| n.contains("no ID of its own")),
            "{notices:?}"
        );
    }

    #[test]
    fn the_title_comes_from_the_metadata_and_not_from_the_slug() {
        // What `slugify` does to a title it cannot keep: every non-ASCII character
        // becomes a dash, so the file name is not reversible.
        let scene = item(
            &[("title", "シャーロック"), ("ID", "1"), ("type", "md")],
            "",
        );
        let (items, _) = read_members(&[("outline/0-------.md", &scene)]);
        assert_eq!(titles(&items), ["シャーロック"]);
    }

    #[test]
    fn a_vocabulary_index_of_zero_is_no_index() {
        let mk = |label: &str, status: &str| {
            item(
                &[
                    ("title", "A"),
                    ("ID", "1"),
                    ("type", "md"),
                    ("label", label),
                    ("status", status),
                ],
                "",
            )
        };
        let (items, _) = read_members(&[("outline/0-A.md", &mk("3", "4"))]);
        assert_eq!(items[0].label, Some(3));
        assert_eq!(items[0].status, Some(4));

        let (items, _) = read_members(&[("outline/0-A.md", &mk("0", "0"))]);
        assert!(items[0].label.is_none());
        assert!(items[0].status.is_none());
    }

    #[test]
    fn a_folder_never_carries_prose_however_the_file_ends() {
        let folder = item(
            &[("title", "Part"), ("ID", "1"), ("type", "folder")],
            "Text a hand edit put here.",
        );
        let (items, _) = read_members(&[("outline/0-Part/folder.txt", &folder)]);
        assert_eq!(items[0].text, "");
    }

    /// A pre-0.3.0 body. Manuskript flattens it through `HTML2PlainText` and loses
    /// the markup; converting to Djot keeps the paragraphs and the emphasis.
    #[test]
    fn an_html_body_keeps_its_shape_instead_of_being_flattened() {
        let scene = item(
            &[("title", "Old"), ("ID", "1"), ("type", "html")],
            "<p>One <em>emphatic</em> line.</p><p>And another.</p>",
        );
        let (items, notices) = read_members(&[("outline/0-Old.md", &scene)]);
        assert!(notices.is_empty(), "{notices:?}");
        assert!(items[0].text.contains("_emphatic_"), "{}", items[0].text);
        assert!(items[0].text.contains("And another."), "{}", items[0].text);
    }

    #[test]
    fn a_field_the_importer_does_not_know_is_named_rather_than_dropped_quietly() {
        let scene = item(
            &[
                ("title", "A"),
                ("ID", "1"),
                ("type", "md"),
                ("wardrobe", "a coat"),
            ],
            "",
        );
        let (_, notices) = read_members(&[("outline/0-A.md", &scene)]);
        assert!(
            notices.iter().any(|n| n.contains("wardrobe")),
            "{notices:?}"
        );
    }

    #[test]
    fn a_file_that_is_not_a_text_item_is_left_out_and_reported() {
        let (items, notices) = read_members(&[("outline/notes.rtf", "junk")]);
        assert!(items.is_empty());
        assert!(
            notices.iter().any(|n| n.contains("notes.rtf")),
            "{notices:?}"
        );
    }

    #[test]
    fn the_legacy_summary_spelling_is_accepted() {
        let scene = item(
            &[
                ("title", "A"),
                ("ID", "1"),
                ("type", "md"),
                ("summarySentance", "The old spelling."),
            ],
            "",
        );
        let (items, notices) = read_members(&[("outline/0-A.md", &scene)]);
        assert_eq!(items[0].summary_sentence, "The old spelling.");
        assert!(notices.is_empty(), "{notices:?}");
    }

    #[test]
    fn a_goal_of_zero_is_no_goal() {
        let mk = |goal: &str| {
            item(
                &[
                    ("title", "A"),
                    ("ID", "1"),
                    ("type", "md"),
                    ("setGoal", goal),
                ],
                "",
            )
        };
        let (items, _) = read_members(&[("outline/0-A.md", &mk("1000"))]);
        assert_eq!(items[0].set_goal, Some(1000));
        let (items, _) = read_members(&[("outline/0-A.md", &mk("0"))]);
        assert!(items[0].set_goal.is_none());
    }

    #[test]
    fn a_slug_title_drops_its_index_and_restores_its_spaces() {
        assert_eq!(title_from_slug("0-Chapter_One"), "Chapter One");
        assert_eq!(title_from_slug("00-A_B_C"), "A B C");
        // Not an index prefix, so nothing is stripped.
        assert_eq!(title_from_slug("Chapter_One"), "Chapter One");
        // `slugify` writes a dash for anything it could not keep; it is not
        // reversible, so it is left visible rather than guessed at.
        assert_eq!(title_from_slug("0-------"), "------");
    }
}
