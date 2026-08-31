// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **The search results, as a tree**: an item, and the hits inside it.
//!
//! Level 0 is a `BinderItem` carrying how many times the query occurs anywhere in
//! it. Level 1 is one row per occurrence, ordered by source and then by position,
//! each showing which kind of text it landed in.
//!
//! ## Why the second level is not in reading order
//!
//! It is ordered by **source first**, prose before everything else, and only then
//! by position. A scene's prose hits therefore come before its comment hits even
//! where the comment is anchored to the first paragraph. That is a deliberate
//! departure from a file-and-line search panel, and it is what makes a long result
//! readable: forty prose hits stay in one run instead of being interleaved with the
//! two annotations that happen to sit among them.
//!
//! ## Why the children are not there until you open the row
//!
//! Because there is no bound on them. A `SearchResult` is one matching *field* and
//! the row set is capped at ten thousand of those, but a field holds as many hits
//! as the prose holds -- a one-letter query over a long manuscript is millions.
//! Worse, nothing carries them: the offsets are computed during the scan and thrown
//! away, so materialising every one would mean re-scanning the whole corpus on the
//! UI thread, on a three-hundred-millisecond debounce.
//!
//! So a row **declares** its children ([`TreeRow::with_children`]) and fetches them
//! when it is opened. The declaration is what makes that possible at all: a slice
//! derives "has children" from the rows it was handed, so a branch that emits
//! nothing draws no chevron, and a chevron nobody can click is a branch nobody can
//! ever open.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::search_management_commands;
use frontend::common::entities::MatchField;
use frontend::direct_access::SearchResultDto;
use frontend::search_management::OccurrencesForResultDto;
use teksilo::data::{FlatEntry, TreeDataSlice, TreeDataSource, TreeRow};
use teksilo::prelude::Signal;

use super::{NameContext, SearchResultsModel, label_and_badge};

/// Stable per-row identity.
///
/// An item is keyed by its **store id**, not by its uid, and that is enough here
/// where it would not be in the binder. The expand set has to survive a `reload`,
/// and reloads happen when a search re-runs or when a branch is filled in -- across
/// neither of those does a `BinderItem`'s id change. It changes when a project is
/// loaded, and a project load takes the whole result set with it, so there is
/// nothing left to have remembered. Resolving a durable uid per result would be a
/// batched read on the keystroke path, bought with nothing.
///
/// An occurrence is keyed by the row it is in and where it starts. The row id is
/// re-minted by every search, which is correct: so is the occurrence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SearchTreeKey {
    Item(u64),
    Occurrence(u64, i64),
}

/// One row of the tree, of either kind.
///
/// Both kinds share a struct, as the trash tree's rows do, with `is_item` telling
/// them apart -- the delegate has to rebuild the key from the node, and a tagged
/// union of two structs cannot be handed to one `TreeDataSlice`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SearchNode {
    /// Item rows: what the item is **called on screen**, which is not always its
    /// title. An untitled chapter is "Chapter 3" here exactly as it is in the
    /// outline, because both ask [`NameContext`] the same question.
    pub title: String,
    /// Occurrence rows: the matched text, and what sits either side of it.
    pub snippet_before: String,
    pub snippet_match: String,
    pub snippet_after: String,
    /// Which of the two kinds this is.
    pub is_item: bool,
    /// Item rows: how many times the query occurs anywhere in this item. The true
    /// count, which is not always the number of children -- see [`Self::truncated`].
    pub occurrence_count: u64,
    /// Item rows: whether the children stop short of `occurrence_count`.
    pub truncated: bool,
    /// The `BinderItem` this row is, or is under.
    pub binder_item_id: u64,
    /// Occurrence rows: which result row it belongs to, and where it starts.
    pub result_id: u64,
    pub char_start: i64,
    pub char_len: i64,
    /// Occurrence rows: which kind of text it landed in, for the icon.
    pub match_field: MatchField,
}

impl SearchNode {
    /// The key a delegate rebuilds for this row.
    pub fn key(&self) -> SearchTreeKey {
        if self.is_item {
            SearchTreeKey::Item(self.binder_item_id)
        } else {
            SearchTreeKey::Occurrence(self.result_id, self.char_start)
        }
    }
}

/// One fetched occurrence, kept against the row it came from.
#[derive(Clone, Debug)]
struct Occurrence {
    char_start: i64,
    char_len: i64,
    before: String,
    matched: String,
    after: String,
}

