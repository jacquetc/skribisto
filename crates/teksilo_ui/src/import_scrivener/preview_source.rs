// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The new project's binder as the Review page previews it: a [`TreeDataSource`] over the
//! planned rows.
//!
//! The plan is flat, a depth per row, exactly as a binder is; the parent and children of a
//! row are derived from the depths, as `models::import_plan_source` does for the document
//! importer. Each binder the new project gets is a band at the top level, with its rows
//! under it, so the preview reads like the binder will.
//!
//! The rows are worded when the plan is set ([`PreviewRowView`]), in the view-model, from
//! the plan the converter's own mapping made: this source only lays them out and remembers
//! which are folded. A fresh plan keeps what the writer folded or unfolded, by row identity
//! (the Scrivener item a row represents), since every choice re-plans the whole binder.
//!
//! No real/mock split: it holds rows the view-model already has in memory.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::data::{FlatEntry, TreeDataSource};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::Signal;

/// A row's identity for as long as one plan is on screen: its ordinal in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreviewKey(pub u32);

/// Which row of the plan a row is, across plans: the binder it is in, the Scrivener item
/// it represents (`-1` for a row the import makes), and how many rows before it in the same
/// binder represent the same item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreviewIdentity {
    pub binder: u8,
    pub item: i64,
    pub occurrence: u32,
}

/// One row as the preview shows it.
#[derive(Clone)]
pub struct PreviewRowView {
    pub key: PreviewKey,
    pub identity: PreviewIdentity,
    /// Depth under its binder's band: the band itself is 0.
    pub depth: usize,
    /// A binder's band rather than a row.
    pub band: bool,
    pub title: LocalizedString,
    /// The title is the type's name, shown for a row whose own title is blank.
    pub title_is_placeholder: bool,
    /// What the row becomes, in the binder's own words.
    pub becomes: LocalizedString,
    /// Whether it is included in exports.
    pub in_exports: bool,
    /// The row arrives in the Trash.
    pub trashed: bool,
    /// Folded when it first appears.
    pub folded_by_default: bool,
}

struct Inner {
    rows: RefCell<Vec<PreviewRowView>>,
    /// The identities the writer folded or unfolded, against each row's default.
    toggled: RefCell<HashMap<PreviewIdentity, bool>>,
    collapsed: RefCell<HashSet<PreviewKey>>,
    visible: RefCell<Vec<PreviewKey>>,
    version: Signal<u64>,
}

/// The preview. Cheap to clone; clones share every row.
#[derive(Clone)]
pub struct PreviewSource {
    inner: Rc<Inner>,
}

impl Default for PreviewSource {
    fn default() -> Self {
        Self::new()
    }
}

impl PreviewSource {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Inner {
                rows: RefCell::new(Vec::new()),
                toggled: RefCell::new(HashMap::new()),
                collapsed: RefCell::new(HashSet::new()),
                visible: RefCell::new(Vec::new()),
                version: Signal::new(0),
            }),
        }
    }

    /// Replace the rows with a fresh plan's, keeping what the writer folded or unfolded.
    pub fn set_rows(&self, rows: Vec<PreviewRowView>) {
        let collapsed = {
            let toggled = self.inner.toggled.borrow();
            rows.iter()
                .filter(|row| {
                    toggled
                        .get(&row.identity)
                        .copied()
                        .unwrap_or(row.folded_by_default)
                })
                .map(|row| row.key)
                .collect()
        };
        *self.inner.rows.borrow_mut() = rows;
        *self.inner.collapsed.borrow_mut() = collapsed;
        self.rebuild();
    }

    /// Forget every row and every fold, for a project read afresh.
    pub fn clear(&self) {
        self.inner.toggled.borrow_mut().clear();
        self.set_rows(Vec::new());
    }

    /// Every row, folded or not, in order.
    pub fn rows(&self) -> Vec<PreviewRowView> {
        self.inner.rows.borrow().clone()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.rows.borrow().is_empty()
    }

    fn row(&self, key: PreviewKey) -> Option<PreviewRowView> {
        self.inner.rows.borrow().get(key.0 as usize).cloned()
    }

    fn has_children(&self, key: PreviewKey) -> bool {
        let rows = self.inner.rows.borrow();
        let at = key.0 as usize;
        match (rows.get(at), rows.get(at + 1)) {
            (Some(row), Some(next)) => next.depth > row.depth,
            _ => false,
        }
    }

    /// Flatten the rows, honouring folds, and tell the views.
    fn rebuild(&self) {
        let visible = {
            let rows = self.inner.rows.borrow();
            let collapsed = self.inner.collapsed.borrow();
            let mut visible = Vec::with_capacity(rows.len());
            let mut hidden_below: Option<usize> = None;
            for row in rows.iter() {
                if let Some(depth) = hidden_below {
                    if row.depth > depth {
                        continue;
                    }
                    hidden_below = None;
                }
                visible.push(row.key);
                if collapsed.contains(&row.key) {
                    hidden_below = Some(row.depth);
                }
            }
            visible
        };
        *self.inner.visible.borrow_mut() = visible;
        self.inner
            .version
            .set(self.inner.version.get().wrapping_add(1));
    }
}

