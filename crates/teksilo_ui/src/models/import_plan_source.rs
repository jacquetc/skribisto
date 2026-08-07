// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The reviewable import plan, as a tree the review panel can bind to.
//!
//! A [`TreeDataSource`] over `document_ingest`'s flat `Vec<PlannedRow>`. The plan
//! has no tree in it — a row carries an `indent`, exactly as the binder itself
//! does — so the parent/child relations here are derived: a row's parent is the
//! nearest preceding row with a smaller indent, and its children are the rows
//! that follow it at one deeper indent until the indent returns.
//!
//! ## No real/mock split
//!
//! Unlike the `models/` handles that query the backend, this one has nothing to
//! differ on between builds: its data is a DTO the view-model already holds in
//! memory, fetched through the same command in both. A second `#[cfg]`-gated
//! `mod imp` would only duplicate the projection — the same reasoning
//! `examples_list_model` records for its own single implementation.
//!
//! ## Why the per-row type signal lives here
//!
//! The framework's table editing gives the caller the *location* of an edit
//! (`editing_cell_signal` is a flat row/column index) and never the value, so a
//! live combo box in a column needs a reactive cell of its own per row, owned by
//! the caller. It lives here rather than in the view-model because the cell
//! delegate is handed this source and nothing else — and because a key that
//! outlives a re-source has to be a domain key, not the flat index the framework
//! hands back.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::data::{FlatEntry, TreeDataSource};
use teksilo::prelude::Signal;

use document_ingest::ImportPlan;
use document_ingest::plan::PlannedRow;
use skribisto_model::CreateType;

/// A row's durable identity for as long as one plan is on screen.
///
/// Its ordinal in the analysed plan — stable across expanding, collapsing and
/// retyping, which the framework's own flat row index is not. A fresh analysis
/// replaces the whole source, so these never have to survive one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlanRowKey(pub u32);

/// One row as the review panel shows it.
#[derive(Debug, Clone)]
pub struct PlanRowView {
    pub key: PlanRowKey,
    pub indent: i64,
    pub title: String,
    /// The ordinal lifted off the title, if any — shown so the writer can see
    /// what was taken away rather than discovering it later.
    pub stripped_ordinal: Option<String>,
    pub djot: String,
    pub word_count: usize,
    pub scene_breaks: usize,
    /// The editors' comments that came with this row's prose, carried whole rather
    /// than counted: the review tree shows how many, and the apply step hands these
    /// same anchors straight back to the backend. Counting them here and re-reading
    /// the files later would mean scanning every document twice.
    pub comments: Vec<document_ingest::plan::PlannedComment>,
    pub origin: String,
    /// Diagnostics belonging to this row.
    pub diagnostics: Vec<document_ingest::ImportDiagnostic>,
}

struct Inner {
    rows: RefCell<Vec<PlanRowView>>,
    /// The live type of each row. Separate from `rows` because it is what the
    /// combo column both reads and writes.
    /// `Option` because that is what a `ComboBox` binds, and the combo must be handed
    /// **this** signal rather than a snapshot of it — see [`ImportPlanSource::type_signal`].
    /// The model always keeps it `Some`; the combo only ever clears a selection when its
    /// *item source* mutates, and the type column's items are a fixed static list.
    types: RefCell<HashMap<PlanRowKey, Signal<Option<CreateType>>>>,
    /// Whether the writer has ticked this row itself. Whether it will actually be
    /// created also depends on its ancestors — see the view-model's `is_included`.
    included: RefCell<HashMap<PlanRowKey, Signal<bool>>>,
    collapsed: RefCell<HashSet<PlanRowKey>>,
    /// Flattened, honouring collapse. Rebuilt whenever either changes.
    visible: RefCell<Vec<PlanRowKey>>,
    version: Signal<u64>,
}

/// The plan, as a tree. Cheap to clone; every clone sees the same rows.
#[derive(Clone)]
pub struct ImportPlanSource {
    inner: Rc<Inner>,
}

