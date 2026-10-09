// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Helpers the reader's integration tests share.
//!
//! Every fixture is built here, in code, from synthetic text: no file from a real project
//! is ever committed. RTF is written as raw strings, with one substitution: `@u` stands for
//! the control word that starts with a backslash and a `u` (`@u233`, `@uc0`). A backslash,
//! a `u` and four hexadecimal digits in a source file are too easily taken for a Unicode
//! escape by the tools that handle the file, which then silently rewrite the fixture.

#![allow(dead_code)]

use rtf_reader::{Block, Document, Inline, Paragraph, Run, paragraphs};

/// `template` as RTF bytes, `@u` written out as the backslash control word it stands for.
pub fn rtf(template: &str) -> Vec<u8> {
    let mut out = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '@' && chars.peek() == Some(&'u') {
            out.push('\\');
        } else {
            out.push(c);
        }
    }
    out.into_bytes()
}

/// Parse `template` (see [`rtf`]), failing the test on a refusal.
pub fn parse(template: &str) -> Document {
    match rtf_reader::parse(&rtf(template)) {
        Ok(doc) => doc,
        Err(refusal) => panic!("refused: {refusal}"),
    }
}

/// Parse raw bytes, failing the test on a refusal.
pub fn parse_bytes(bytes: &[u8]) -> Document {
    match rtf_reader::parse(bytes) {
        Ok(doc) => doc,
        Err(refusal) => panic!("refused: {refusal}"),
    }
}

/// The text of a paragraph's runs, hidden ones included, line breaks as `\n`.
pub fn paragraph_text(paragraph: &Paragraph) -> String {
    let mut out = String::new();
    for inline in &paragraph.inlines {
        match inline {
            Inline::Text(run) => out.push_str(&run.text),
            Inline::Break(_) => out.push('\n'),
            _ => {}
        }
    }
    out
}

/// The text of every paragraph of `blocks`, in reading order.
pub fn texts(blocks: &[Block]) -> Vec<String> {
    paragraphs(blocks).map(paragraph_text).collect()
}

/// The text of every body paragraph.
pub fn body(doc: &Document) -> Vec<String> {
    texts(&doc.body)
}

/// The first body paragraph.
pub fn first(doc: &Document) -> &Paragraph {
    match paragraphs(&doc.body).next() {
        Some(paragraph) => paragraph,
        None => panic!("no paragraph in {doc:?}"),
    }
}

/// The runs of the first body paragraph.
pub fn runs(doc: &Document) -> Vec<&Run> {
    first(doc).runs().collect()
}

/// The run holding `needle`.
pub fn run_with<'a>(doc: &'a Document, needle: &str) -> &'a Run {
    for paragraph in paragraphs(&doc.body) {
        for run in paragraph.runs() {
            if run.text.contains(needle) {
                return run;
            }
        }
    }
    panic!("no run holds {needle:?}");
}

/// A Cocoa document around `body`: the header macOS writes, paragraphs ended by a
/// backslash before a line feed.
pub fn cocoa(body: &str) -> String {
    format!(
        "{{\\rtf1\\ansi\\ansicpg1252\\cocoartf2822\n\\cocoatextscaling0\\cocoaplatform0\
         {{\\fonttbl\\f0\\fswiss\\fcharset0 Helvetica;\\f1\\froman\\fcharset0 Palatino-Bold;}}\n\
         {{\\colortbl;\\red255\\green255\\blue255;\\red251\\green2\\blue7;}}\n\
         {{\\*\\expandedcolortbl;;\\cssrgb\\c100000\\c0\\c0;}}\n\
         \\pard\\tx560\\tx1120\\pardirnatural\\partightenfactor0\n\n\\f0\\fs24 \\cf0 {body}}}"
    )
}

/// A Windows/Qt document around `body`, as Scrivener for Windows writes it: every font
/// declared `\fmodern`, every run a group of its own.
pub fn windows(body: &str) -> String {
    format!(
        "{{\\rtf1\\ansi\\ansicpg1252\\uc1\\deff0{{\\fonttbl{{\\f0\\fmodern\\fcharset0\\fprq2 Cambria;}}\
         {{\\f1\\fmodern\\fcharset0\\fprq2 Calibri;}}}}\n\
         {{\\colortbl;\\red0\\green0\\blue0;\\red255\\green0\\blue0;}}\n\
         \\paperw12240\\paperh15840\\margl1800\\margr1800\\margt1440\\margb1440\\fet2\\ftnbj\\aenddoc\n\
         \\pard\\plain {body}}}"
    )
}