impl TreeDataSource for PreviewSource {
    type Item = PreviewRowView;
    type Key = PreviewKey;

    fn visible_count(&self) -> usize {
        self.inner.visible.borrow().len()
    }

    fn with_entry<R>(
        &self,
        flat_index: usize,
        f: impl FnOnce(&Self::Item, &FlatEntry<Self::Key>) -> R,
    ) -> Option<R> {
        let key = *self.inner.visible.borrow().get(flat_index)?;
        let row = self.row(key)?;
        let entry = FlatEntry {
            node_id: key,
            depth: row.depth,
            has_children: self.has_children(key),
            is_expanded: !self.inner.collapsed.borrow().contains(&key),
        };
        Some(f(&row, &entry))
    }

    fn key_at(&self, flat_index: usize) -> Option<Self::Key> {
        self.inner.visible.borrow().get(flat_index).copied()
    }

    fn flat_index_of(&self, key: &Self::Key) -> Option<usize> {
        self.inner.visible.borrow().iter().position(|k| k == key)
    }

    fn parent(&self, key: &Self::Key) -> Option<Self::Key> {
        let rows = self.inner.rows.borrow();
        let at = key.0 as usize;
        let depth = rows.get(at)?.depth;
        rows[..at]
            .iter()
            .rev()
            .find(|r| r.depth < depth)
            .map(|r| r.key)
    }

    fn child_keys(&self, key: &Self::Key) -> Vec<Self::Key> {
        let rows = self.inner.rows.borrow();
        let at = key.0 as usize;
        let Some(base) = rows.get(at).map(|r| r.depth) else {
            return Vec::new();
        };
        rows[at + 1..]
            .iter()
            .take_while(|r| r.depth > base)
            .filter(|r| r.depth == base + 1)
            .map(|r| r.key)
            .collect()
    }

    fn version_signal(&self) -> Signal<u64> {
        self.inner.version.clone()
    }

    fn is_expanded(&self, key: &Self::Key) -> bool {
        !self.inner.collapsed.borrow().contains(key)
    }

    fn set_expanded(&self, key: &Self::Key, expanded: bool) {
        if let Some(row) = self.row(*key) {
            self.inner
                .toggled
                .borrow_mut()
                .insert(row.identity, !expanded);
        }
        {
            let mut collapsed = self.inner.collapsed.borrow_mut();
            if expanded {
                collapsed.remove(key);
            } else {
                collapsed.insert(*key);
            }
        }
        self.rebuild();
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        (key.0 as usize) < self.inner.rows.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::i18n::lit;

    fn row(key: u32, depth: usize, item: i64, folded: bool) -> PreviewRowView {
        PreviewRowView {
            key: PreviewKey(key),
            identity: PreviewIdentity {
                binder: 0,
                item,
                occurrence: 0,
            },
            depth,
            band: depth == 0,
            title: lit!(format!("row {key}")),
            title_is_placeholder: false,
            becomes: lit!(""),
            in_exports: true,
            trashed: false,
            folded_by_default: folded,
        }
    }

    /// A band, a part folded by default with two chapters, and a second part.
    fn source() -> PreviewSource {
        let source = PreviewSource::new();
        source.set_rows(vec![
            row(0, 0, -1, false),
            row(1, 1, 1, true),
            row(2, 2, 2, false),
            row(3, 2, 3, false),
            row(4, 1, 4, false),
        ]);
        source
    }

    #[test]
    fn a_folded_row_hides_what_is_under_it() {
        let source = source();
        assert_eq!(source.visible_count(), 3, "band, folded part, second part");
        assert_eq!(
            source.child_keys(&PreviewKey(1)),
            vec![PreviewKey(2), PreviewKey(3)]
        );
        assert_eq!(source.parent(&PreviewKey(3)), Some(PreviewKey(1)));
        source.set_expanded(&PreviewKey(1), true);
        assert_eq!(source.visible_count(), 5);
    }

    /// A choice re-plans the whole binder; the writer's folds follow the rows they were
    /// made on, not the positions.
    #[test]
    fn a_fresh_plan_keeps_the_writers_folds() {
        let source = source();
        source.set_expanded(&PreviewKey(1), true);
        source.set_expanded(&PreviewKey(4), false);
        // The same rows, one row inserted before them.
        source.set_rows(vec![
            row(0, 0, -1, false),
            row(1, 1, 9, false),
            row(2, 1, 1, true),
            row(3, 2, 2, false),
            row(4, 2, 3, false),
            row(5, 1, 4, false),
        ]);
        assert!(source.is_expanded(&PreviewKey(2)), "unfolded by the writer");
        assert!(!source.is_expanded(&PreviewKey(5)), "folded by the writer");
    }
}
