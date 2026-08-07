// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which number each footnote prints — "this is note 12".
//!
//! The sibling of [`crate::numbering`], and it exists for the same reason and takes
//! the same shape: **the number is a fact about the manuscript, not about the
//! selection or the view.** Export chapter five on its own and its notes must be
//! numbered as the whole book numbers them, because that is the number the writer
//! sees in the editor and the number a reader would find in the finished book. A
//! per-export counter would have said 1, 2, 3 — and would have disagreed with the
//! badge in the editor at the same moment, for the same note.
//!
//! So this is one pass over the **whole** ordered stream, before anyone filters
//! anything, and both the exporter and the editor's live marker look their answer
//! up by `(item, label)`. There is no second pass to drift.
//!
//! Store-free and IO-free, like its siblings. It takes the same flat
//! [`ItemMeta`](crate::compile::ItemMeta)
//! slice its neighbours do, plus each row's prose — because where a note sits in
//! the book is decided by where its **reference** sits in the text, not by anything
//! stored on the note.
//!
//! # Finding the references
//!
//! By plain substring search for `[^label]`, deliberately, rather than by parsing
//! the Djot. Two reasons:
//!
//! * It keeps this module in the store-free, dependency-free family its siblings
//!   are in — no document model, no parser, no allocation per scene beyond the
//!   answer.
//! * The label is **minted by Skribisto**, never typed by the writer, so there is no
//!   ambiguity to resolve. `[^` and `]` bracket it on both sides, so `[^1]` cannot
//!   match inside `[^10]` — the trailing bracket is what makes the naive search
//!   exact.
//!
//! A plain substring search is still not a *blind* one. Djot itself refuses to
//! read `[^label]`-shaped bytes as a reference in two cases, and a search that
//! ignored them would seize a manuscript position — and hand out a number, and a
//! dock navigation target — for text that was never a citation:
//!
//! * **Inside a code span or fenced code block** (`` `[^fn2]` ``, or the same
//!   inside a ` ``` ` fence): a writer showing the *syntax* as an example, not
//!   using it. `document_io`'s real Djot parser never creates a reference object
//!   there; this module has to know the same rule, or an aside in a "Style
//!   notes to self" document can steal a citation's place.
//! * **Right after a backslash escape** (`\[^fn2]`): Djot's own escape for
//!   literal punctuation, asking for the bracket rather than the syntax it would
//!   otherwise start.
//!
//! Recognising both stops short of a real parse — `verbatim_ranges` reads just
//! enough of Djot's own delimiter grammar (backtick runs, fence lines) to find
//! the byte ranges a parser would treat as inert, which keeps this in the
//! dependency-free family the rest of the module is.
//!
//! # One collapse, shared
//!
//! [`number_map`](crate::footnote_numbering::number_map) is keyed by `(item_id, label)`, which is right for the count
//! but wrong for a *marker*: a document draws one glyph per `[^label]`, and
//! `bastyde::text_document::TextDocument::set_footnote_markers` (the call both
//! the editor and every exporter push their answer through) takes exactly one
//! marker **per label**, with no way to say "citation from item 150 reads
//! differently than citation from item 100." Something has to collapse the
//! per-item entries down to the one a label actually prints — and if the editor's
//! live badge and an exporter each did that collapse themselves, a citation
//! could legitimately draw two different numbers depending only on which one you
//! looked at. [`label_homes`](crate::footnote_numbering::label_homes) is that collapse, done once, so both callers share
//! it and cannot drift apart. See its own doc for the rule and why it must run
//! over the whole manuscript, never a scope.

use std::collections::HashMap;

use crate::SubRoleExt;
use crate::compile::ItemMeta;

/// Where a note's number restarts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FootnoteRestart {
    /// One run of numbers through the whole book. Common in non-fiction, where a
    /// reader may cite "note 214" and expect it to be findable.
    #[default]
    Continuous,
    /// Numbers restart at each chapter — common in fiction, and what keeps a long
    /// novel's markers from reaching four digits.
    PerChapter,
    /// Restart at each book, for a manuscript holding several.
    PerBook,
}