#[derive(Clone)]
pub struct SearchTreeModel {
    slice: TreeDataSlice<SearchTreeKey, SearchNode>,
    ctx: Rc<AppContext>,
    results: SearchResultsModel,
    work_id: Signal<Option<u64>>,
    /// Occurrences already fetched, by the result row they belong to.
    fetched: Rc<RefCell<HashMap<u64, Vec<Occurrence>>>>,
    /// Result rows whose fetch came back short of the row's own count.
    truncated: Rc<RefCell<HashSet<u64>>>,
    /// What each matched item is called on screen, and its ordinal.
    ///
    /// Read once per search rather than per row: the generated name of one untitled
    /// chapter depends on every item before it, so naming N of them from their ids
    /// alone is one pass over the manuscript, not N. Refreshed only in
    /// [`Self::reload`] -- the reloads a fetch performs must not re-read the binder.
    names: Rc<RefCell<HashMap<u64, String>>>,
    /// Guards the re-entrant `reload` a fetch performs.
    filling: Rc<Cell<bool>>,
    /// Where the tree is scrolled to.
    ///
    /// Held here rather than inside the view, because the view does not last: the
    /// search dock's content is torn down and rebuilt whenever the layout changes,
    /// and the first result a writer activates reveals the preview band, which is
    /// such a change. Without this they scroll to chapter fourteen, click a hit,
    /// and land back at chapter one.
    ///
    /// Animated, because the view animates it; a plain signal makes every wheel
    /// notch a jump.
    scroll: Signal<f32>,
    /// What the writer has dismissed: whole result rows, and single occurrences.
    ///
    /// A dismissal **leaves the list**, which is what makes it a dismissal rather
    /// than a tick-box: the row goes and Replace All stops counting it. Held here
    /// as plain sets the view model writes, so the tree can filter without asking
    /// it a question per row.
    dismissed_rows: Rc<RefCell<HashSet<u64>>>,
    dismissed_occurrences: Rc<RefCell<HashSet<(u64, i64)>>>,
}

impl SearchTreeModel {
    pub fn new(
        ctx: Rc<AppContext>,
        results: SearchResultsModel,
        work_id: Signal<Option<u64>>,
    ) -> Self {
        let slice = TreeDataSlice::new();
        let fetched: Rc<RefCell<HashMap<u64, Vec<Occurrence>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let truncated: Rc<RefCell<HashSet<u64>>> = Rc::new(RefCell::new(HashSet::new()));
        let names: Rc<RefCell<HashMap<u64, String>>> = Rc::new(RefCell::new(HashMap::new()));
        let dismissed_rows: Rc<RefCell<HashSet<u64>>> = Rc::new(RefCell::new(HashSet::new()));
        let dismissed_occurrences: Rc<RefCell<HashSet<(u64, i64)>>> =
            Rc::new(RefCell::new(HashSet::new()));
        {
            let results = results.clone();
            let fetched = fetched.clone();
            let truncated = truncated.clone();
            let names = names.clone();
            let d_rows = dismissed_rows.clone();
            let d_occ = dismissed_occurrences.clone();
            slice.set_source(move || {
                rows(
                    &results,
                    &fetched.borrow(),
                    &truncated.borrow(),
                    &names.borrow(),
                    &d_rows.borrow(),
                    &d_occ.borrow(),
                )
            });
        }
        Self {
            slice,
            ctx,
            results,
            work_id,
            fetched,
            truncated,
            names,
            filling: Rc::new(Cell::new(false)),
            // The very same handles the row source captured. Minting fresh ones
            // here compiles, shares nothing, and leaves `set_dismissed` writing to
            // a set that nothing reads -- the rows stayed put and the state was
            // right, which is the hardest shape of wrong to see.
            scroll: Signal::new_animated(0.0),
            dismissed_rows,
            dismissed_occurrences,
        }
    }

    /// Tell the tree what is dismissed, and rebuild without it.
    pub fn set_dismissed(&self, rows: HashSet<u64>, occurrences: HashSet<(u64, i64)>) {
        self.dismissed_rows.replace(rows);
        self.dismissed_occurrences.replace(occurrences);
        self.slice.reload();
    }

