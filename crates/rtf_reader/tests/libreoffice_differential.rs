// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The reader's plain text against LibreOffice's, over a local corpus. Opt-in, never in CI.
//!
//! LibreOffice is the oracle here, used as a black box: its text export of each stream is
//! compared with this reader's text, after normalisations that remove what the two
//! legitimately disagree on. The corpus is real writers' projects whose licences are
//! unknown, so it is never committed and this test prints ids and hashes only, never text
//! or paths.
//!
//! ## Running it
//!
//! ```text
//! RTF_READER_DIFFERENTIAL=/path/to/dir \
//!   cargo test -p skribisto-rtf-reader --test libreoffice_differential -- --ignored --nocapture
//! ```
//!
//! The directory holds:
//!
//! * `rtf/<id>.rtf`, one RTF stream per file;
//! * `lo/<id>.txt`, LibreOffice's export of it, made with
//!   `soffice --headless --convert-to "txt:Text (encoded):UTF8"` (25.8 was used, and the
//!   normalisations below are that version's);
//! * optionally `ids.txt`, the ids to compare, one per line (every `rtf/*.rtf` otherwise);
//! * optionally `known.txt`, lines of `<id> <class>` naming a difference already explained,
//!   such as a list label LibreOffice computes that the file does not hold. The test fails
//!   on any difference not listed there.
//!
//! ## What is compared
//!
//! The body's paragraphs in reading order, one per line, each with the list label the file
//! stored; hidden text left out; a line break as a new line; a table row on one line, its
//! cells separated by a tab, as LibreOffice lays one out. Both sides then lose a byte-order
//! mark, have line ends made uniform, and have every run of white space folded into one
//! space ("flat"). LibreOffice's side also has its U+2006 turned into a space: its export
//! writes a double space as U+2006 U+0020 U+2006 U+0020.
//!
//! Two figures come out. *Exact* compares the flat texts. *Normalised* first strips one
//! leading list label from each line on both sides (a bullet, `1.`, `iv)`, `a.`), because
//! LibreOffice prints the label its list table computes where the file stores its own.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rtf_reader::{Block, Document, Inline, Paragraph};

const DIRECTORY: &str = "RTF_READER_DIFFERENTIAL";

#[test]
#[ignore = "needs a local corpus and LibreOffice's output of it; see the module docs"]
fn the_reader_agrees_with_libreoffice_on_a_local_corpus() {
    let Some(directory) = std::env::var_os(DIRECTORY).map(PathBuf::from) else {
        println!("{DIRECTORY} is not set: nothing to compare.");
        return;
    };
    let ids = corpus_ids(&directory);
    let known = known_differences(&directory);

    let mut compared = 0usize;
    let mut exact = 0usize;
    let mut normalised = 0usize;
    let mut refused: Vec<String> = Vec::new();
    let mut unexplained: Vec<String> = Vec::new();
    let mut explained: BTreeMap<String, usize> = BTreeMap::new();

    for id in &ids {
        let Ok(bytes) = std::fs::read(directory.join("rtf").join(format!("{id}.rtf"))) else {
            continue;
        };
        let Ok(oracle) = std::fs::read(directory.join("lo").join(format!("{id}.txt"))) else {
            continue;
        };
        compared += 1;
        let oracle = String::from_utf8_lossy(&oracle).replace('\u{2006}', " ");
        let ours = match rtf_reader::parse(&bytes) {
            Ok(doc) => plain_text(&doc),
            Err(refusal) => {
                refused.push(format!("{id} {:016x} {refusal}", fnv1a(&bytes)));
                continue;
            }
        };
        if flat(&ours) == flat(&oracle) {
            exact += 1;
        }
        let (ours_norm, oracle_norm) =
            (flat(&without_labels(&ours)), flat(&without_labels(&oracle)));
        if ours_norm == oracle_norm {
            normalised += 1;
            continue;
        }
        match known.get(id) {
            Some(class) => *explained.entry(class.clone()).or_insert(0) += 1,
            None => unexplained.push(format!(
                "{id} {:016x} ours {} chars, oracle {} chars, first difference at {}",
                fnv1a(&bytes),
                ours_norm.chars().count(),
                oracle_norm.chars().count(),
                first_difference(&ours_norm, &oracle_norm),
            )),
        }
    }

    println!("streams compared: {compared}");
    println!("exact (flat): {exact}");
    println!("normalised (flat, labels stripped): {normalised}");
    println!("refused: {}", refused.len());
    for line in &refused {
        println!("  refused {line}");
    }
    for (class, count) in &explained {
        println!("known difference {class}: {count}");
    }
    for line in &unexplained {
        println!("  unexplained {line}");
    }
    assert!(
        refused.is_empty() && unexplained.is_empty(),
        "{} refused and {} unexplained difference(s); see the lines above",
        refused.len(),
        unexplained.len()
    );
}

