// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Repetition pane's tree: one row per text that echoes, its repeated words beneath.
//!
//! ## Why a tree, and why it starts closed
//!
//! An echo report is long by nature — a drafted novel produces hundreds of findings — and
//! the flat list it used to be made the writer read all of them to learn which *scenes*
//! were worth opening. The question a report should answer first is "where do I look",
//! not "what did you find"; the parent rows answer it in one screen, each carrying the
//! number of distinct words that echo inside it, and the findings stay one click away.
//!
//! Collapsed is therefore the whole point rather than a default worth overriding, and it
//! comes for free: a fresh [`TreeDataSlice`] starts with every node collapsed unless
//! `set_expand_new_nodes(true)` says otherwise — which the binder tree *does* say, for the
//! opposite reason (a freshly created scene should be visible where it landed). This model
//! deliberately does not.
//!
//! ## Why the expand set lives here and not in the widget
//!
//! `TreeView::new`/`new_with_context` build a `TreeSlice` **per view**, so expansion is the
//! widget's and dies with it. This pane is rebuilt whenever the analysis state changes —
//! and, less obviously, whenever anything above it rebuilds — which would silently close
//! every row the writer had opened. Holding the source in the view-model instead puts the
//! expand set on the `TreeDataSlice`, which outlives the widget.
//!
//! Unlike the binder tree, this one needs no second `remembered` set: it re-sources only
//! when a new analysis result arrives, never with a narrower row set mid-session, so the
//! slice's own pruning has nothing to forget.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::data::{TreeDataSlice, TreeRow};

/// A row's identity.
///
/// A word key carries its owning item, because the same word legitimately echoes in two
/// different scenes and the two rows are different findings — keyed by the word alone they
/// would collide, and the tree would show one of them twice.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RepetitionTreeKey {
    Item(u64),
    Word(u64, String),
}

/// What a row shows.
#[derive(Clone, Debug, PartialEq)]
pub enum RepetitionNode {
    /// A text that echoes: its title, and how many *distinct* words echo inside it.
    Item {
        item_id: u64,
        title: String,
        /// Distinct echoing words, not their total occurrences — the number that answers
        /// "is this row worth opening" before it is opened, which is what a collapsed tree
        /// exists to let the reader ask. A sum would read the same for "three words repeat
        /// once each" and "one word repeats three times", two quite different problems.
        words: usize,
    },
    /// One repeated word inside that text.
    Word {
        item_id: u64,
        word: String,
        /// How many uses of the word fell close enough to another use to read as an echo —
        /// **not** every appearance in the scene. Occurrences that pair with nothing inside
        /// the window are not part of the finding.
        occurrences: i64,
        /// Words between the two closest uses. The unit is words, the same unit the echo
        /// window itself is stated in, because it is the one a writer can act on.
        closest_gap: i64,
    },
}

impl RepetitionNode {
    /// The text shown as the row's label.
    pub fn label(&self) -> &str {
        match self {
            RepetitionNode::Item { title, .. } => title,
            RepetitionNode::Word { word, .. } => word,
        }
    }

    /// The binder item this row belongs to — the parent's own, or its child's owner.
    pub fn item_id(&self) -> u64 {
        match self {
            RepetitionNode::Item { item_id, .. } | RepetitionNode::Word { item_id, .. } => *item_id,
        }
    }
}

/// One text and the words that echo in it, in the order they should be shown.
pub type RepetitionGroup = (u64, String, Vec<(String, i64, i64)>);

/// Flatten grouped findings into the depth-tagged row stream a [`TreeDataSlice`] takes.
///
/// Pure, so the shape of the tree is testable without a widget: a parent at depth 0
/// followed by its words at depth 1, in the order given. The slice derives parentage from
/// the depths — "a row's parent is the nearest preceding row with a strictly smaller
/// depth" — so a group with no words would produce a childless parent, which is why the
/// caller filters those out before it gets here.
pub fn rows_of(groups: &[RepetitionGroup]) -> Vec<TreeRow<RepetitionTreeKey, RepetitionNode>> {
    let mut out = Vec::new();
    for (item_id, title, words) in groups {
        out.push(TreeRow {
            key: RepetitionTreeKey::Item(*item_id),
            item: RepetitionNode::Item {
                item_id: *item_id,
                title: title.clone(),
                words: words.len(),
            },
            depth: 0,
        });
        for (word, occurrences, closest_gap) in words {
            out.push(TreeRow {
                key: RepetitionTreeKey::Word(*item_id, word.clone()),
                item: RepetitionNode::Word {
                    item_id: *item_id,
                    word: word.clone(),
                    occurrences: *occurrences,
                    closest_gap: *closest_gap,
                },
                depth: 1,
            });
        }
    }
    out
}

/// The Repetition tree's data source.
#[derive(Clone)]
pub struct RepetitionTreeModel {
    slice: TreeDataSlice<RepetitionTreeKey, RepetitionNode>,
    /// The rows the slice re-reads on every reload. Held rather than captured by value so
    /// a new analysis result can replace them without rebuilding the slice — which would
    /// take the expand set with it.
    groups: Rc<RefCell<Vec<RepetitionGroup>>>,
}

impl Default for RepetitionTreeModel {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RepetitionTreeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepetitionTreeModel")
            .field("groups", &self.groups.borrow().len())
            .finish()
    }
}

impl RepetitionTreeModel {
    pub fn new() -> Self {
        let slice = TreeDataSlice::new();
        // Deliberately NOT `set_expand_new_nodes(true)`: every node here is new on each
        // run, so that flag would open the whole report and undo the one thing the tree is
        // for. See the module docs.
        let groups: Rc<RefCell<Vec<RepetitionGroup>>> = Rc::new(RefCell::new(Vec::new()));
        {
            let groups = groups.clone();
            slice.set_source(move || rows_of(&groups.borrow()));
        }
        Self { slice, groups }
    }