impl ImportPlanSource {
    /// An empty source — what the panel shows before anything has been analysed.
    pub fn empty() -> Self {
        Self {
            inner: Rc::new(Inner {
                rows: RefCell::new(Vec::new()),
                types: RefCell::new(HashMap::new()),
                included: RefCell::new(HashMap::new()),
                collapsed: RefCell::new(HashSet::new()),
                visible: RefCell::new(Vec::new()),
                version: Signal::new(0),
            }),
        }
    }

    /// Replace everything with a freshly analysed plan.
    pub fn set_plan(&self, plan: &ImportPlan) {
        let mut rows = Vec::with_capacity(plan.rows.len());
        let mut types = HashMap::with_capacity(plan.rows.len());
        let mut included = HashMap::with_capacity(plan.rows.len());
        for (index, row) in plan.rows.iter().enumerate() {
            let key = PlanRowKey(index as u32);
            types.insert(key, Signal::new(Some(row.create_type)));
            included.insert(key, Signal::new(row.included));
            rows.push(view_of(key, row));
        }
        *self.inner.rows.borrow_mut() = rows;
        *self.inner.types.borrow_mut() = types;
        *self.inner.included.borrow_mut() = included;
        self.inner.collapsed.borrow_mut().clear();
        self.rebuild();
    }

    /// Every row, in plan order, ignoring collapse.
    pub fn rows(&self) -> Vec<PlanRowView> {
        self.inner.rows.borrow().clone()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.rows.borrow().is_empty()
    }

    pub fn row(&self, key: PlanRowKey) -> Option<PlanRowView> {
        self.inner.rows.borrow().get(key.0 as usize).cloned()
    }

    /// The live type signal for one row — what the combo column binds to.
    /// The row's **live** type signal, for the combo cell to bind.
    ///
    /// Handing the combo `Signal::new(Some(type_of(key)))` instead — a fresh signal
    /// holding a snapshot taken while the cell was being built — is what made the
    /// bulk "every level-2 heading is a Chapter" rule look broken: `set_type` wrote
    /// the real signal, the cell was watching a copy, and nothing re-sourced the
    /// table because retyping is not a shape change. The combo owns and observes
    /// whatever signal it is given, so it must be given this one.
    pub fn type_signal(&self, key: PlanRowKey) -> Option<Signal<Option<CreateType>>> {
        self.inner.types.borrow().get(&key).cloned()
    }

    pub fn type_of(&self, key: PlanRowKey) -> Option<CreateType> {
        self.inner.types.borrow().get(&key).and_then(|s| s.get())
    }

    /// Retype one row. Used by the combo column, and by the level-rule table when
    /// it reapplies itself.
    pub fn set_type(&self, key: PlanRowKey, kind: CreateType) {
        if let Some(signal) = self.inner.types.borrow().get(&key) {
            signal.set(Some(kind));
        }
    }

    /// The row's own include tick — what its checkbox binds to and writes.
    pub fn included_signal(&self, key: PlanRowKey) -> Option<Signal<bool>> {
        self.inner.included.borrow().get(&key).cloned()
    }

    /// Whether the writer ticked this row itself, ignoring its ancestors.
    pub fn is_ticked(&self, key: PlanRowKey) -> bool {
        self.inner
            .included
            .borrow()
            .get(&key)
            .is_some_and(|s| s.get())
    }

    pub fn set_ticked(&self, key: PlanRowKey, ticked: bool) {
        if let Some(signal) = self.inner.included.borrow().get(&key) {
            signal.set(ticked);
        }
    }

    /// Every ancestor of `key`, nearest first.
    pub fn ancestors(&self, key: PlanRowKey) -> Vec<PlanRowKey> {
        let rows = self.inner.rows.borrow();
        let Some(at) = rows.iter().position(|r| r.key == key) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut ceiling = rows[at].indent;
        for row in rows[..at].iter().rev() {
            if row.indent < ceiling {
                out.push(row.key);
                ceiling = row.indent;
            }
        }
        out
    }

