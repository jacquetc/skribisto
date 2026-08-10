// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The merge between a returning file and the book it left from, as a tree the reconcile step
//! can bind to.
//!
//! A [`TreeDataSource`] over the flat `Vec<MergeRowView>` the view-model computes. Same shape
//! as [`super::import_plan_source`] and the same reasoning behind it: there is no tree in the
//! data — every row carries an `indent`, exactly as the binder itself does — so parent and
//! child are derived, a row's parent being the nearest preceding row with a smaller indent.
//!
//! The tree it draws is the **project's**, not the file's. The writer reads their own book down
//! the first column and sees what the returning file has to say about each row beside it, with
//! a gap where the editor added something. That is why a row only the file has is given its
//! neighbours' depth by the view-model rather than the plan's own: the plan's indent is in the
//! file's coordinate space and would put an inserted chapter at the root.
//!
//! ## No real/mock split
//!
//! Nothing here queries a backend — its data is computed by the view-model and handed over — so
//! there is nothing for a second `#[cfg]`-gated implementation to differ on. Same reasoning
//! `import_plan_source` records for itself.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use teksilo::data::{FlatEntry, TreeDataSource};
use teksilo::prelude::Signal;

use crate::view_models::import_document::{MergeRowKey, MergeRowView};

struct Inner {
    rows: RefCell<Vec<MergeRowView>>,
    collapsed: RefCell<HashSet<MergeRowKey>>,
    /// Flattened, honouring collapse. Rebuilt whenever either changes.
    visible: RefCell<Vec<MergeRowKey>>,
    version: Signal<u64>,
}

/// The merge sequence, bindable.
#[derive(Clone)]
pub struct ImportMergeSource {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for ImportMergeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportMergeSource")
            .field("rows", &self.inner.rows.borrow().len())
            .finish()
    }
}

impl Default for ImportMergeSource {
    fn default() -> Self {
        Self::empty()
    }
}

impl ImportMergeSource {
    pub fn empty() -> Self {
        Self {
            inner: Rc::new(Inner {
                rows: RefCell::new(Vec::new()),
                collapsed: RefCell::new(HashSet::new()),
                visible: RefCell::new(Vec::new()),
                version: Signal::new(0),
            }),
        }
    }

    /// Replace the sequence.
    ///
    /// Collapse state is cleared with it: the rows are different rows, and carrying a
    /// collapsed key across would hide a subtree of a *different* destination's tree.
    pub fn set_rows(&self, rows: Vec<MergeRowView>) {
        *self.inner.rows.borrow_mut() = rows;
        self.inner.collapsed.borrow_mut().clear();
        self.rebuild();
    }

    pub fn is_empty(&self) -> bool {
        self.inner.rows.borrow().is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.rows.borrow().len()
    }

    /// The row behind a key, cloned.
    pub fn row(&self, key: MergeRowKey) -> Option<MergeRowView> {
        self.inner
            .rows
            .borrow()
            .iter()
            .find(|r| r.key == key)
            .cloned()
    }

    fn rebuild(&self) {
        let rows = self.inner.rows.borrow();
        let collapsed = self.inner.collapsed.borrow();

        let mut visible = Vec::with_capacity(rows.len());
        // The indent at or below which a row becomes visible again. `None` means nothing is
        // hiding anything.
        let mut hidden_below: Option<i64> = None;
        for row in rows.iter() {
            match hidden_below {
                Some(limit) if row.indent > limit => continue,
                _ => hidden_below = None,
            }
            visible.push(row.key);
            if collapsed.contains(&row.key) {
                hidden_below = Some(row.indent);
            }
        }
        drop(rows);
        drop(collapsed);

        *self.inner.visible.borrow_mut() = visible;
        self.inner
            .version
            .set(self.inner.version.get().wrapping_add(1));
    }

    fn has_children(&self, key: MergeRowKey) -> bool {
        let rows = self.inner.rows.borrow();
        let Some(at) = rows.iter().position(|r| r.key == key) else {
            return false;
        };
        rows.get(at + 1)
            .is_some_and(|next| next.indent > rows[at].indent)
    }
}

