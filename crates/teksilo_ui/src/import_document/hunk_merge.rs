// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Taking *some* of what an editor sent back.
//!
//! # The gap this closes
//!
//! [`RowAction`](skribisto_model::reconcile::RowAction) is whole-row:
//! `TakeImport` replaces a scene's prose entirely, `KeepCurrent` keeps it
//! entirely. So an editor who fixed thirty commas across a chapter presented the
//! writer with one choice — take all thirty and any rewriting that came with
//! them, or take none and retype them by hand. That is the decision surface the
//! whole returning-file feature funnels into, and it was the coarsest possible
//! one.
//!
//! A hunk is one block that differs. Accepting a hunk takes the returning file's
//! version of that block; rejecting it keeps the project's. Everything else in
//! the row is untouched either way.
//!
//! # Why this does not reuse `versions::version_diff`
//!
//! It looks like it should: that module already pairs blocks, already produces
//! interleaved word runs, and already renders into `widgets::diff_pane`. It is
//! the right thing to *show*, and the wizard should keep showing it.
//!
//! It is the wrong thing to *apply*, for one reason that is easy to miss:
//! `version_diff::blocks` runs the Djot through
//! [`djot_to_plain_text`](skrib_format::djot_plain_text) before comparing, so
//! every block it hands back is plain text. Rebuilding a row from those blocks
//! would write back prose stripped of its emphasis, its links, its scene breaks
//! and its `[^footnote]` references — silently, and only in the parts the writer
//! accepted. A diff for reading may flatten markup; a diff for writing may not.
//!
//! So the blocks here are **exact slices of the Djot source**, and reassembly
//! concatenates the original bytes, separators included. That is what makes the
//! central guarantee true by construction rather than by care:
//!
//! > **Accepting nothing returns the project's prose byte for byte.**
//!
//! It is asserted as a property test rather than left as a comment, because the
//! whole footnote/image/anchor-preservation argument rests on it and it would
//! evaporate silently under a refactor.

use std::collections::BTreeSet;

/// One block of Djot, as it appears in its source, with the separator that
/// followed it.
///
/// Keeping the separator is what makes reassembly exact: a document ending
/// without a trailing newline, or one using three blank lines between scenes,
/// comes back exactly as it went in.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceBlock {
    text: String,
    sep: String,
}

/// Split Djot into block-level slices without parsing it.
///
/// A block boundary is a run of one or more blank lines. Deliberately textual:
/// this function's contract is that concatenating every `text` and `sep` in order
/// reproduces the input exactly, and a parse-and-reserialise round trip cannot
/// promise that (it normalises).
fn split_blocks(src: &str) -> Vec<SourceBlock> {
    let mut out: Vec<SourceBlock> = Vec::new();
    let mut block = String::new();
    let mut sep = String::new();

    for line in src.split_inclusive('\n') {
        let is_blank = line.trim().is_empty();
        if is_blank {
            // A blank line closes whatever block is open; further blank lines
            // join the same separator.
            if block.is_empty() {
                if let Some(last) = out.last_mut() {
                    last.sep.push_str(line);
                } else {
                    sep.push_str(line);
                }
            } else {
                out.push(SourceBlock {
                    text: std::mem::take(&mut block),
                    sep: line.to_string(),
                });
            }
        } else {
            if !sep.is_empty() && out.is_empty() && block.is_empty() {
                // Leading blank lines belong to the first block, so they are not
                // lost when nothing precedes them.
                block.push_str(&std::mem::take(&mut sep));
            }
            block.push_str(line);
        }
    }
    if !block.is_empty() {
        out.push(SourceBlock {
            text: block,
            sep: String::new(),
        });
    } else if !sep.is_empty() {
        // A document that is only blank lines still has to round-trip.
        match out.last_mut() {
            Some(last) => last.sep.push_str(&sep),
            None => out.push(SourceBlock {
                text: String::new(),
                sep,
            }),
        }
    }
    out
}

/// What one hunk proposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkKind {
    /// The returning file has a block the project does not.
    Added,
    /// The project has a block the returning file does not.
    Removed,
    /// Both have a block in the same place, saying different things.
    Changed,
}

/// One block-level difference the writer can take or leave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// Stable within one `local`/`incoming` pair: the writer's decisions are
    /// keyed on it, and it must not shift when a neighbouring hunk is accepted.
    pub index: usize,
    pub kind: HunkKind,
    /// What the project has here. Empty for [`HunkKind::Added`].
    pub local: String,
    /// What the returning file has here. Empty for [`HunkKind::Removed`].
    pub incoming: String,
}