    /// Re-read what every matched item is called.
    ///
    /// The result rows carry `item_title`, which is the item's *title* and is empty
    /// for every untitled chapter -- so a tree built from it alone shows blank rows
    /// where the outline shows "Chapter 3". The name is not on the row because it
    /// cannot be: it is generated from the item's position among all the others.
    fn refresh_names(&self) {
        let mut names = self.names.borrow_mut();
        names.clear();
        let Some(work_id) = self.work_id.get() else {
            return;
        };
        let mut wanted: Vec<u64> = Vec::new();
        self.results.for_each(|r| {
            if !wanted.contains(&r.binder_item_id) {
                wanted.push(r.binder_item_id);
            }
        });
        if wanted.is_empty() {
            return;
        }
        let context = NameContext::read(&self.ctx, work_id);
        for item_id in wanted {
            let Some(item) = context.item(item_id) else {
                continue;
            };
            // No ordinal badge: `label_and_badge` is asked for the name only. An
            // untitled chapter therefore reads "Chapter 3" -- the generated name
            // carries the number already -- and a titled one reads its title, which
            // is what a result row wants. The outline's separate badge column is an
            // outline affordance and does not belong beside a search hit.
            let generated = context.generated_name(item);
            let (name, _) = label_and_badge(&item.title, generated.as_deref(), None);
            names.insert(item_id, name);
        }
    }

    /// Where the tree is scrolled to, for the view to drive and be driven by.
    pub fn scroll(&self) -> Signal<f32> {
        self.scroll.clone()
    }

    pub fn slice(&self) -> TreeDataSlice<SearchTreeKey, SearchNode> {
        self.slice.clone()
    }

    /// Rebuild from a fresh result set.
    ///
    /// Everything fetched is dropped: the rows it was keyed against no longer
    /// exist, since a search re-mints every one of them. Keeping it would answer a
    /// new query with an old query's hits.
    pub fn reload(&self) {
        // A new result set is a new list, so the old position means nothing in it.
        // This is the one place the scroll *should* go back to the top, and it is
        // why keeping it across a rebuild is not the same as never resetting it.
        self.scroll.set(0.0);
        self.fetched.borrow_mut().clear();
        self.truncated.borrow_mut().clear();
        self.refresh_names();
        self.slice.reload();
    }

    /// Re-read some rows, and leave the rest of the tree exactly where it is.
    ///
    /// What a replace of one hit needs. [`Self::reload`] is for a *new* result set
    /// and throws away everything keyed against the old one -- every fetched
    /// occurrence, and the scroll position -- because none of it means anything
    /// against rows that no longer exist. After a scoped replace almost all of them
    /// do still exist: the backend restates the rows it rewrote and leaves the
    /// others alone, so the only stale thing in this model is what was fetched
    /// under `rows`.
    ///
    /// `rows` is read from **before** the replace, because a row that lost its last
    /// hit is gone from the result set by the time this runs and nothing here could
    /// name it any more -- its fetched occurrences would sit in the map forever,
    /// keyed to an id nothing looks up.
    ///
    /// Items still open are refilled on the spot rather than on next expand: an open
    /// branch whose occurrences were just dropped renders as a row with a chevron
    /// and nothing under it, which is the one thing the fetch-on-expand design is
    /// there to prevent.
    pub fn refresh_rows(&self, items: &[u64], rows: &[u64]) {
        {
            let mut fetched = self.fetched.borrow_mut();
            let mut truncated = self.truncated.borrow_mut();
            for id in rows {
                fetched.remove(id);
                truncated.remove(id);
            }
        }
        for item in items {
            if self.slice.is_expanded(&SearchTreeKey::Item(*item)) {
                self.fill(*item);
            }
        }
        self.slice.reload();
    }

    /// Close every item.
    ///
    /// Deliberately without a matching "open every item": that would fetch every
    /// occurrence of every field, which is the work this model is lazy to avoid.
    /// What has already been fetched is kept — reopening a row it has seen costs
    /// nothing, and the writer collapsing to regain the outline has not said they
    /// are finished with it.
    pub fn collapse_all(&self) {
        self.slice.set_expanded_keys(&[]);
    }

    /// Whether anything is open, for the control that closes it.
    pub fn any_expanded(&self) -> bool {
        !self.slice.expanded_keys().is_empty()
    }

    /// Where every fetched occurrence of one result row starts.
    ///
    /// What "replace just this one" needs: the replace runs over the whole result
    /// set minus exclusions, so singling one occurrence out means naming all the
    /// others. Empty for a row nobody has opened, which is correct — nothing there
    /// can be singled out yet either.
    pub fn occurrence_offsets(&self, result_id: u64) -> Vec<i64> {
        self.fetched
            .borrow()
            .get(&result_id)
            .map(|list| list.iter().map(|o| o.char_start).collect())
            .unwrap_or_default()
    }