impl TreeDataSource for ImportMergeSource {
    type Item = MergeRowView;
    type Key = MergeRowKey;

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
            depth: row.indent.max(0) as usize,
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
        let at = rows.iter().position(|r| r.key == *key)?;
        let indent = rows[at].indent;
        rows[..at]
            .iter()
            .rev()
            .find(|r| r.indent < indent)
            .map(|r| r.key)
    }

    fn child_keys(&self, key: &Self::Key) -> Vec<Self::Key> {
        let rows = self.inner.rows.borrow();
        let Some(at) = rows.iter().position(|r| r.key == *key) else {
            return Vec::new();
        };
        let base = rows[at].indent;
        let mut out = Vec::new();
        for row in &rows[at + 1..] {
            if row.indent <= base {
                break;
            }
            if row.indent == base + 1 {
                out.push(row.key);
            }
        }
        out
    }

    fn version_signal(&self) -> Signal<u64> {
        self.inner.version.clone()
    }

    fn is_expanded(&self, key: &Self::Key) -> bool {
        !self.inner.collapsed.borrow().contains(key)
    }

    fn set_expanded(&self, key: &Self::Key, expanded: bool) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use skribisto_model::reconcile::{RowAction, RowStatus};

    fn row(indent: i64, uid: u128, title: &str) -> MergeRowView {
        MergeRowView {
            key: MergeRowKey::Current(uuid::Uuid::from_u128(uid)),
            indent,
            current_title: Some(title.into()),
            current_item_id: Some(uid as u64),
            incoming_title: Some(title.into()),
            incoming_key: None,
            status: RowStatus::Identical,
            moved: false,
            actions: vec![RowAction::CommentsOnly],
        }
    }

    /// Book / Chapter One (Scene A) / Chapter Two
    fn source() -> ImportMergeSource {
        let s = ImportMergeSource::empty();
        s.set_rows(vec![
            row(0, 1, "Book"),
            row(1, 2, "Chapter One"),
            row(2, 3, "Scene A"),
            row(1, 4, "Chapter Two"),
        ]);
        s
    }

    #[test]
    fn every_row_is_visible_until_something_is_collapsed() {
        assert_eq!(source().visible_count(), 4);
    }

    #[test]
    fn collapsing_a_chapter_hides_its_scenes_and_nothing_else() {
        let s = source();
        s.set_expanded(&MergeRowKey::Current(uuid::Uuid::from_u128(2)), false);
        assert_eq!(s.visible_count(), 3, "only Scene A is hidden");
        assert_eq!(
            s.key_at(2),
            Some(MergeRowKey::Current(uuid::Uuid::from_u128(4))),
            "Chapter Two follows the collapsed chapter"
        );
    }

    #[test]
    fn parent_and_children_are_derived_from_indent() {
        let s = source();
        let chapter = MergeRowKey::Current(uuid::Uuid::from_u128(2));
        assert_eq!(
            s.parent(&chapter),
            Some(MergeRowKey::Current(uuid::Uuid::from_u128(1)))
        );
        assert_eq!(
            s.child_keys(&chapter),
            vec![MergeRowKey::Current(uuid::Uuid::from_u128(3))]
        );
        assert!(
            s.child_keys(&MergeRowKey::Current(uuid::Uuid::from_u128(3)))
                .is_empty()
        );
    }

    /// Re-sourcing is what happens when the writer goes back and picks a different
    /// destination. A collapse carried across would hide a subtree of a tree that no longer
    /// exists — and the key it was recorded against may name a different row entirely.
    #[test]
    fn re_sourcing_forgets_what_was_collapsed() {
        let s = source();
        s.set_expanded(&MergeRowKey::Current(uuid::Uuid::from_u128(2)), false);
        assert_eq!(s.visible_count(), 3);

        s.set_rows(vec![row(0, 9, "Somewhere else"), row(1, 10, "Its chapter")]);
        assert_eq!(s.visible_count(), 2, "nothing is hidden in the new tree");
    }
}
