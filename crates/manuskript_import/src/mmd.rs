// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The metadata-header format shared by `infos.txt`, `summary.txt`,
//! `characters/*.txt` and every outline item.
//!
//! Manuskript calls it MultiMarkdown, but it is its own hand-rolled reader
//! (`load_save/version_1.py::parseMMDFile`) and only the surface resembles MMD. A
//! file is a run of `key:` lines, then one empty line, then the body. Getting this
//! exactly right matters more than anything else in this crate: every structural
//! decision downstream reads a key that comes out of here.
//!
//! The rules, each observed in Manuskript 0.17's reader:
//!
//! 1. A metadata line is `^([^\s].*?):\s*(.*)$` — the key starts with a
//!    non-space, runs to the **first** colon, and needs at least one character
//!    before it.
//! 2. A line beginning with exactly four spaces **continues** the previous value:
//!    the line is stripped and joined with `\n`. Interior indentation is
//!    therefore not preserved, in Manuskript either.
//! 3. The header ends at the first **empty** line. The writer emits two, and the
//!    reader drops one, so the body starts after the second.
//! 4. A key written `None` reads back as the empty key. (The writer spells an
//!    empty key that way.)
//! 5. Any other line inside the header is ignored, and the header continues.
//!
//! # Two places this deliberately does not match Manuskript
//!
//! **A header with no trailing blank line keeps its last key.** Manuskript commits
//! a pending key only when it meets the next key or the blank line, so a file
//! ending mid-header silently loses one — their own
//! `test_ParseMMDFile.py::test_text_hanging_space` asserts that loss. A file
//! Manuskript wrote always has the blank lines, so recovering the key can only
//! affect a hand-edited file, where keeping it is the better answer. The recovery
//! is reported through [`MmdFile::recovered_trailing_key`] rather than done
//! silently.
//!
//! **`_.._` in a key reads back as `:`.** The writer escapes a colon in a key that
//! way (`formatMetaData`), and nothing in Manuskript ever unescapes it, so a
//! character field named "Born: where" is `Born_.._ where` from its first save
//! onward. Values are left alone: the escape is only ever applied to keys.
//!
//! Line endings are normalised first. Manuskript forced `newline="\n"` on read in
//! 0.16.0 and reverted it seven days later in 0.16.1, so both `\r\n` and lone `\r`
//! reach us, and an unnormalised parse would leave a `\r` on the end of every
//! value.

/// The escape Manuskript's writer applies to a colon inside a key.
const COLON_ESCAPE: &str = "_.._";

/// One `key: value` pair, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmdEntry {
    pub key: String,
    pub value: String,
}

/// A parsed header plus the body that followed it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MmdFile {
    /// Every pair, in file order, duplicates included.
    ///
    /// Kept as a list rather than a map because two readers need different
    /// tie-breaks: a character file's first `Color` is the swatch and any later
    /// one is a user field, while Manuskript's own `asDict` path keeps the last
    /// of a repeated key. Both are available, and neither is imposed here.
    pub entries: Vec<MmdEntry>,
    /// Everything after the header.
    pub body: String,
    /// True when the file ended mid-header and the last key was recovered — see
    /// the module docs. The caller reports it; it is never silently dropped.
    pub recovered_trailing_key: bool,
}

impl MmdFile {
    /// The last value for `key`, which is what Manuskript's `asDict` path yields.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.key == key)
            .map(|e| e.value.as_str())
    }

    /// The first value for `key`. What a character file's `Color` needs.
    pub fn first(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.key == key)
            .map(|e| e.value.as_str())
    }

    /// Every key that is not in `known`, in file order and without repeats.
    ///
    /// Manuskript drops an unrecognised key without a word, so a field a future
    /// release adds would vanish here with no trace. This is what lets the import
    /// summary say which keys it did not understand, and in which file.
    pub fn unknown_keys(&self, known: &[&str]) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for e in &self.entries {
            if !known.contains(&e.key.as_str()) && !out.contains(&e.key) {
                out.push(e.key.clone());
            }
        }
        out
    }
}