/// One footnote's place in the manuscript.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberedNote {
    /// What the marker prints.
    pub number: usize,
    /// Position in the whole manuscript, ignoring restarts — the order an endnote
    /// list is written in, and a stable sort key that survives a change of restart
    /// rule.
    pub ordinal: usize,
}

/// Which rows carry notes that count.
///
/// The same three filters [`crate::numbering`] applies to chapters, and for the same
/// reasons: a trashed row is not in the book, a non-exportable row is not in the
/// book, and both close the gap behind them rather than leaving a hole in the
/// sequence.
///
/// `exclude_from_numbering` is deliberately **not** among them. It means "this row's
/// own heading takes no chapter number" — a prologue — and says nothing about
/// whether the prose inside it may carry a note. A footnote in a prologue is still
/// a footnote in the book.
fn counts(item: &ItemMeta) -> bool {
    item.activated && item.is_exportable
}

/// Every reference in `prose`, in the order they are read, as `(byte_offset, label)`.
///
/// Public because the same walk answers "which notes does this scene reference, and
/// in what order" for the editor's dock, which needs it without wanting numbers.
///
/// Skips a `[^label]`-shaped run of bytes sitting inside a code span/fenced code
/// block, or right after a backslash escape — see the module docs' "Finding the
/// references" section for why: both are places Djot's own parser would read the
/// bytes as inert text, never as a citation.
pub fn references_in(prose: &str, labels: &[String]) -> Vec<(usize, String)> {
    let verbatim = verbatim_ranges(prose);
    let mut found: Vec<(usize, String)> = Vec::new();
    for label in labels {
        let needle = format!("[^{label}]");
        let mut from = 0usize;
        while let Some(rel) = prose[from..].find(&needle) {
            let at = from + rel;
            from = at + needle.len();
            if in_verbatim(&verbatim, at) || preceded_by_odd_backslashes(prose, at) {
                continue;
            }
            found.push((at, label.clone()));
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    found
}

/// Byte `at` is escaped: an odd run of backslashes sits immediately before it.
///
/// Djot's rule is that a backslash escapes the one ASCII punctuation mark right
/// after it, and a backslash escapes a backslash — so `\[^fn1]` is a literal
/// bracket (one backslash, odd) but `\\[^fn1]` is an escaped backslash followed
/// by a real reference (two backslashes, even). Counting the run, rather than
/// just checking the one byte before, is what tells those two apart.
fn preceded_by_odd_backslashes(prose: &str, at: usize) -> bool {
    let bytes = prose.as_bytes();
    let mut count = 0usize;
    let mut i = at;
    while i > 0 && bytes[i - 1] == b'\\' {
        count += 1;
        i -= 1;
    }
    count % 2 == 1
}

/// Whether byte offset `at` falls inside one of `ranges`.
fn in_verbatim(ranges: &[(usize, usize)], at: usize) -> bool {
    ranges.iter().any(|&(start, end)| at >= start && at < end)
}

/// Byte ranges of `prose` that Djot's own grammar takes verbatim — inline code
/// spans and fenced code blocks — where a `[^label]`-shaped run of bytes is
/// exactly what it looks like typed out, never a reference.
///
/// Deliberately not a full parse (see the module docs): just the two delimiter
/// rules that matter for telling an example from a citation.
fn verbatim_ranges(prose: &str) -> Vec<(usize, usize)> {
    let fenced = fenced_block_ranges(prose);
    let mut ranges = fenced.clone();
    ranges.extend(inline_code_ranges(prose, &fenced));
    ranges.sort_unstable_by_key(|r| r.0);
    ranges
}

/// Fenced code blocks: a line of three or more backticks or tildes (at most 3
/// spaces of indent) opens one, closed by a line of the same character, at
/// least as long, with nothing else on it — Djot's fence rule, shared with
/// CommonMark. Everything from the opening fence's line to the closing fence's
/// line (inclusive) is verbatim; an unclosed fence runs to the end of the text,
/// exactly as Djot renders it.
fn fenced_block_ranges(prose: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut open: Option<(u8, usize, usize)> = None; // (fence_byte, fence_len, block_start)
    let mut offset = 0usize;
    for line in prose.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        let trimmed = content.trim_start_matches(' ');
        let indent = content.len() - trimmed.len();
        let fence_char = trimmed.as_bytes().first().copied();
        let run = trimmed
            .bytes()
            .take_while(|&b| Some(b) == fence_char)
            .count();
        if let Some((open_char, open_len, start)) = open {
            let closes = indent <= 3
                && fence_char == Some(open_char)
                && run >= open_len
                && trimmed[run..].trim().is_empty();
            if closes {
                ranges.push((start, offset + line.len()));
                open = None;
            }
        } else if let Some(ch @ (b'`' | b'~')) = fence_char
            && indent <= 3
            && run >= 3
            // A backtick fence's info string may not itself contain a backtick —
            // the rule that tells a fence line from an ordinary paragraph that
            // happens to start with backticks.
            && (ch != b'`' || !trimmed[run..].contains('`'))
        {
            open = Some((ch, run, offset));
        }
        offset += line.len();
    }
    if let Some((_, _, start)) = open {
        ranges.push((start, prose.len()));
    }
    ranges
}

/// Inline code spans: a run of *N* backticks opens one, closed by the next run
/// of *exactly* N backticks before the next paragraph break — a code span, like
/// CommonMark's, cannot cross one. `skip` is already-fenced ranges: scanning
/// inside one risks reading a fence's own backtick run as a span delimiter and
/// spilling past the block it closes, so those bytes are walked over rather
/// than scanned.
fn inline_code_ranges(prose: &str, skip: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let bytes = prose.as_bytes();
    let mut ranges = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if let Some(&(_, end)) = skip.iter().find(|&&(s, e)| i >= s && i < e) {
            i = end;
            continue;
        }
        if bytes[i] != b'`' {
            i += 1;
            continue;
        }
        let run_start = i;
        let mut j = i;
        while j < bytes.len() && bytes[j] == b'`' {
            j += 1;
        }
        let run_len = j - run_start;
        let boundary = prose[j..]
            .find("\n\n")
            .map(|p| j + p)
            .unwrap_or(bytes.len());
        let mut k = j;
        let mut closed = None;
        while k < boundary {
            if bytes[k] == b'`' {
                let close_start = k;
                let mut m = k;
                while m < boundary && bytes[m] == b'`' {
                    m += 1;
                }
                if m - close_start == run_len {
                    closed = Some(m);
                    break;
                }
                k = m;
            } else {
                k += 1;
            }
        }
        match closed {
            Some(end) => {
                ranges.push((run_start, end));
                i = end;
            }
            // No matching close: these backticks are literal, not a span.
            None => i = j,
        }
    }
    ranges
}