/// One step of the alignment, in document order.
#[derive(Debug, Clone)]
enum Step {
    /// Both sides agree; carries the local block so reassembly is exact.
    Same(SourceBlock),
    Hunk {
        index: usize,
        kind: HunkKind,
        local: Option<SourceBlock>,
        incoming: Option<SourceBlock>,
    },
}

/// Align the two block sequences.
///
/// A longest-common-subsequence over block text, then a pass that pairs a
/// removal immediately followed by an addition into a single `Changed` hunk —
/// which is what an edited paragraph is, and presenting it as "delete this, add
/// that" would make the commonest case the hardest to read.
fn align(local: &[SourceBlock], incoming: &[SourceBlock]) -> Vec<Step> {
    let keep = lcs(local, incoming);

    let mut steps: Vec<Step> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    let mut next_index = 0usize;
    let push_hunk = |steps: &mut Vec<Step>,
                     next_index: &mut usize,
                     kind,
                     local: Option<SourceBlock>,
                     incoming: Option<SourceBlock>| {
        steps.push(Step::Hunk {
            index: *next_index,
            kind,
            local,
            incoming,
        });
        *next_index += 1;
    };

    for (li, ri) in &keep {
        while i < *li {
            push_hunk(
                &mut steps,
                &mut next_index,
                HunkKind::Removed,
                Some(local[i].clone()),
                None,
            );
            i += 1;
        }
        while j < *ri {
            push_hunk(
                &mut steps,
                &mut next_index,
                HunkKind::Added,
                None,
                Some(incoming[j].clone()),
            );
            j += 1;
        }
        steps.push(Step::Same(local[*li].clone()));
        i += 1;
        j += 1;
    }
    while i < local.len() {
        push_hunk(
            &mut steps,
            &mut next_index,
            HunkKind::Removed,
            Some(local[i].clone()),
            None,
        );
        i += 1;
    }
    while j < incoming.len() {
        push_hunk(
            &mut steps,
            &mut next_index,
            HunkKind::Added,
            None,
            Some(incoming[j].clone()),
        );
        j += 1;
    }

    coalesce(steps)
}

/// Fold `Removed` immediately followed by `Added` into one `Changed`.
fn coalesce(steps: Vec<Step>) -> Vec<Step> {
    let mut out: Vec<Step> = Vec::with_capacity(steps.len());
    let mut it = steps.into_iter().peekable();
    while let Some(step) = it.next() {
        match step {
            Step::Hunk {
                kind: HunkKind::Removed,
                local,
                ..
            } if matches!(
                it.peek(),
                Some(Step::Hunk {
                    kind: HunkKind::Added,
                    ..
                })
            ) =>
            {
                let Some(Step::Hunk { incoming, .. }) = it.next() else {
                    unreachable!("peeked an Added hunk")
                };
                out.push(Step::Hunk {
                    index: 0, // renumbered below
                    kind: HunkKind::Changed,
                    local,
                    incoming,
                });
            }
            other => out.push(other),
        }
    }
    // Renumber so indices stay dense and in document order after coalescing —
    // the writer's decisions are keyed on them.
    let mut n = 0usize;
    for step in &mut out {
        if let Step::Hunk { index, .. } = step {
            *index = n;
            n += 1;
        }
    }
    out
}

