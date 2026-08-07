// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `.txt` — the same pipeline, one flag different.
//!
//! Plain text is Markdown with no markup, so it runs through the Markdown
//! scanner rather than a code path of its own. That is not laziness: the
//! prior-art survey's clearest structural lesson is that a *separate* batch or
//! format path is where the crashes live (Manuskript ships one importer per
//! entry point and its folder path is the one with the open crash reports). One
//! path, exercised by every import, stays correct.
//!
//! The one real difference is scene-break spelling. In `.txt` a line of `***` is
//! not thematic-break *syntax*, it is a writer drawing a break — which lands on
//! the same answer, because Skribisto's vocabulary matches the text either way.

use anyhow::Result;

use crate::block::SourceDocument;
use crate::scanner::SourceScanner;
use crate::sources::markdown::MarkdownScanner;

pub struct PlainTextScanner;

impl SourceScanner for PlainTextScanner {
    fn extensions(&self) -> &[&str] {
        &["txt", "text"]
    }

    fn format_name(&self) -> &'static str {
        "plain-text"
    }

    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        MarkdownScanner.scan(bytes, display_name, origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::SourceBlock;
    use skribisto_model::scene_break::SceneBreakTier;

    #[test]
    fn unstructured_text_becomes_one_row() {
        let doc = PlainTextScanner
            .scan(
                b"She turned the corner and the street was gone.",
                "s",
                "s.txt",
            )
            .unwrap();
        assert!(matches!(doc.blocks.as_slice(), [SourceBlock::Prose { .. }]));
    }

    #[test]
    fn a_drawn_break_in_plain_text_is_still_a_break() {
        let doc = PlainTextScanner
            .scan(b"Before.\n\n* * *\n\nAfter.", "s", "s.txt")
            .unwrap();
        assert!(doc.blocks.iter().any(|b| matches!(
            b,
            SourceBlock::SceneBreak {
                tier: SceneBreakTier::Minor
            }
        )));
    }
}