/// Number every footnote in the manuscript.
///
/// `prose_of` yields each item's prose rows — a scene has one, but a row can carry
/// several (a synopsis, a note body), and a reference in any of them is a reference
/// in the book. `labels` is every label the project knows, which is what lets the
/// search be exact rather than a scan for `[^…]`-shaped text.
///
/// Keyed by `(item_id, label)`: the same note referenced from two items would be two
/// markers, and the key has to tell them apart.
pub fn number_map<'a, F>(
    items: &[ItemMeta],
    labels: &[String],
    mut prose_of: F,
    restart: FootnoteRestart,
) -> HashMap<(u64, String), NumberedNote>
where
    F: FnMut(u64) -> Vec<&'a str>,
{
    let mut out: HashMap<(u64, String), NumberedNote> = HashMap::new();
    let mut counter = 0usize;
    let mut ordinal = 0usize;

    for item in items {
        // A structural row restarts the count before its own prose is walked, so a
        // note in a chapter's own opening paragraph is note 1 of that chapter.
        let restarts_here = match restart {
            FootnoteRestart::Continuous => false,
            FootnoteRestart::PerBook => item.sub_role.opens_book(),
            // A new book opens a new chapter too, so per-chapter restarts there as
            // well — otherwise book two's first chapter would continue book one's
            // run, which is the one arrangement nobody asks for.
            FootnoteRestart::PerChapter => {
                item.sub_role.opens_chapter() || item.sub_role.opens_book()
            }
        };
        if restarts_here {
            counter = 0;
        }

        if !counts(item) {
            continue;
        }
        for prose in prose_of(item.id) {
            for (_, label) in references_in(prose, labels) {
                let key = (item.id, label);
                // A label referenced twice in one row keeps one number: it is one
                // note, cited twice, exactly as it would be in print.
                if out.contains_key(&key) {
                    continue;
                }
                counter += 1;
                ordinal += 1;
                out.insert(
                    key,
                    NumberedNote {
                        number: counter,
                        ordinal,
                    },
                );
            }
        }
    }

    out
}