/// Split one header line into its key and value, or `None` if it is not one.
///
/// Mirrors `^([^\s].*?):\s*(.*)$`: a leading non-space, a colon at index 1 or
/// later (the non-greedy `.*?` still needs the one character `[^\s]` ate), and
/// the value with its leading whitespace consumed by `\s*`.
fn split_header_line(line: &str) -> Option<(String, String)> {
    if line.starts_with(char::is_whitespace) {
        return None;
    }
    let colon = line.find(':')?;
    if colon == 0 {
        return None;
    }
    let key = line[..colon].replace(COLON_ESCAPE, ":");
    // `colon` indexes a one-byte ASCII ':', so `colon + 1` is a char boundary.
    let value = line[colon + 1..].trim_start().to_string();
    Some((key, value))
}

/// Parse a Manuskript metadata file.
///
/// Never fails: a file that is not one at all parses as an empty header and a
/// body, which is exactly how Manuskript treats it, and lets the caller decide
/// whether the missing keys matter.
pub fn parse(text: &str) -> MmdFile {
    let normalised = normalise_newlines(text);

    let mut entries: Vec<MmdEntry> = Vec::new();
    let mut body: Vec<&str> = Vec::new();
    // The key/value being accumulated. `None` until the first header line, which
    // is what makes Manuskript's `if descr:` guard fall out rather than needing a
    // sentinel empty string.
    let mut pending: Option<(String, String)> = None;
    let mut in_body = false;
    let mut recovered_trailing_key = false;

    for line in normalised.split('\n') {
        if in_body {
            body.push(line);
            continue;
        }
        if let Some((key, value)) = split_header_line(line) {
            commit(&mut entries, pending.take());
            pending = Some((key, value));
        } else if line.starts_with("    ") {
            // A continuation of the value above. With no value above, Manuskript
            // appends to its empty accumulator and the result is dropped by the
            // `if descr:` guard, so ignoring the line here is the same outcome.
            if let Some((_, value)) = pending.as_mut() {
                value.push('\n');
                value.push_str(line.trim());
            }
        } else if line.is_empty() {
            in_body = true;
            commit(&mut entries, pending.take());
        }
        // Any other line is ignored and the header continues, as it does there.
    }

    // The file ended without a blank line and a key was still open. Manuskript
    // loses it here; we keep it and say so.
    if let Some(open) = pending.take() {
        recovered_trailing_key = true;
        commit(&mut entries, Some(open));
    }

    // The writer separates header from body with two empty lines and the reader
    // drops the second, so a body that starts empty is one line shorter than it
    // looks.
    if body.first() == Some(&"") {
        body.remove(0);
    }

    MmdFile {
        entries,
        body: body.join("\n"),
        recovered_trailing_key,
    }
}

/// Append a finished pair, applying the `None` key convention.
///
/// A trailing newline can only have come from a continuation line that was all
/// whitespace, so it is trimmed; interior blank lines inside a multi-line value
/// are untouched, since those are what the writer emits for a real empty line.
fn commit(entries: &mut Vec<MmdEntry>, pending: Option<(String, String)>) {
    let Some((key, value)) = pending else { return };
    let key = if key == "None" { String::new() } else { key };
    let value = value.trim_end_matches(['\n', '\r', ' ', '\t']).to_string();
    entries.push(MmdEntry { key, value });
}