    /// Whether `TEKSILO_SEARCH_DEBUG` asked for a trace of the fetch, in a build
    /// with the `debug-traces` feature.
    ///
    /// Behind a feature as well as a variable, and the traces are `#[cfg]` out
    /// rather than merely switched off: a runtime `false` still leaves every
    /// format string in the binary. Off, none of this exists.
    #[cfg(feature = "debug-traces")]
    fn debug() -> bool {
        static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ON.get_or_init(|| std::env::var_os("TEKSILO_SEARCH_DEBUG").is_some())
    }

    /// Fetch every occurrence of every field of one item, once.
    fn fill(&self, item_id: u64) {
        if self.filling.get() {
            #[cfg(feature = "debug-traces")]
            if Self::debug() {
                eprintln!("FILL item={item_id} refused: already filling");
            }
            return;
        }
        let Some(work_id) = self.work_id.get() else {
            #[cfg(feature = "debug-traces")]
            if Self::debug() {
                eprintln!("FILL item={item_id} refused: no work id");
            }
            return;
        };
        let mut wanted: Vec<u64> = Vec::new();
        self.results.for_each(|r| {
            if r.binder_item_id == item_id && !self.fetched.borrow().contains_key(&r.id) {
                wanted.push(r.id);
            }
        });
        #[cfg(feature = "debug-traces")]
        if Self::debug() {
            eprintln!("FILL item={item_id} work={work_id} rows={wanted:?}");
        }
        if wanted.is_empty() {
            return;
        }
        self.filling.set(true);
        for result_id in wanted {
            let dto = OccurrencesForResultDto { work_id, result_id };
            // A row that cannot be opened is left unfetched rather than shown as
            // empty: the next attempt tries again, and an item whose branch is
            // genuinely gone simply has nothing under it.
            let found = match search_management_commands::occurrences_for_result(&self.ctx, &dto) {
                Ok(found) => found,
                // `err` is read only by the trace below, so it is bound only
                // where the trace exists.
                #[cfg_attr(not(feature = "debug-traces"), allow(unused_variables))]
                Err(err) => {
                    #[cfg(feature = "debug-traces")]
                    if Self::debug() {
                        eprintln!("FILL row={result_id} failed: {err:#}");
                    }
                    continue;
                }
            };
            #[cfg(feature = "debug-traces")]
            if Self::debug() {
                eprintln!(
                    "FILL row={result_id} got={} truncated={}",
                    found.char_starts.len(),
                    found.truncated
                );
            }
            if found.truncated {
                self.truncated.borrow_mut().insert(result_id);
            }
            let list: Vec<Occurrence> = found
                .char_starts
                .iter()
                .zip(&found.char_lens)
                .zip(&found.snippets_before)
                .zip(&found.snippets_match)
                .zip(&found.snippets_after)
                .map(|((((start, len), before), matched), after)| Occurrence {
                    char_start: *start,
                    char_len: *len,
                    before: before.clone(),
                    matched: matched.clone(),
                    after: after.clone(),
                })
                .collect();
            self.fetched.borrow_mut().insert(result_id, list);
        }
        self.filling.set(false);
        self.slice.reload();
        #[cfg(feature = "debug-traces")]
        if Self::debug() {
            eprintln!(
                "FILL item={item_id} rows now {}",
                self.slice.visible_count()
            );
        }
    }
}

/// The model **is** the tree's data source, and that is the point.
///
/// Everything delegates to the slice except [`set_expanded`](Self::set_expanded),
/// which fetches the branch first. Hanging the fetch off the row widget's chevron
/// callback instead looks equivalent and is not: a `TreeView` opens a branch from
/// the chevron, from the keyboard, and from an accessibility action, and only the
/// first of those goes through the widget. The other two call the *source*. With
/// the fetch on the widget a keyboard user could never open a branch at all --
/// found by the live probe, which opens rows the way a keyboard does.
impl TreeDataSource for SearchTreeModel {
    type Item = SearchNode;
    type Key = SearchTreeKey;

    fn visible_count(&self) -> usize {
        self.slice.visible_count()
    }

    fn with_entry<R>(
        &self,
        flat_index: usize,
        f: impl FnOnce(&Self::Item, &FlatEntry<Self::Key>) -> R,
    ) -> Option<R> {
        self.slice.with_entry(flat_index, f)
    }