    /// Every key, in plan order — the order rows must be created in.
    pub fn keys_in_order(&self) -> Vec<PlanRowKey> {
        self.inner.rows.borrow().iter().map(|r| r.key).collect()
    }

    /// Rows that follow `key` at a strictly greater indent — its whole subtree,
    /// the same "a subtree is X plus everything deeper after it" rule the binder
    /// itself uses.
    pub fn subtree(&self, key: PlanRowKey) -> Vec<PlanRowKey> {
        let rows = self.inner.rows.borrow();
        let Some(start) = rows.iter().position(|r| r.key == key) else {
            return Vec::new();
        };
        let base = rows[start].indent;
        let mut out = vec![key];
        for row in &rows[start + 1..] {
            if row.indent <= base {
                break;
            }
            out.push(row.key);
        }
        out
    }

    fn rebuild(&self) {
        let rows = self.inner.rows.borrow();
        let collapsed = self.inner.collapsed.borrow();

        let mut visible = Vec::with_capacity(rows.len());
        // The indent at or below which a row becomes visible again. `None` means
        // nothing is hiding anything.
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

    fn has_children(&self, key: PlanRowKey) -> bool {
        let rows = self.inner.rows.borrow();
        let Some(at) = rows.iter().position(|r| r.key == key) else {
            return false;
        };
        rows.get(at + 1)
            .is_some_and(|next| next.indent > rows[at].indent)
    }
}

fn view_of(key: PlanRowKey, row: &PlannedRow) -> PlanRowView {
    PlanRowView {
        key,
        indent: row.indent,
        title: row.title.clone(),
        stripped_ordinal: row.stripped_ordinal.clone(),
        djot: row.djot.clone(),
        word_count: row.word_count,
        scene_breaks: row.scene_breaks,
        comments: row.comments.clone(),
        origin: row.origin.clone(),
        diagnostics: row.diagnostics.clone(),
    }
}

impl TreeDataSource for ImportPlanSource {
    type Item = PlanRowView;
    type Key = PlanRowKey;

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
            // Named `node_id` on the struct, but generic over the key type — it
            // carries this source's own `PlanRowKey`, not a framework NodeId.
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
    use document_ingest::plan::PlannedRow;