/// A label's one home: which `(item_id, label)` entry of [`number_map`]'s output
/// a per-label marker must be built from.
pub type LabelHomes = HashMap<String, (u64, NumberedNote)>;

/// Collapse [`number_map`]'s per-`(item, label)` entries to the ONE each label
/// actually prints — see the module docs' "One collapse, shared" section for why
/// a collapse has to happen here at all, and why both the editor's live badge and
/// every exporter must call this instead of picking their own winner.
///
/// # The rule
///
/// The item **earliest in manuscript order** wins: the entry with the smallest
/// `ordinal`. `number_map` mints ordinals while walking `items` in that exact
/// order, so the smallest-ordinal entry sharing a label is, by construction, the
/// first place in the whole manuscript that label is cited from — which is also
/// what a reader turning the pages in order would meet first, and therefore the
/// number the writer already sees in the editor's own live badge (fed by this
/// same function over the same whole-manuscript map).
///
/// # Why this must be fed the whole-manuscript map
///
/// A caller that built `numbered` from only a *subset* of items — a scoped
/// export's own included rows, say — could see a different, later entry as the
/// smallest-ordinal one, purely because the true first citation fell outside
/// that subset. That is precisely the bug this function exists to close: pass it
/// the map [`number_map`] built over the **whole** tree (as its own doc directs),
/// never a scope-limited one, and every caller collapsing the same manuscript
/// state lands on the same winner.
///
/// A label absent from `numbered` (referenced only by trashed/non-exportable
/// rows, or not referenced by any eligible row at all) has no home here — there
/// is no eligible entry to pick from, and a caller needing to place it anyway
/// (the editor's dock still shows an unnumbered note) has to fall back to its own
/// wider walk; see `bastyde_ui`'s `places()`.
pub fn label_homes(numbered: &HashMap<(u64, String), NumberedNote>) -> LabelHomes {
    let mut homes: LabelHomes = HashMap::new();
    for ((item_id, label), note) in numbered {
        homes
            .entry(label.clone())
            .and_modify(|(home_item, home_note)| {
                if note.ordinal < home_note.ordinal {
                    *home_item = *item_id;
                    *home_note = *note;
                }
            })
            .or_insert((*item_id, *note));
    }
    homes
}

