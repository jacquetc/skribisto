// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `UserDictionaryViewModel` — the per-project personal-dictionary feature's
//! business logic, shared by the Settings pane and the editor's "Add to
//! dictionary" context-menu action.
//!
//! It owns no state of its own beyond the Layer-A handles it composes: the
//! reactive [`DictWordListModel`] (the words
//! list + its collection writes), a [`SingleDictWord`]
//! (the inline rename), and [`AppIds`] (the owner
//! `Work` plus the undo stack every mutation needs). Dedup is exact-case
//! (`"the"` and `"The"`
//! are distinct — spell-check matching is exact-case); every add is one undoable
//! step (`create_dict_word_multi`) so a multi-word add / import reverts atomically.
//!
//! Plain Rust, no `#[cfg]`: the real/mock seam lives in the model + single below
//! it, so this is unit-testable headless and identical in both builds.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::widgets::{Toast, ToastAction};

use crate::app_ids::{AppIds, HasWorkId};
use crate::models::{DictWordListModel, DictWordRow};
use crate::singles::SingleDictWord;
use crate::toast_scope::ToastWorkExt;

/// Refuse an import larger than this — a `.txt` this big is far more likely a
/// wrong file (a whole hunspell dictionary, a log) than a hand-curated word list,
/// and importing it would balloon the undo stack and the `.skrib` bundle.
const MAX_IMPORT_LINES: usize = 5_000;

/// What an import did, for the confirmation toast.
pub struct ImportSummary {
    /// Words actually created.
    pub added: usize,
    /// Valid words skipped because they were already present (or repeated).
    pub duplicates: usize,
    /// Blank lines skipped.
    pub blank: usize,
}

#[derive(Clone)]
pub struct UserDictionaryViewModel {
    list: DictWordListModel,
    single: SingleDictWord,
    ids: AppIds,
}

impl UserDictionaryViewModel {
    pub fn new(list: DictWordListModel, single: SingleDictWord, ids: AppIds) -> Self {
        Self { list, single, ids }
    }

    /// Wire the held Layer-A handles' event subscriptions (once, from `App::build`).
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.list.wire(ctx);
        self.single.wire(ctx);
    }

    /// The reactive list to bind (the pane wraps it in a `SortFilterListModel`).
    pub fn list_model(&self) -> ListModel<DictWordRow> {
        self.list.list_model()
    }

    /// Bumped on every change — the pane's empty-state binds this.
    pub fn changed_signal(&self) -> Signal<u64> {
        self.list.version_signal()
    }

    /// Current words, sorted (for export / tests).
    pub fn words(&self) -> Vec<DictWordRow> {
        self.list.rows()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Exact-case membership of the trimmed word.
    pub fn contains(&self, word: &str) -> bool {
        self.list.contains(word.trim())
    }

    /// Whether `word` is a valid, non-duplicate addition (drives the Add button's
    /// enabled state / validation).
    pub fn can_add(&self, word: &str) -> bool {
        let w = word.trim();
        !w.is_empty() && is_wordlike(w) && !self.list.contains(w)
    }

    /// Add one word. Returns the created id(s) (empty when blank / duplicate / no
    /// project). A thin wrapper over [`add_words`](Self::add_words).
    pub fn add_word(&self, word: &str) -> Vec<u64> {
        self.add_words(std::slice::from_ref(&word.to_string()))
    }

    /// Add words: trim, drop blank / non-word-like, dedup case-exactly (within the
    /// batch **and** against the existing set), then create the survivors in one
    /// undoable step. Returns the created ids (for the toast's Undo).
    pub fn add_words(&self, words: &[String]) -> Vec<u64> {
        let fresh = self.dedup_new(words);
        if fresh.is_empty() {
            return Vec::new();
        }
        self.list
            .add_words(&fresh, self.ids.work_id.get(), self.ids.stack_id.get())
    }

    /// Remove one word (its own undo step).
    pub fn remove(&self, id: u64) {
        self.list.remove_all(&[id], self.ids.stack_id.get());
    }

    /// Remove several (the toast Undo — one undo step, missing ids ignored).
    pub fn remove_all(&self, ids: &[u64]) {
        self.list.remove_all(ids, self.ids.stack_id.get());
    }

    /// Rename a word in place. No-op on blank / non-word-like / unchanged, and
    /// rejected if it would duplicate another entry (exact-case). Undoable.
    pub fn rename(&self, id: u64, new_word: &str) -> Result<()> {
        let w = new_word.trim();
        if w.is_empty() || !is_wordlike(w) {
            return Ok(());
        }
        // Reject a rename onto a word that already exists elsewhere. Renaming a
        // word to itself hits `contains` too, but `SingleDictWord::rename` no-ops
        // on an unchanged value — so this only ever blocks a genuine duplicate.
        if self.list.contains(w) {
            return Ok(());
        }
        self.single.set_id(Some(id));
        self.single.rename(w, self.ids.stack_id.get())
    }

    /// **Merge** a plain-text word list (one word per line). Reads lossily (a
    /// stray Latin-1 export doesn't hard-fail), trims, drops blanks, keeps only
    /// word-like tokens, dedups case-exactly against the existing set, and adds
    /// the survivors in one undo step. Reports counts for the toast.
    pub fn import_from(&self, path: &Path) -> Result<ImportSummary> {
        let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        let text = String::from_utf8_lossy(&bytes);

        let mut blank = 0usize;
        let mut valid: Vec<String> = Vec::new();
        for (n, line) in text.lines().enumerate() {
            if n >= MAX_IMPORT_LINES {
                anyhow::bail!(
                    "the file has more than {MAX_IMPORT_LINES} lines — that doesn't look like a \
                     personal word list"
                );
            }
            let t = line.trim();
            if t.is_empty() {
                blank += 1;
            } else if is_wordlike(t) {
                valid.push(t.to_string());
            }
        }

        let fresh = self.dedup_new(&valid);
        let added = fresh.len();
        let duplicates = valid.len().saturating_sub(added);
        if !fresh.is_empty() {
            self.list
                .add_words(&fresh, self.ids.work_id.get(), self.ids.stack_id.get());
        }
        Ok(ImportSummary {
            added,
            duplicates,
            blank,
        })
    }

    /// Write the words to a plain-text file, one per line (sorted, unique),
    /// UTF-8 with a trailing newline. Returns the count written.
    pub fn export_to(&self, path: &Path) -> Result<usize> {
        let words: Vec<String> = self.list.rows().into_iter().map(|r| r.word).collect();
        std::fs::write(path, format_txt(&words))
            .with_context(|| format!("writing {}", path.display()))?;
        Ok(words.len())
    }

    /// Show the "added" toast with an Undo that removes exactly the created ids.
    /// A no-op when nothing was created (all duplicates). Replaces a prior
    /// "dict.added" toast rather than stacking, since adds come in bursts.
    pub fn added_toast(&self, ctx: &mut EventContext, ids: Vec<u64>, sample: Option<String>) {
        if ids.is_empty() {
            return;
        }
        let message = match (ids.len(), sample) {
            (1, Some(word)) => tr!(editor_dict_added(word = word)),
            _ => tr!(editor_dict_added_multi(count = ids.len() as i64)),
        };
        let me = self.clone();
        let work_id = self.ids.work_id.get();
        ctx.show_toast(
            Toast::info(message)
                // Work-scoped (F1): a bare "dict.added" shared by every window
                // would let a second Work's own add-burst find THIS Work's
                // still-live toast and silently steal/retarget it (and its
                // Undo action along with it).
                .scoped_id("dict.added", work_id)
                // Work-scoped: this project's own personal dictionary, not
                // every open window's.
                .target_work(work_id)
                .action(ToastAction::primary(tr!(toast_undo()), move |_c| {
                    me.remove_all(&ids)
                })),
        );
    }

    /// Trim → drop blank / non-word-like → dedup exact-case within the batch and
    /// against the existing set.
    fn dedup_new(&self, words: &[String]) -> Vec<String> {
        let mut seen: HashSet<String> = HashSet::new();
        let mut out = Vec::new();
        for w in words {
            let t = w.trim();
            if t.is_empty() || !is_wordlike(t) {
                continue;
            }
            if !seen.insert(t.to_string()) {
                continue; // repeated within this batch
            }
            if self.list.contains(t) {
                continue; // already in the dictionary
            }
            out.push(t.to_string());
        }
        out
    }
}