    fn row(indent: i64, title: &str, kind: CreateType) -> PlannedRow {
        PlannedRow {
            indent,
            create_type: kind,
            title: title.into(),
            stripped_ordinal: None,
            djot: String::new(),
            scene_breaks: 0,
            word_count: 0,
            origin: "a.md".into(),
            included: true,
            comments: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Book / Chapter One (Scene A, Scene B) / Chapter Two
    fn source() -> ImportPlanSource {
        let plan = ImportPlan {
            rows: vec![
                row(0, "Book", CreateType::Book),
                row(1, "Chapter One", CreateType::Chapter),
                row(2, "Scene A", CreateType::Scene),
                row(2, "Scene B", CreateType::Scene),
                row(1, "Chapter Two", CreateType::Chapter),
            ],
            diagnostics: Vec::new(),
        };
        let source = ImportPlanSource::empty();
        source.set_plan(&plan);
        source
    }

    fn titles(source: &ImportPlanSource) -> Vec<String> {
        (0..source.visible_count())
            .filter_map(|i| source.key_at(i))
            .filter_map(|k| source.row(k))
            .map(|r| r.title)
            .collect()
    }

    #[test]
    fn the_flat_plan_becomes_a_tree_through_its_indents() {
        let s = source();
        assert_eq!(s.visible_count(), 5);
        assert_eq!(s.parent(&PlanRowKey(2)), Some(PlanRowKey(1)));
        assert_eq!(s.parent(&PlanRowKey(1)), Some(PlanRowKey(0)));
        assert_eq!(s.parent(&PlanRowKey(0)), None);
        assert_eq!(
            s.child_keys(&PlanRowKey(1)),
            vec![PlanRowKey(2), PlanRowKey(3)]
        );
        assert_eq!(
            s.child_keys(&PlanRowKey(0)),
            vec![PlanRowKey(1), PlanRowKey(4)],
            "grandchildren are not children"
        );
    }

    #[test]
    fn collapsing_a_row_hides_its_whole_subtree() {
        let s = source();
        s.set_expanded(&PlanRowKey(1), false);
        assert_eq!(
            titles(&s),
            vec!["Book", "Chapter One", "Chapter Two"],
            "the two scenes under Chapter One are hidden"
        );
        s.set_expanded(&PlanRowKey(1), true);
        assert_eq!(titles(&s).len(), 5);
    }

    #[test]
    fn collapsing_the_root_hides_everything_below_it() {
        let s = source();
        s.set_expanded(&PlanRowKey(0), false);
        assert_eq!(titles(&s), vec!["Book"]);
    }

    /// The rule the binder itself uses: a subtree is a row plus everything that
    /// follows at a strictly greater indent.
    #[test]
    fn a_subtree_is_the_row_and_everything_deeper_after_it() {
        let s = source();
        assert_eq!(
            s.subtree(PlanRowKey(1)),
            vec![PlanRowKey(1), PlanRowKey(2), PlanRowKey(3)]
        );
        assert_eq!(s.subtree(PlanRowKey(3)), vec![PlanRowKey(3)]);
        assert_eq!(s.subtree(PlanRowKey(0)).len(), 5);
    }

    /// A signal handed out before a retype still sees it — which is what a combo
    /// cell needs, since it takes its signal once when the cell is built.
    ///
    /// This test passed all along while the type column was in fact binding
    /// `Signal::new(Some(type_of(key)))` — a *copy* — so "the cell sees the change"
    /// was a claim about a cell nobody had wired that way. The model was right; the
    /// panel was not. The column now binds `type_signal(key)` itself.
    #[test]
    fn retyping_a_row_is_visible_through_its_signal() {
        let s = source();
        let signal = s.type_signal(PlanRowKey(2)).expect("row 2 has a type");
        assert_eq!(signal.get(), Some(CreateType::Scene));
        s.set_type(PlanRowKey(2), CreateType::Note);
        assert_eq!(
            signal.get(),
            Some(CreateType::Note),
            "the cell sees the change"
        );
        assert_eq!(s.type_of(PlanRowKey(2)), Some(CreateType::Note));
    }

    /// Retyping must NOT re-source the table: the view rebuilding on every combo
    /// change would throw away scroll position and the writer's expand state, and
    /// the cells no longer need it now that they bind the live signal.
    #[test]
    fn a_retype_is_not_a_shape_change() {
        let s = source();
        let before = s.version_signal().get();
        s.set_type(PlanRowKey(1), CreateType::Note);
        assert_eq!(
            s.version_signal().get(),
            before,
            "retyping changes a value, not the shape of the tree"
        );
    }

    #[test]
    fn the_version_signal_moves_when_the_shape_does() {
        let s = source();
        let before = s.version_signal().get();
        s.set_expanded(&PlanRowKey(1), false);
        assert_ne!(s.version_signal().get(), before, "the view must rebuild");
    }

    #[test]
    fn a_fresh_plan_replaces_everything_including_collapse_state() {
        let s = source();
        s.set_expanded(&PlanRowKey(0), false);
        assert_eq!(s.visible_count(), 1);

        s.set_plan(&ImportPlan {
            rows: vec![row(0, "Only", CreateType::Scene)],
            diagnostics: Vec::new(),
        });
        assert_eq!(titles(&s), vec!["Only"], "no stale collapse survives");
    }

    #[test]
    fn an_empty_source_is_a_valid_tree_with_nothing_in_it() {
        let s = ImportPlanSource::empty();
        assert!(s.is_empty());
        assert_eq!(s.visible_count(), 0);
        assert_eq!(s.key_at(0), None);
    }
}
