// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **The text of one searchable field**, in the two shapes it comes in.
//!
//! Its own module because two use cases now reach for a field's text and must
//! reach for it the same way: `run_search` over every field in the manuscript,
//! `occurrences_for_result` over the one field a writer opened. Which of the two
//! matchers a field goes through is decided by which shape it is, so a second
//! copy of this enum would be a second answer to that.

use crate::corpus_cache::Corpus;
use std::sync::Arc;
use text_document::matching::MatchOptions;

/// The two kinds of searchable text, which are not the same kind of thing at all.
pub enum FieldText {
    /// A plain string that lives on the item — a title, a label. A dozen characters, no
    /// markup, no parser. Folding it costs nothing, so it is folded on the spot and not
    /// cached: it changes whenever the writer renames anything, and caching it would fill the
    /// cache with entries nobody looks up twice.
    Plain(String),
    /// A scene's **prose**: parsed out of its Djot and folded, once, and kept between
    /// keystrokes (see [`crate::corpus_cache`]). This is where all the cost was.
    Prose(Arc<Corpus>),
}

impl FieldText {
    /// The text a snippet is cut from — the prose the writer sees, never the markup.
    pub fn as_str(&self) -> &str {
        match self {
            FieldText::Plain(s) => s,
            FieldText::Prose(c) => c.source(),
        }
    }

    /// Every occurrence of `query` in this field, as `(char_start, char_len)`.
    ///
    /// **The one place that picks between the two matchers**, and it has to be one
    /// place. Three use cases ask a field where the query occurs — `run_search` to
    /// count them, `occurrences_for_result` to list them, `replace_in_project` to
    /// work out which of them a writer named — and the third *identifies* an
    /// occurrence by an offset the first two produced. A second copy of this choice
    /// is a second coordinate system, and the failure it buys is silent: an offset
    /// resolved against the wrong list of hits rewrites the wrong word.
    ///
    /// The two arms are not interchangeable implementations of one thing. A title
    /// is a dozen characters and folds on the spot; prose is folded once and kept
    /// between keystrokes, which is where all the cost of a search went. Which is
    /// also why a `Prose` field only reads `whole_word` off `options`: case and
    /// diacritics were already decided when the corpus was folded, and the caller
    /// owes it a corpus folded under *these* options' [`MatchOptions::fold_spec`].
    pub fn hits(&self, query: &str, options: MatchOptions) -> Vec<(usize, usize)> {
        match self {
            FieldText::Plain(s) => crate::matching::occurrences(s, query, options),
            FieldText::Prose(corpus) => corpus
                .find_all(query, options.whole_word)
                .into_iter()
                .map(|m| (m.char_start, m.char_len))
                .collect(),
        }
    }
}