/// Notes the manuscript no longer references.
///
/// A note is orphaned when nothing in the prose names it — its reference was
/// deleted along with the sentence that carried it, or its annotated row was
/// purged. The note's *words* survive (they are the writer's, and the format keeps
/// them), but nothing in the book points at them any more.
///
/// Reported rather than repaired: putting the reference back would mean guessing
/// where the sentence went, and a footnote attached to the wrong sentence is worse
/// than one the writer is told about. This is the number an export preflight shows.
pub fn orphaned_labels<'a, F>(items: &[ItemMeta], labels: &[String], mut prose_of: F) -> Vec<String>
where
    F: FnMut(u64) -> Vec<&'a str>,
{
    let mut referenced: std::collections::HashSet<String> = std::collections::HashSet::new();
    for item in items {
        // Deliberately not filtered by `counts`: a reference in a trashed or
        // non-exportable row still means the writer has not lost track of the note.
        // Calling it orphaned would send them hunting for a reference that is right
        // there, in a chapter they chose to leave out.
        for prose in prose_of(item.id) {
            for (_, label) in references_in(prose, labels) {
                referenced.insert(label);
            }
        }
    }
    let mut out: Vec<String> = labels
        .iter()
        .filter(|l| !referenced.contains(*l))
        .cloned()
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::{BinderItemRole, BinderItemSubRole};

    fn meta(id: u64, sub_role: BinderItemSubRole) -> ItemMeta {
        ItemMeta {
            id,
            role: BinderItemRole::Item,
            sub_role,
            indent: 0,
            activated: true,
            is_exportable: true,
            exclude_from_numbering: false,
        }
    }

    #[test]
    fn references_come_back_in_reading_order() {
        let labels = vec!["b".to_string(), "a".to_string()];
        let found = references_in("one[^a] two[^b] three", &labels);
        assert_eq!(
            found.iter().map(|(_, l)| l.as_str()).collect::<Vec<_>>(),
            vec!["a", "b"],
            "order is where they sit, not the order labels were given"
        );
    }

    /// The bracket on both sides is what makes the naive search exact.
    #[test]
    fn a_short_label_does_not_match_inside_a_longer_one() {
        let labels = vec!["1".to_string(), "10".to_string()];
        let found = references_in("only[^10] here", &labels);
        assert_eq!(found.len(), 1, "matched {found:?}");
        assert_eq!(found[0].1, "10");
    }

    /// A writer explaining the syntax in a Style Notes aside must not have that
    /// example steal the real citation's position and number.
    #[test]
    fn a_label_shaped_run_inside_a_code_span_is_not_a_reference() {
        let labels = vec!["fn2".to_string()];
        let found = references_in(
            "Skribisto footnotes look like `[^fn2]` in the raw file.",
            &labels,
        );
        assert!(found.is_empty(), "matched inside a code span: {found:?}");
    }

    /// The real citation right after the example is still found — the code span
    /// only shadows what is inside its own delimiters.
    #[test]
    fn a_real_reference_after_a_code_span_still_counts() {
        let labels = vec!["fn2".to_string()];
        let found = references_in("Example: `[^fn2]`. Something[^fn2] happened.", &labels);
        assert_eq!(found.len(), 1, "found {found:?}");
        assert_eq!(found[0].1, "fn2");
    }

    /// A longer run of backticks lets a literal backtick sit inside the span —
    /// the same accommodation CommonMark and Djot both make.
    #[test]
    fn a_double_backtick_span_hides_a_reference_that_contains_a_backtick() {
        let labels = vec!["fn1".to_string()];
        let found = references_in("see `` `[^fn1]` `` here", &labels);
        assert!(
            found.is_empty(),
            "matched inside a double-backtick span: {found:?}"
        );
    }

    /// A fenced code block hides an example the same way a span does.
    #[test]
    fn a_label_shaped_run_inside_a_fenced_block_is_not_a_reference() {
        let labels = vec!["fn3".to_string()];
        let prose = "Before.\n\n```\nA raw file shows [^fn3] like this.\n```\n\nAfter.";
        let found = references_in(prose, &labels);
        assert!(found.is_empty(), "matched inside a fenced block: {found:?}");
    }

    /// An unclosed backtick is just a stray character, not a span that swallows
    /// the rest of the document — a real reference after it must still be found.
    #[test]
    fn an_unmatched_backtick_does_not_hide_the_rest_of_the_prose() {
        let labels = vec!["fn1".to_string()];
        let found = references_in("a stray ` backtick, then[^fn1] a real one", &labels);
        assert_eq!(found.len(), 1, "found {found:?}");
    }

    /// `\[^fn1]` is Djot's own escape for a literal bracket — the writer asked
    /// for the punctuation, not the syntax.
    #[test]
    fn a_backslash_escaped_bracket_is_not_a_reference() {
        let labels = vec!["fn1".to_string()];
        let found = references_in(r"literally \[^fn1] here", &labels);
        assert!(found.is_empty(), "matched an escaped bracket: {found:?}");
    }

    /// `\\[^fn1]` is an escaped backslash *followed by* a real reference — the
    /// even run of backslashes must not be read as escaping the bracket too.
    #[test]
    fn an_escaped_backslash_before_a_bracket_leaves_the_reference_real() {
        let labels = vec!["fn1".to_string()];
        let found = references_in(r"a literal backslash \\[^fn1] then real", &labels);
        assert_eq!(found.len(), 1, "found {found:?}");
    }

    #[test]
    fn notes_are_numbered_in_manuscript_order() {
        let items = vec![
            meta(1, BinderItemSubRole::Scene),
            meta(2, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                1 => vec!["first[^a] and[^b]"],
                2 => vec!["second[^c]"],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        assert_eq!(map[&(1, "a".into())].number, 1);
        assert_eq!(map[&(1, "b".into())].number, 2);
        assert_eq!(map[&(2, "c".into())].number, 3);
    }

    /// **The regression class `ef2a98a0` fixed for chapters.** The number must not
    /// depend on what a given export happens to include — `number_map` never sees a
    /// selection, so exporting the second scene alone still calls its note 2.
    #[test]
    fn a_notes_number_does_not_depend_on_the_export_selection() {
        let items = vec![
            meta(1, BinderItemSubRole::Scene),
            meta(2, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                1 => vec!["first[^a]"],
                2 => vec!["second[^b]"],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        assert_eq!(
            map[&(2, "b".into())].number,
            2,
            "the second scene's note is note 2 of the book, whatever is exported"
        );
    }

    /// A trashed or excluded row's notes hold no number, and the sequence closes the
    /// gap behind them rather than leaving a hole.
    #[test]
    fn an_excluded_row_holds_no_numbers_and_leaves_no_gap() {
        let mut skipped = meta(2, BinderItemSubRole::Scene);
        skipped.is_exportable = false;
        let items = vec![
            meta(1, BinderItemSubRole::Scene),
            skipped,
            meta(3, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                1 => vec!["one[^a]"],
                2 => vec!["skipped[^b]"],
                3 => vec!["three[^c]"],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        assert!(!map.contains_key(&(2, "b".into())), "excluded row numbered");
        assert_eq!(map[&(3, "c".into())].number, 2, "the gap must close");
    }

    /// One note cited twice keeps one number.
    #[test]
    fn a_note_referenced_twice_keeps_one_number() {
        let items = vec![meta(1, BinderItemSubRole::Scene)];
        let labels = vec!["a".to_string()];
        let map = number_map(
            &items,
            &labels,
            |_| vec!["here[^a] and again[^a]"],
            FootnoteRestart::Continuous,
        );
        assert_eq!(map.len(), 1);
        assert_eq!(map[&(1, "a".into())].number, 1);
    }

    /// **The failure `label_homes` exists to close.** A duplicated scene (a raw
    /// text copy that never remints its footnote labels — see
    /// `binder_item_management::duplicate_uc.rs`) can leave the same label
    /// referenced from two different items, so `number_map` — correctly, per its
    /// own keying — mints two separate entries for it. Whoever turns that into a
    /// single per-label marker has to pick one, and `label_homes` must pick the
    /// item earliest in manuscript order: the one a reader (and the editor's own
    /// badge) meets first.
    #[test]
    fn label_homes_picks_the_earliest_manuscript_occurrence() {
        let items = vec![
            meta(100, BinderItemSubRole::Scene),
            meta(150, BinderItemSubRole::Scene),
            meta(200, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                100 => vec!["First[^a]."],
                150 => vec!["Also[^a] here."],
                200 => vec!["Second[^b]."],
                _ => vec![],
            },
            FootnoteRestart::Continuous,
        );
        // Sanity: number_map really did mint two independent entries for "a".
        assert_eq!(map[&(100, "a".into())].number, 1);
        assert_eq!(map[&(150, "a".into())].number, 2);

        let homes = label_homes(&map);
        let (home_item, home_note) = homes[&"a".to_string()];
        assert_eq!(home_item, 100, "the earlier item must win the home");
        assert_eq!(
            home_note.number, 1,
            "the marker must be the earlier item's number, not the later one's"
        );
        assert_eq!(homes[&"b".to_string()].0, 200);
        assert_eq!(homes[&"b".to_string()].1.number, 3);
    }

    /// A label with only one eligible entry has an unambiguous home — the
    /// ordinary case, unaffected by the tie-break rule.
    #[test]
    fn label_homes_is_a_no_op_when_a_label_has_one_entry() {
        let items = vec![meta(1, BinderItemSubRole::Scene)];
        let labels = vec!["a".to_string()];
        let map = number_map(
            &items,
            &labels,
            |_| vec!["only[^a] here"],
            FootnoteRestart::Continuous,
        );
        let homes = label_homes(&map);
        assert_eq!(homes[&"a".to_string()], (1, map[&(1, "a".into())]));
    }

    #[test]
    fn a_note_nothing_references_is_reported_orphaned() {
        let items = vec![meta(1, BinderItemSubRole::Scene)];
        let labels = vec!["kept".to_string(), "lost".to_string()];
        let orphans = orphaned_labels(&items, &labels, |_| vec!["still here[^kept]"]);
        assert_eq!(orphans, vec!["lost".to_string()]);
    }

    /// A reference in a chapter the writer excluded from export is still a
    /// reference. Calling that note orphaned would send them hunting for something
    /// that is exactly where they left it.
    #[test]
    fn a_reference_in_an_excluded_row_still_counts_as_referenced() {
        let mut excluded = meta(1, BinderItemSubRole::Scene);
        excluded.is_exportable = false;
        let items = vec![excluded];
        let labels = vec!["a".to_string()];
        let orphans = orphaned_labels(&items, &labels, |_| vec!["here[^a]"]);
        assert!(orphans.is_empty(), "reported {orphans:?}");
    }

    /// `ordinal` ignores restarts, so an endnote list stays in manuscript order even
    /// when the printed markers repeat.
    #[test]
    fn the_ordinal_survives_a_restart() {
        let items = vec![
            meta(1, BinderItemSubRole::Book),
            meta(2, BinderItemSubRole::Scene),
            meta(3, BinderItemSubRole::Book),
            meta(4, BinderItemSubRole::Scene),
        ];
        let labels = vec!["a".to_string(), "b".to_string()];
        let map = number_map(
            &items,
            &labels,
            |id| match id {
                2 => vec!["one[^a]"],
                4 => vec!["two[^b]"],
                _ => vec![],
            },
            FootnoteRestart::PerBook,
        );
        assert_eq!(map[&(2, "a".into())].number, 1);
        assert_eq!(map[&(4, "b".into())].number, 1, "the second book restarts");
        assert_eq!(map[&(2, "a".into())].ordinal, 1);
        assert_eq!(map[&(4, "b".into())].ordinal, 2, "the ordinal does not");
    }
}