/// Collapse `\r\n` and lone `\r` to `\n`, the way Python's universal-newline read
/// does before Manuskript's parser ever sees the text.
fn normalise_newlines(text: &str) -> String {
    if !text.contains('\r') {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact shape `formatMetaData(name, value, 15)` writes for an outline item.
    fn header(pairs: &[(&str, &str)]) -> String {
        pairs
            .iter()
            .map(|(k, v)| format!("{k}:{}{v}\n", " ".repeat(15usize.saturating_sub(k.len()))))
            .collect()
    }

    fn keys(f: &MmdFile) -> Vec<&str> {
        f.entries.iter().map(|e| e.key.as_str()).collect()
    }

    #[test]
    fn an_empty_file_has_no_metadata_and_no_body() {
        let f = parse("");
        assert!(f.entries.is_empty());
        assert_eq!(f.body, "");
        assert!(!f.recovered_trailing_key);
    }

    #[test]
    fn a_header_terminated_by_a_blank_line_parses_in_order() {
        let f =
            parse(&(header(&[("title", "Jerusalem"), ("ID", "35"), ("type", "folder")]) + "\n"));
        assert_eq!(keys(&f), ["title", "ID", "type"]);
        assert_eq!(f.get("ID"), Some("35"));
        assert_eq!(f.body, "");
        assert!(!f.recovered_trailing_key);
    }

    /// What `outlineToMMD` actually produces: header, then `"\n\n"`, then the prose.
    /// The reader drops the second blank line, so the body starts at the real text.
    #[test]
    fn the_writers_two_blank_lines_yield_a_body_with_neither() {
        let src = header(&[("title", "Introduction"), ("ID", "1"), ("type", "md")])
            + "\n\n"
            + "First line.\n\nThird line.";
        let f = parse(&src);
        assert_eq!(keys(&f), ["title", "ID", "type"]);
        assert_eq!(f.body, "First line.\n\nThird line.");
    }

    #[test]
    fn a_key_ends_at_the_first_colon_and_the_value_loses_its_padding() {
        let f = parse("notes:          {P:0:The good news spreads}\n\n");
        assert_eq!(f.get("notes"), Some("{P:0:The good news spreads}"));
    }

    #[test]
    fn a_four_space_line_continues_the_value_and_loses_its_indent() {
        let f = parse("Full Summary:   One.\n    Two.\n        Three.\n\n");
        assert_eq!(f.get("Full Summary"), Some("One.\nTwo.\nThree."));
    }

    /// The writer emits a genuinely empty line inside a multi-line value as a run
    /// of spaces, which is a continuation, so interior blank lines survive.
    #[test]
    fn an_all_space_continuation_keeps_an_interior_blank_line_but_not_a_trailing_one() {
        let f = parse("Notes:          One.\n                 \n    Two.\n\n");
        assert_eq!(f.get("Notes"), Some("One.\n\nTwo."));
        let g = parse("Notes:          One.\n                 \n\n");
        assert_eq!(g.get("Notes"), Some("One."));
    }

    #[test]
    fn the_none_key_reads_back_empty_wherever_it_sits() {
        let start = parse(&(header(&[("None", "hello"), ("title", "X")]) + "\n"));
        assert_eq!(keys(&start), ["", "title"]);
        assert_eq!(start.get(""), Some("hello"));

        let end = parse(&(header(&[("title", "X"), ("None", "hello")]) + "\n"));
        assert_eq!(keys(&end), ["title", ""]);
    }

    /// Manuskript's own `test_text_hanging_space` asserts the last key is LOST
    /// here. We keep it, and flag that we did.
    #[test]
    fn a_header_with_no_trailing_blank_line_keeps_its_last_key_and_says_so() {
        let f = parse(&(header(&[("title", "X"), ("ID", "42"), ("type", "folder")]) + "     "));
        assert_eq!(keys(&f), ["title", "ID", "type"]);
        assert_eq!(f.get("type"), Some("folder"));
        assert!(f.recovered_trailing_key);
        assert_eq!(f.body, "");
    }

    #[test]
    fn a_file_that_is_only_a_header_still_recovers_its_last_key() {
        let f = parse("title:          X\nID:             1");
        assert_eq!(keys(&f), ["title", "ID"]);
        assert!(f.recovered_trailing_key);
    }

    #[test]
    fn an_escaped_colon_is_restored_in_the_key_and_left_alone_in_the_value() {
        let f = parse("Born_.._ where:  a place_.._ named\n\n");
        assert_eq!(keys(&f), ["Born: where"]);
        assert_eq!(f.get("Born: where"), Some("a place_.._ named"));
    }

    /// A character file's first `Color` is the swatch; a later one is a user field.
    #[test]
    fn a_repeated_key_is_reachable_from_either_end() {
        let f = parse("Color:          #ff0000\nColor:          blue eyes\n\n");
        assert_eq!(f.first("Color"), Some("#ff0000"));
        assert_eq!(f.get("Color"), Some("blue eyes"));
        assert_eq!(f.entries.len(), 2);
    }

    #[test]
    fn a_line_that_is_neither_a_pair_nor_a_continuation_is_ignored_and_the_header_goes_on() {
        // One leading space: not a pair (leading whitespace) and not a
        // continuation (fewer than four spaces).
        let f = parse("title:          X\n stray\nID:             7\n\n");
        assert_eq!(keys(&f), ["title", "ID"]);
        assert_eq!(f.get("ID"), Some("7"));
    }

    #[test]
    fn a_colon_in_the_first_column_is_not_a_pair() {
        let f = parse(":not a key\ntitle:          X\n\n");
        assert_eq!(keys(&f), ["title"]);
    }

    #[test]
    fn both_windows_and_classic_mac_line_endings_parse() {
        let crlf = parse("title:          X\r\nID:             9\r\n\r\nBody.");
        assert_eq!(crlf.get("title"), Some("X"));
        assert_eq!(crlf.get("ID"), Some("9"));
        assert_eq!(crlf.body, "Body.");

        let cr = parse("title:          X\rID:             9\r\rBody.");
        assert_eq!(cr.get("ID"), Some("9"));
        assert_eq!(cr.body, "Body.");
    }

    #[test]
    fn unknown_keys_are_reported_once_each_in_file_order() {
        let f = parse(
            &(header(&[
                ("title", "X"),
                ("wardrobe", "a coat"),
                ("ID", "1"),
                ("wardrobe", "a hat"),
            ]) + "\n"),
        );
        assert_eq!(f.unknown_keys(&["title", "ID"]), ["wardrobe"]);
        assert!(f.unknown_keys(&["title", "ID", "wardrobe"]).is_empty());
    }

    /// The shape every real `characters/*.txt` and `infos.txt` has: the writer
    /// ends each pair with a newline and adds nothing after, so the split yields a
    /// final empty element which terminates the header and commits the last key.
    /// Nothing is recovered, and the flag must stay down.
    #[test]
    fn a_file_ending_in_a_single_newline_commits_its_last_key_normally() {
        let f = parse(
            "Name:                Peter\nID:                  0\nColor:               #ff0000\n",
        );
        assert_eq!(keys(&f), ["Name", "ID", "Color"]);
        assert_eq!(f.get("Color"), Some("#ff0000"));
        assert!(!f.recovered_trailing_key);
        assert_eq!(f.body, "");
    }

    #[test]
    fn a_body_with_no_header_at_all_is_still_a_body() {
        let f = parse("\nJust prose, no metadata.");
        assert!(f.entries.is_empty());
        assert_eq!(f.body, "Just prose, no metadata.");
    }
}

/// Parse every metadata file of a real Manuskript project and report what the
/// parser made of it.
///
/// Gated on `SKRIBISTO_MANUSKRIPT_CORPUS`, which names a Manuskript checkout or any
/// directory holding real projects. Skipped, loudly, when it is unset: the
/// fixtures beside it are ours, and this is the only check that runs against files
/// this project did not author.
#[cfg(test)]
mod corpus {
    use super::*;
    use std::path::{Path, PathBuf};

    fn corpus_root() -> Option<PathBuf> {
        std::env::var_os("SKRIBISTO_MANUSKRIPT_CORPUS").map(PathBuf::from)
    }

    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("txt") | Some("md")
            ) {
                out.push(path);
            }
        }
    }

    #[test]
    fn every_metadata_file_in_a_real_project_parses_with_a_key_we_expect() {
        let Some(root) = corpus_root() else {
            eprintln!("SKRIBISTO_MANUSKRIPT_CORPUS unset — skipping the real-project parse");
            return;
        };
        let mut files = Vec::new();
        walk(&root, &mut files);
        assert!(
            !files.is_empty(),
            "no .txt or .md under {} — is this a Manuskript checkout?",
            root.display()
        );

        let mut outline_items = 0usize;
        let mut characters = 0usize;
        for path in &files {
            let Ok(bytes) = std::fs::read(path) else {
                continue;
            };
            let f = parse(&String::from_utf8_lossy(&bytes));
            assert!(
                !f.recovered_trailing_key,
                "{} ended mid-header; Manuskript itself would have lost its last key",
                path.display()
            );
            // `settings.txt` is JSON and has no header; everything else that lives
            // in an outline or characters folder must carry the keys its reader needs.
            let in_outline = path.components().any(|c| c.as_os_str() == "outline");
            let in_characters = path.components().any(|c| c.as_os_str() == "characters");
            if in_outline {
                outline_items += 1;
                assert!(
                    f.get("ID").is_some(),
                    "{} has no ID: — Manuskript drops or crashes on this",
                    path.display()
                );
                assert!(f.get("title").is_some(), "{} has no title:", path.display());
            } else if in_characters {
                characters += 1;
                assert!(f.get("ID").is_some(), "{} has no ID:", path.display());
                assert!(f.get("Name").is_some(), "{} has no Name:", path.display());
            }
        }
        assert!(
            outline_items > 0 && characters > 0,
            "found {outline_items} outline items and {characters} characters — \
             expected a project with both"
        );
        eprintln!("parsed {outline_items} outline items and {characters} characters");
    }
}