/// The heap the model holds, picture data aside: every list's allocated room and every
/// string's, measured on the model itself. What [`rtf_reader::Stats::document_bytes`]
/// accounts has to keep up with it, or a limit on the one is no limit on the other.
pub fn model_heap_bytes(doc: &Document) -> usize {
    use rtf_reader::{
        Annotation, Cell, Field, Font, List, ListLevel, ListOverride, Note, Picture, Row, Style,
    };
    use std::mem::size_of;

    fn list<T>(items: &[T], capacity: usize) -> usize {
        debug_assert!(items.len() <= capacity);
        capacity * size_of::<T>()
    }
    fn text(s: &str, capacity: usize) -> usize {
        debug_assert!(s.len() <= capacity);
        capacity
    }
    fn optional(s: &Option<String>) -> usize {
        s.as_ref().map_or(0, |s| text(s, s.capacity()))
    }
    fn blocks(blocks: &Vec<Block>) -> usize {
        let mut total = list(blocks, blocks.capacity());
        let mut stack: Vec<&Vec<Block>> = vec![blocks];
        while let Some(blocks) = stack.pop() {
            for block in blocks {
                match block {
                    Block::Paragraph(paragraph) => {
                        total += optional(&paragraph.label);
                        total += list(&paragraph.inlines, paragraph.inlines.capacity());
                        for inline in &paragraph.inlines {
                            match inline {
                                Inline::Text(run) => total += text(&run.text, run.text.capacity()),
                                Inline::AnnotationRangeStart(tag)
                                | Inline::AnnotationRangeEnd(tag) => {
                                    total += text(tag, tag.capacity())
                                }
                                _ => {}
                            }
                        }
                    }
                    Block::Table(table) => {
                        total += list::<Row>(&table.rows, table.rows.capacity());
                        for row in &table.rows {
                            total += list::<Cell>(&row.cells, row.cells.capacity());
                            for cell in &row.cells {
                                total += list(&cell.blocks, cell.blocks.capacity());
                                stack.push(&cell.blocks);
                            }
                        }
                    }
                }
            }
        }
        total
    }

    let mut total = blocks(&doc.body);
    total += list::<Note>(&doc.notes, doc.notes.capacity());
    total += doc.notes.iter().map(|n| blocks(&n.blocks)).sum::<usize>();
    total += list::<Annotation>(&doc.annotations, doc.annotations.capacity());
    for a in &doc.annotations {
        total += optional(&a.initials) + optional(&a.author) + optional(&a.date);
        total += optional(&a.reference) + optional(&a.parent) + blocks(&a.blocks);
    }
    total += list::<Field>(&doc.fields, doc.fields.capacity());
    total += doc
        .fields
        .iter()
        .map(|f| text(&f.instruction, f.instruction.capacity()))
        .sum::<usize>();
    total += list::<Picture>(&doc.pictures, doc.pictures.capacity());
    total += doc
        .pictures
        .iter()
        .map(|p| optional(&p.name))
        .sum::<usize>();
    total += list::<Font>(&doc.fonts, doc.fonts.capacity());
    total += doc
        .fonts
        .iter()
        .map(|f| text(&f.name, f.name.capacity()))
        .sum::<usize>();
    total += list::<Style>(&doc.styles, doc.styles.capacity());
    total += doc
        .styles
        .iter()
        .map(|s| text(&s.name, s.name.capacity()))
        .sum::<usize>();
    total += list::<List>(&doc.lists, doc.lists.capacity());
    for l in &doc.lists {
        total += list::<ListLevel>(&l.levels, l.levels.capacity());
        total += l
            .levels
            .iter()
            .map(|level| optional(&level.marker))
            .sum::<usize>();
    }
    total += list::<ListOverride>(&doc.list_overrides, doc.list_overrides.capacity());
    total += list(&doc.colors, doc.colors.capacity());
    let info = &doc.info;
    total += optional(&info.title) + optional(&info.subject) + optional(&info.author);
    total += optional(&info.keywords) + optional(&info.comment) + optional(&doc.header.generator);
    total
}

/// How much more heap than it accounts a model may hold: a list's doubling and a string's
/// keep either within twice what they hold, plus the first allocation of each of the
/// document's own lists, which the reading does not count one by one.
pub const HEAP_SLACK: usize = 8 << 10;

/// Whether the heap `doc` holds stays within what its reading accounted.
pub fn heap_within_accounting(doc: &Document) -> Result<(), String> {
    let heap = model_heap_bytes(doc);
    let accounted = doc.stats.document_bytes;
    if heap <= 2 * accounted + HEAP_SLACK {
        Ok(())
    } else {
        Err(format!(
            "the model holds {heap} bytes of heap, more than twice the {accounted} its reading accounted"
        ))
    }
}