    fn key_at(&self, flat_index: usize) -> Option<Self::Key> {
        self.slice.key_at(flat_index)
    }

    fn flat_index_of(&self, key: &Self::Key) -> Option<usize> {
        self.slice.flat_index_of(key)
    }

    fn parent(&self, key: &Self::Key) -> Option<Self::Key> {
        self.slice.parent(key)
    }

    fn child_keys(&self, key: &Self::Key) -> Vec<Self::Key> {
        self.slice.child_keys(key)
    }

    fn version_signal(&self) -> Signal<u64> {
        self.slice.version_signal()
    }

    fn is_expanded(&self, key: &Self::Key) -> bool {
        self.slice.is_expanded(key)
    }

    /// **The fetch happens here, before the expand.** A row that declared children
    /// and opens onto nothing is the one thing the declaration makes possible, and
    /// the only place to be sure it cannot happen is the one call every route to
    /// opening a branch passes through.
    fn set_expanded(&self, key: &Self::Key, expanded: bool) {
        #[cfg(feature = "debug-traces")]
        if Self::debug() {
            eprintln!("SET_EXPANDED key={key:?} expanded={expanded}");
        }
        if expanded && let SearchTreeKey::Item(item_id) = key {
            self.fill(*item_id);
        }
        self.slice.set_expanded(key, expanded);
        #[cfg(feature = "debug-traces")]
        if Self::debug() {
            eprintln!(
                "SET_EXPANDED done key={key:?} is_expanded={} visible={}",
                self.slice.is_expanded(key),
                self.slice.visible_count()
            );
        }
    }

    fn first_changed_index(&self) -> Option<usize> {
        self.slice.first_changed_index()
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.slice.contains_key(key)
    }
}

/// Where a source sorts among its siblings under one item.
///
/// The manuscript first, then what describes it, then what is attached to it. A
/// total `match`, so a ninth `MatchField` fails the build here rather than sorting
/// silently to the front.
fn field_rank(field: &MatchField) -> u8 {
    match field {
        MatchField::Body => 0,
        MatchField::Epigraph => 1,
        MatchField::Synopsis => 2,
        MatchField::Title => 3,
        MatchField::Label => 4,
        MatchField::Comment => 5,
        MatchField::CommentReply => 6,
        MatchField::Footnote => 7,
    }
}