/// The ids to compare: `ids.txt`, else every `rtf/*.rtf`.
fn corpus_ids(directory: &Path) -> Vec<String> {
    if let Ok(list) = std::fs::read_to_string(directory.join("ids.txt")) {
        return list
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect();
    }
    let mut ids: Vec<String> = std::fs::read_dir(directory.join("rtf"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".rtf").map(str::to_string)
        })
        .collect();
    ids.sort();
    ids
}

fn known_differences(directory: &Path) -> BTreeMap<String, String> {
    let Ok(list) = std::fs::read_to_string(directory.join("known.txt")) else {
        return BTreeMap::new();
    };
    list.lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            let id = words.next()?;
            let class = words.next().unwrap_or("unclassified");
            Some((id.to_string(), class.to_string()))
        })
        .collect()
}

/// The body as LibreOffice's text export lays it out, near enough for [`flat`] to compare.
fn plain_text(doc: &Document) -> String {
    let mut out = String::new();
    blocks_text(&doc.body, &mut out);
    out
}

/// Recursive, which this test may be: the reader holds tables to `MAX_TABLE_NESTING`.
fn blocks_text(blocks: &[Block], out: &mut String) {
    for block in blocks {
        match block {
            Block::Paragraph(paragraph) => {
                paragraph_text(paragraph, out);
                out.push('\n');
            }
            Block::Table(table) => {
                for row in &table.rows {
                    for (index, cell) in row.cells.iter().enumerate() {
                        if index > 0 {
                            out.push('\t');
                        }
                        let mut text = String::new();
                        blocks_text(&cell.blocks, &mut text);
                        out.push_str(text.trim_end_matches('\n'));
                    }
                    out.push('\n');
                }
            }
        }
    }
}

fn paragraph_text(paragraph: &Paragraph, out: &mut String) {
    if let Some(label) = &paragraph.label {
        out.push_str(label);
    }
    for inline in &paragraph.inlines {
        match inline {
            Inline::Text(run) if !run.props.hidden => out.push_str(&run.text),
            Inline::Break(_) => out.push('\n'),
            _ => {}
        }
    }
}

/// White space folded: every run of it one space, none at either end.
fn flat(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut blank = false;
    for c in text.chars() {
        if c == '\u{FEFF}' {
            continue;
        }
        if c.is_ascii_whitespace() || matches!(c, '\u{2028}' | '\u{2029}' | '\u{0B}' | '\u{0C}') {
            blank = true;
            continue;
        }
        if blank && !out.is_empty() {
            out.push(' ');
        }
        blank = false;
        out.push(c);
    }
    out
}

/// Each line without one leading list label.
fn without_labels(text: &str) -> String {
    text.replace('\u{FEFF}', "")
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .split('\n')
        .map(strip_label)
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_label(line: &str) -> &str {
    let is_space = |c: char| matches!(c, ' ' | '\t' | '\u{A0}' | '\u{2006}');
    let rest = line.trim_start_matches(is_space);
    let label_end = label_length(rest);
    let Some(after) = label_end.and_then(|n| rest.get(n..)) else {
        return line;
    };
    match after.chars().next() {
        None => after,
        Some(c) if is_space(c) => after.trim_start_matches(is_space),
        Some(_) => line,
    }
}

/// The byte length of a list label at the start of `text`: a bullet glyph, or one to three
/// digits, one to six roman numerals or one letter, followed by `.` or `)`.
fn label_length(text: &str) -> Option<usize> {
    const BULLETS: &str = "\u{2022}\u{25E6}\u{25AA}\u{25CF}\u{25CB}\u{2043}\u{25C6}\u{2713}\u{25A0}\u{25A1}\u{2023}\u{2219}\u{B7}\u{2013}-";
    let first = text.chars().next()?;
    if BULLETS.contains(first) {
        return Some(first.len_utf8());
    }
    let numbered = |accept: &dyn Fn(char) -> bool, most: usize| -> Option<usize> {
        let count = text.chars().take_while(|c| accept(*c)).count();
        if count == 0 || count > most {
            return None;
        }
        let rest = text.get(count..)?;
        rest.starts_with(['.', ')']).then_some(count + 1)
    };
    numbered(&|c| c.is_ascii_digit(), 3)
        .or_else(|| numbered(&|c| "ivxlcdmIVXLCDM".contains(c), 6))
        .or_else(|| numbered(&|c| c.is_ascii_alphabetic(), 1))
}

fn first_difference(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

/// A stable name for a stream that says nothing about its content.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