/// Classic LCS over block text, returning the paired `(local, incoming)` indices.
///
/// Quadratic, and deliberately: a scene is tens of blocks, not thousands, and the
/// clarity of the table is worth more here than an asymptotic win nobody can
/// measure. `reconcile` makes the same call for the same reason.
fn lcs(a: &[SourceBlock], b: &[SourceBlock]) -> Vec<(usize, usize)> {
    let (n, m) = (a.len(), b.len());
    let mut table = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            table[i][j] = if a[i].text.trim_end() == b[j].text.trim_end() {
                table[i + 1][j + 1] + 1
            } else {
                table[i + 1][j].max(table[i][j + 1])
            };
        }
    }
    let mut out = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < n && j < m {
        if a[i].text.trim_end() == b[j].text.trim_end() {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// Every block-level difference between the project's prose and the returning
/// file's, in document order.
pub fn hunks(local: &str, incoming: &str) -> Vec<Hunk> {
    align(&split_blocks(local), &split_blocks(incoming))
        .into_iter()
        .filter_map(|s| match s {
            Step::Same(_) => None,
            Step::Hunk {
                index,
                kind,
                local,
                incoming,
            } => Some(Hunk {
                index,
                kind,
                local: local.map(|b| b.text).unwrap_or_default(),
                incoming: incoming.map(|b| b.text).unwrap_or_default(),
            }),
        })
        .collect()
}

/// Rebuild the row's prose, taking the hunks in `accepted` and leaving the rest.
///
/// Indices are [`Hunk::index`] values from the same `local`/`incoming` pair.
/// Unknown indices are ignored rather than an error: a decision left over from a
/// merge the writer re-sourced is stale, not wrong, and refusing the whole apply
/// over one would lose every other decision with it.
pub fn apply(local: &str, incoming: &str, accepted: &BTreeSet<usize>) -> String {
    let steps = align(&split_blocks(local), &split_blocks(incoming));

    // Collected first, joined second. The separator a block carries is the one
    // that followed it *in its own document*, and the last block of any document
    // carries none — so a block appended after it would weld onto it. Deciding
    // separators once, with the whole sequence in hand, is what makes both the
    // append case and the byte-for-byte guarantee fall out of the same rule.
    let mut pieces: Vec<SourceBlock> = Vec::new();
    for step in steps {
        match step {
            Step::Same(b) => pieces.push(b),
            Step::Hunk {
                index,
                kind,
                local,
                incoming,
            } => {
                let take = accepted.contains(&index);
                let chosen = match (kind, take) {
                    // Take the file's block, or leave the row without it.
                    (HunkKind::Added, true) => incoming,
                    (HunkKind::Added, false) => None,
                    // Accepting a removal means letting it go.
                    (HunkKind::Removed, true) => None,
                    (HunkKind::Removed, false) => local,
                    (HunkKind::Changed, true) => incoming,
                    (HunkKind::Changed, false) => local,
                };
                if let Some(b) = chosen {
                    pieces.push(b);
                }
            }
        }
    }

    let mut out = String::with_capacity(local.len().max(incoming.len()));
    let last = pieces.len().saturating_sub(1);
    for (n, piece) in pieces.iter().enumerate() {
        out.push_str(&piece.text);
        if n == last {
            // The final block keeps whatever it had, including nothing: a
            // document that ended without a trailing newline still does.
            out.push_str(&piece.sep);
        } else if piece.sep.is_empty() {
            // It was last in its own document (so it kept no separator) but is
            // not last here. A block's `text` already carries its own trailing
            // newline; what a separator adds is the *blank* line, so supplying
            // two would open a gap the writer never typed.
            out.push_str(if piece.text.ends_with('\n') {
                "\n"
            } else {
                "\n\n"
            });
        } else {
            out.push_str(&piece.sep);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(indices: &[usize]) -> BTreeSet<usize> {
        indices.iter().copied().collect()
    }

    #[test]
    fn splitting_and_rejoining_is_exact() {
        for src in [
            "One paragraph.\n",
            "One.\n\nTwo.\n",
            "One.\n\n\n\nTwo, after three blank lines.\n",
            "No trailing newline",
            "\n\nLeading blank lines.\n",
            "",
            "Trailing blanks.\n\n\n",
        ] {
            let rebuilt: String = split_blocks(src)
                .iter()
                .map(|b| format!("{}{}", b.text, b.sep))
                .collect();
            assert_eq!(rebuilt, src, "round trip failed for {src:?}");
        }
    }

    /// **The guarantee the whole feature rests on.** Accepting nothing must
    /// return the project's prose byte for byte — otherwise "reject this hunk"
    /// silently rewrites the parts the writer did not touch, which is the exact
    /// failure hunk-level merge exists to prevent.
    #[test]
    fn accepting_nothing_returns_the_local_prose_verbatim() {
        let cases = [
            ("The ferry was late.\n", "The ferry was very late.\n"),
            ("A.\n\nB.\n\nC.\n", "A.\n\nB changed.\n\nC.\n\nD added.\n"),
            ("Only local.\n", ""),
            ("", "Only incoming.\n"),
            (
                "*Emphasis* and a [link](x) and a [^note].\n",
                "Rewritten entirely.\n",
            ),
            ("One.\n\n\n\nTwo.\n", "One.\n\nTwo.\n"),
        ];
        for (local, incoming) in cases {
            assert_eq!(
                apply(local, incoming, &BTreeSet::new()),
                local,
                "accept-none changed the prose for {local:?} / {incoming:?}"
            );
        }
    }

    /// Markup must survive an accepted neighbour. This is what reusing
    /// `version_diff` would have broken: its blocks are plain text, so accepting
    /// one hunk would have flattened every other block in the row.
    #[test]
    fn accepting_one_hunk_leaves_its_neighbours_markup_intact() {
        let local = "*Emphasis* here.\n\nPlain middle.\n\nA [^note] at the end.\n";
        let incoming = "*Emphasis* here.\n\nRewritten middle.\n\nA [^note] at the end.\n";
        let hunks = hunks(local, incoming);
        assert_eq!(hunks.len(), 1, "{hunks:#?}");
        assert_eq!(hunks[0].kind, HunkKind::Changed);

        let merged = apply(local, incoming, &set(&[0]));
        assert!(merged.contains("*Emphasis* here."), "{merged:?}");
        assert!(merged.contains("[^note]"), "{merged:?}");
        assert!(merged.contains("Rewritten middle."), "{merged:?}");
        assert!(!merged.contains("Plain middle."), "{merged:?}");
    }

    #[test]
    fn an_added_paragraph_is_its_own_hunk() {
        let local = "One.\n\nTwo.\n";
        let incoming = "One.\n\nInserted.\n\nTwo.\n";
        let hunks = hunks(local, incoming);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].kind, HunkKind::Added);
        assert_eq!(hunks[0].incoming.trim(), "Inserted.");
        assert!(hunks[0].local.is_empty());

        assert_eq!(apply(local, incoming, &set(&[0])), incoming);
        assert_eq!(apply(local, incoming, &BTreeSet::new()), local);
    }

    #[test]
    fn a_removed_paragraph_is_its_own_hunk() {
        let local = "One.\n\nDoomed.\n\nTwo.\n";
        let incoming = "One.\n\nTwo.\n";
        let hunks = hunks(local, incoming);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].kind, HunkKind::Removed);
        assert_eq!(hunks[0].local.trim(), "Doomed.");

        // Accepting a removal lets it go; rejecting keeps it.
        assert_eq!(apply(local, incoming, &set(&[0])), incoming);
        assert_eq!(apply(local, incoming, &BTreeSet::new()), local);
    }

    /// An edited paragraph is one decision, not two. Presenting it as a delete
    /// beside an add would make the commonest case the hardest to read.
    #[test]
    fn an_edited_paragraph_is_one_changed_hunk_not_a_delete_and_an_add() {
        let hunks = hunks("The ferry was late.\n", "The ferry was very late.\n");
        assert_eq!(hunks.len(), 1, "{hunks:#?}");
        assert_eq!(hunks[0].kind, HunkKind::Changed);
        assert_eq!(hunks[0].local.trim(), "The ferry was late.");
        assert_eq!(hunks[0].incoming.trim(), "The ferry was very late.");
    }

    /// Taking every hunk is the whole-row `TakeImport` the wizard already
    /// offered, so the two must agree — otherwise the granular path and the
    /// coarse one would disagree about the same file.
    #[test]
    fn accepting_every_hunk_equals_taking_the_import() {
        for (local, incoming) in [
            ("A.\n\nB.\n\nC.\n", "A.\n\nB changed.\n\nC.\n\nD.\n"),
            ("Gone.\n\nKept.\n", "Kept.\n"),
            ("One.\n", "One.\n\nTwo.\n\nThree.\n"),
        ] {
            let all: BTreeSet<usize> = hunks(local, incoming).iter().map(|h| h.index).collect();
            assert_eq!(
                apply(local, incoming, &all),
                incoming,
                "taking every hunk did not reproduce the file for {local:?}"
            );
        }
    }

    #[test]
    fn indices_are_dense_and_in_document_order() {
        let local = "A.\n\nB.\n\nC.\n\nD.\n";
        let incoming = "A changed.\n\nB.\n\nInserted.\n\nD.\n";
        let hunks = hunks(local, incoming);
        assert!(hunks.len() >= 2, "{hunks:#?}");
        for (n, h) in hunks.iter().enumerate() {
            assert_eq!(h.index, n, "indices must be dense: {hunks:#?}");
        }
    }

    /// A stale decision from a merge the writer re-sourced must not lose every
    /// other decision with it.
    #[test]
    fn an_unknown_index_is_ignored_rather_than_fatal() {
        let local = "One.\n\nTwo.\n";
        let incoming = "One.\n\nTwo changed.\n";
        assert_eq!(apply(local, incoming, &set(&[0, 99])), incoming);
    }

    #[test]
    fn two_identical_documents_have_no_hunks() {
        let same = "One.\n\nTwo.\n";
        assert!(hunks(same, same).is_empty());
        assert_eq!(apply(same, same, &BTreeSet::new()), same);
    }
}
