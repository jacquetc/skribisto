// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Markdown nested exactly to the ceiling, and past it.
//!
//! `text-document` reads Markdown on a thread of its own and writes the Djot the
//! importer stores on the thread that asks, recursing once per blockquote in both.
//! Before the ceiling, importing about a kilobyte of Markdown holding 500 nested
//! blockquotes aborted the process from the long operation's 2 MiB thread, and
//! 5,000 aborted `text-document`'s own reader. Every document here is scanned on a
//! thread with that stack, in the debug build the test suite runs in.
//!
//! At the ceiling the document keeps its structure. Past it the words arrive as
//! plain text, one paragraph per line, and the writer is told
//! (`ImportDiagnostic::ProseNotVerbatim`).

use std::path::Path;

use document_ingest::{ImportDiagnostic, ScannerRegistry, SourceBlock, SourceDocument};
use skrib_format::MAX_MARKDOWN_DEPTH;

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 * 1024 * 1024;

/// Scan `markdown` as `hostile.md` on a thread with a long operation's stack. An
/// overflow would abort the test binary, not fail the test, which is exactly the
/// failure this file exists to rule out.
fn scan_on_a_long_operation_stack(markdown: String) -> SourceDocument {
    std::thread::Builder::new()
        .stack_size(LONG_OPERATION_STACK)
        .spawn(move || {
            ScannerRegistry::with_builtin_scanners()
                .scan_bytes(Path::new("hostile.md"), markdown.as_bytes())
        })
        .expect("spawn the scan thread")
        .join()
        .expect("the scan must not unwind")
}

/// A chapter heading, then a paragraph inside `levels` nested blockquotes.
fn quoted(levels: usize) -> String {
    format!(
        "# Chapter One\n\nOpening words.\n\n{}Deep words.\n",
        "> ".repeat(levels)
    )
}

/// The stored Djot of every prose block, in order.
fn prose_djot(doc: &SourceDocument) -> Vec<&str> {
    doc.blocks
        .iter()
        .filter_map(|block| match block {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect()
}

fn plain_text(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .map(SourceBlock::plain_text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn reported_as_not_verbatim(doc: &SourceDocument) -> bool {
    doc.diagnostics
        .iter()
        .any(|d| matches!(d, ImportDiagnostic::ProseNotVerbatim { count, .. } if *count > 0))
}

#[test]
fn markdown_nested_to_the_ceiling_keeps_its_structure_from_a_long_operation_stack() {
    let doc = scan_on_a_long_operation_stack(quoted(MAX_MARKDOWN_DEPTH));

    assert!(
        !reported_as_not_verbatim(&doc),
        "at the ceiling nothing is flattened: {:?}",
        doc.diagnostics
    );
    let djot = prose_djot(&doc);
    assert!(
        djot.iter()
            .any(|djot| djot.contains(&"> ".repeat(MAX_MARKDOWN_DEPTH))),
        "the quotation keeps every level: {djot:?}"
    );
    for djot in djot {
        assert!(skrib_format::djot_depth::check(djot).is_ok());
    }
    assert!(plain_text(&doc).contains("Deep words."));
}

/// One level past the ceiling, five hundred (which aborted the writer on the
/// scanning thread) and five thousand (which aborted `text-document`'s own
/// reader): every word arrives, as prose the next load of the project accepts,
/// and the writer is told. The chapter heading above keeps its own block.
#[test]
fn markdown_nested_past_the_ceiling_keeps_its_words_and_says_so() {
    for levels in [MAX_MARKDOWN_DEPTH + 1, 500, 5_000] {
        let doc = scan_on_a_long_operation_stack(quoted(levels));

        assert!(
            reported_as_not_verbatim(&doc),
            "{levels}: the writer is told"
        );
        assert!(
            doc.blocks
                .iter()
                .any(|b| matches!(b, SourceBlock::Heading { text, .. } if text == "Chapter One")),
            "{levels}: the heading keeps its block"
        );
        let text = plain_text(&doc);
        assert!(text.contains("Opening words."), "{levels}: {text:?}");
        assert!(text.contains("Deep words."), "{levels}: {text:?}");
        for djot in prose_djot(&doc) {
            assert!(
                skrib_format::djot_depth::check(djot).is_ok(),
                "{levels}: the load must accept {djot:.80?}"
            );
        }
    }
}