/// The whole row stream, item by item, in the order the results arrived.
///
/// The result set is already in binder order -- the scan walks the binder's ordered
/// stream -- so grouping by item while preserving first appearance keeps the tree in
/// the order a writer reads the book, without a sort.
fn rows(
    results: &SearchResultsModel,
    fetched: &HashMap<u64, Vec<Occurrence>>,
    truncated: &HashSet<u64>,
    names: &HashMap<u64, String>,
    dismissed_rows: &HashSet<u64>,
    dismissed_occurrences: &HashSet<(u64, i64)>,
) -> Vec<TreeRow<SearchTreeKey, SearchNode>> {
    let mut order: Vec<u64> = Vec::new();
    let mut by_item: HashMap<u64, Vec<SearchResultDto>> = HashMap::new();
    results.for_each(|r| {
        // A dismissed row is gone, not unticked. An item whose every row went with
        // it never reaches `order`, so the item disappears too — which is what a
        // writer means by dismissing all of it.
        if dismissed_rows.contains(&r.id) {
            return;
        }
        if !by_item.contains_key(&r.binder_item_id) {
            order.push(r.binder_item_id);
        }
        by_item.entry(r.binder_item_id).or_default().push(r.clone());
    });

    let mut out = Vec::new();
    for item_id in order {
        let Some(fields) = by_item.get(&item_id) else {
            continue;
        };
        // Minus what has been dismissed inside it, or the badge counts hits the
        // writer has already said they do not want.
        let dismissed_here = dismissed_occurrences
            .iter()
            .filter(|(row, _)| fields.iter().any(|f| f.id == *row))
            .count() as u64;
        let count: u64 = fields
            .iter()
            .map(|f| f.occurrence_count)
            .sum::<u64>()
            .saturating_sub(dismissed_here);
        // The screen name, falling back to whatever the row carried only if the
        // item has gone from the binder under us -- which leaves the writer the
        // stale title rather than a blank row.
        let title = names.get(&item_id).cloned().unwrap_or_else(|| {
            fields
                .first()
                .map(|f| f.item_title.clone())
                .unwrap_or_default()
        });
        let short = fields.iter().any(|f| truncated.contains(&f.id));
        out.push(
            TreeRow::new(
                SearchTreeKey::Item(item_id),
                SearchNode {
                    title,
                    is_item: true,
                    occurrence_count: count,
                    truncated: short,
                    binder_item_id: item_id,
                    ..Default::default()
                },
                0,
            )
            // Declared, not derived: the children are not here until the row is
            // opened, and without this it could never be opened. See the module note.
            .with_children(count > 0),
        );

        // Children, source first and then position. Sorted across the item's fields
        // rather than within each, so the prose of a scene is one run even where its
        // synopsis matched between two of them.
        let mut children: Vec<(u8, i64, &SearchResultDto, &Occurrence)> = Vec::new();
        for field in fields {
            let Some(list) = fetched.get(&field.id) else {
                continue;
            };
            for occurrence in list {
                if dismissed_occurrences.contains(&(field.id, occurrence.char_start)) {
                    continue;
                }
                children.push((
                    field_rank(&field.match_field),
                    occurrence.char_start,
                    field,
                    occurrence,
                ));
            }
        }
        children.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        for (_, _, field, occurrence) in children {
            out.push(TreeRow::new(
                SearchTreeKey::Occurrence(field.id, occurrence.char_start),
                SearchNode {
                    snippet_before: occurrence.before.clone(),
                    snippet_match: occurrence.matched.clone(),
                    snippet_after: occurrence.after.clone(),
                    is_item: false,
                    binder_item_id: item_id,
                    result_id: field.id,
                    char_start: occurrence.char_start,
                    char_len: occurrence.char_len,
                    match_field: field.match_field.clone(),
                    ..Default::default()
                },
                1,
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Prose first, then position.** The ordering the second level exists to
    /// give: a scene's prose hits in one run, then its synopsis, then what is
    /// attached to it — not interleaved by where they happen to fall.
    #[test]
    fn a_source_sorts_before_its_position() {
        let mut all = [
            MatchField::Footnote,
            MatchField::Body,
            MatchField::CommentReply,
            MatchField::Epigraph,
            MatchField::Comment,
            MatchField::Label,
            MatchField::Title,
            MatchField::Synopsis,
        ];
        all.sort_by_key(field_rank);
        assert_eq!(
            all,
            [
                MatchField::Body,
                MatchField::Epigraph,
                MatchField::Synopsis,
                MatchField::Title,
                MatchField::Label,
                MatchField::Comment,
                MatchField::CommentReply,
                MatchField::Footnote,
            ]
        );
    }

    /// Every source ranks differently from every other. A tie would make two
    /// sources interleave by position, which is the thing the rank exists to stop.
    #[test]
    fn no_two_sources_share_a_rank() {
        let all = [
            MatchField::Body,
            MatchField::Epigraph,
            MatchField::Synopsis,
            MatchField::Title,
            MatchField::Label,
            MatchField::Comment,
            MatchField::CommentReply,
            MatchField::Footnote,
        ];
        let mut ranks: Vec<u8> = all.iter().map(field_rank).collect();
        ranks.sort_unstable();
        ranks.dedup();
        assert_eq!(ranks.len(), all.len(), "two sources share a rank");
    }

    /// A row rebuilds its own key, which is what the delegate does to answer a
    /// click. The two kinds must never collide: an item and an occurrence can
    /// carry the same number and mean different things.
    #[test]
    fn the_two_kinds_of_row_key_apart() {
        let item = SearchNode {
            is_item: true,
            binder_item_id: 7,
            ..Default::default()
        };
        let occurrence = SearchNode {
            is_item: false,
            binder_item_id: 7,
            result_id: 7,
            char_start: 7,
            ..Default::default()
        };
        assert_eq!(item.key(), SearchTreeKey::Item(7));
        assert_eq!(occurrence.key(), SearchTreeKey::Occurrence(7, 7));
        assert_ne!(item.key(), occurrence.key());
    }

    /// Two occurrences of one row are told apart by where they start, and two rows
    /// by which row they are — so a hit at the same offset in two different fields
    /// of one scene is two keys, not one.
    #[test]
    fn occurrences_key_by_row_and_offset() {
        assert_ne!(
            SearchTreeKey::Occurrence(1, 40),
            SearchTreeKey::Occurrence(1, 91)
        );
        assert_ne!(
            SearchTreeKey::Occurrence(1, 40),
            SearchTreeKey::Occurrence(2, 40)
        );
    }
}