/// The open Work this word list belongs to — every toast this view-model
/// (and its settings pane) raises routes here (via [`HasWorkId::work_id`],
/// see `crate::toast_scope::ToastWorkExt`) rather than broadcasting a
/// per-project dictionary edit into every open window.
impl HasWorkId for UserDictionaryViewModel {
    fn app_ids(&self) -> &AppIds {
        &self.ids
    }
}

/// A token is addable only if it holds at least one alphabetic character — the
/// same guard `SpellChecker::misspelled` uses, so nothing unaddable can ever be
/// flagged (and pure numbers / punctuation never enter the dictionary).
fn is_wordlike(word: &str) -> bool {
    word.chars().any(|c| c.is_alphabetic())
}

/// Format words as a `.txt` word list: one per line, `\n`, trailing newline.
pub(crate) fn format_txt(words: &[String]) -> String {
    let mut out = String::new();
    for w in words {
        out.push_str(w);
        out.push('\n');
    }
    out
}

/// Parse a `.txt` word list: split lines, trim, drop blanks, keep order. (The
/// pure half of [`UserDictionaryViewModel::import_from`], for tests.)
#[cfg(test)]
pub(crate) fn parse_txt(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_txt_is_one_word_per_line_with_trailing_newline() {
        let out = format_txt(&["alpha".to_string(), "beta".to_string()]);
        assert_eq!(out, "alpha\nbeta\n");
    }

    #[test]
    fn parse_txt_trims_and_drops_blank_lines_keeping_order() {
        let got = parse_txt("  alpha \n\n\tbeta\n   \ngamma\r\n");
        assert_eq!(got, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn parse_format_round_trip_survives() {
        let words = vec![
            "Gandalf".to_string(),
            "café".to_string(),
            "don't".to_string(),
        ];
        let round = parse_txt(&format_txt(&words));
        assert_eq!(round, words);
    }

    #[test]
    fn is_wordlike_rejects_numbers_and_punctuation() {
        assert!(is_wordlike("Gandalf"));
        assert!(is_wordlike("don't"));
        assert!(!is_wordlike("123"));
        assert!(!is_wordlike("!!!"));
        assert!(!is_wordlike("   "));
    }
}