    /// Replace the findings and reload.
    ///
    /// Idempotent on identical input: a rebuild that re-pushes the same groups leaves the
    /// tree — and every row the writer expanded — exactly where it was.
    pub fn set_groups(&self, groups: Vec<RepetitionGroup>) {
        if *self.groups.borrow() == groups {
            return;
        }
        *self.groups.borrow_mut() = groups;
        self.slice.reload();
    }

    /// The source a `TreeView` binds.
    pub fn source(&self) -> TreeDataSlice<RepetitionTreeKey, RepetitionNode> {
        self.slice.clone()
    }

    /// How many findings are in the tree, across every text — what the truncation note and
    /// the empty state both ask.
    pub fn finding_count(&self) -> usize {
        self.groups.borrow().iter().map(|(_, _, w)| w.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.borrow().is_empty()
    }

    /// The node behind a key, for the activation handler — which gets a flat index and
    /// needs the item to open from it.
    pub fn node_of(&self, key: &RepetitionTreeKey) -> Option<RepetitionNode> {
        let item_id = match key {
            RepetitionTreeKey::Item(id) | RepetitionTreeKey::Word(id, _) => *id,
        };
        let groups = self.groups.borrow();
        let (id, title, words) = groups.iter().find(|(g, _, _)| *g == item_id)?;
        match key {
            RepetitionTreeKey::Item(_) => Some(RepetitionNode::Item {
                item_id: *id,
                title: title.clone(),
                words: words.len(),
            }),
            RepetitionTreeKey::Word(_, w) => {
                let (word, occurrences, closest_gap) = words.iter().find(|(x, _, _)| x == w)?;
                Some(RepetitionNode::Word {
                    item_id: *id,
                    word: word.clone(),
                    occurrences: *occurrences,
                    closest_gap: *closest_gap,
                })
            }
        }
    }

    /// The title of the text a row belongs to — what an "open this" needs beside the id.
    pub fn title_of(&self, item_id: u64) -> Option<String> {
        self.groups
            .borrow()
            .iter()
            .find(|(g, _, _)| *g == item_id)
            .map(|(_, t, _)| t.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(id: u64, title: &str, words: &[(&str, i64, i64)]) -> RepetitionGroup {
        (
            id,
            title.to_string(),
            words
                .iter()
                .map(|(w, c, g)| (w.to_string(), *c, *g))
                .collect(),
        )
    }

    #[test]
    fn a_group_becomes_a_parent_at_depth_zero_with_its_words_beneath() {
        let rows = rows_of(&[group(
            7,
            "The lamp",
            &[("glanced", 3, 12), ("suddenly", 2, 40)],
        )]);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].depth, 0);
        assert_eq!(rows[1].depth, 1);
        assert_eq!(rows[2].depth, 1);
        assert_eq!(rows[0].key, RepetitionTreeKey::Item(7));
        assert_eq!(rows[1].key, RepetitionTreeKey::Word(7, "glanced".into()));
    }

    /// The parent counts **words**, not occurrences — the number that says whether the row
    /// is worth opening.
    #[test]
    fn a_parent_counts_distinct_words_not_their_occurrences() {
        let rows = rows_of(&[group(1, "Scene", &[("a", 9, 3), ("b", 4, 5)])]);
        match &rows[0].item {
            RepetitionNode::Item { words, .. } => assert_eq!(*words, 2),
            other => panic!("expected an item row, got {other:?}"),
        }
    }

    /// The same word in two texts is two findings, not one — so the keys must differ, or
    /// the slice would treat the second as a duplicate of the first.
    #[test]
    fn the_same_word_in_two_texts_keeps_two_distinct_keys() {
        let rows = rows_of(&[
            group(1, "One", &[("glanced", 2, 10)]),
            group(2, "Two", &[("glanced", 3, 8)]),
        ]);
        let words: Vec<&RepetitionTreeKey> = rows
            .iter()
            .filter(|r| r.depth == 1)
            .map(|r| &r.key)
            .collect();
        assert_eq!(words.len(), 2);
        assert_ne!(words[0], words[1]);
    }

    #[test]
    fn groups_keep_the_order_they_were_given() {
        let rows = rows_of(&[
            group(5, "Later", &[("x", 2, 2)]),
            group(3, "Earlier", &[("y", 2, 2)]),
        ]);
        assert_eq!(rows[0].key, RepetitionTreeKey::Item(5));
        assert_eq!(rows[2].key, RepetitionTreeKey::Item(3));
    }

    #[test]
    fn an_empty_report_produces_no_rows() {
        assert!(rows_of(&[]).is_empty());
    }

    /// Re-pushing identical findings must not reload — a reload prunes and re-derives the
    /// expand set, so a rebuild that changed nothing would close rows the writer opened.
    #[test]
    fn setting_the_same_groups_twice_is_a_no_op() {
        let model = RepetitionTreeModel::new();
        let g = vec![group(1, "Scene", &[("a", 2, 3)])];
        model.set_groups(g.clone());
        let before = model.source().version_signal().get();
        model.set_groups(g);
        assert_eq!(
            model.source().version_signal().get(),
            before,
            "an identical set must not bump the source version"
        );
    }

    #[test]
    fn the_finding_count_sums_every_text() {
        let model = RepetitionTreeModel::new();
        model.set_groups(vec![
            group(1, "One", &[("a", 2, 3), ("b", 2, 3)]),
            group(2, "Two", &[("c", 2, 3)]),
        ]);
        assert_eq!(model.finding_count(), 3);
        assert!(!model.is_empty());
    }
}
