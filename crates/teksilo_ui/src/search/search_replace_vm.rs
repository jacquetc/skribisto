// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SearchReplaceViewModel` — the single shared handle behind both search docks:
//! the leading dock (query, options, scopes, facet chips, result list) and the
//! bottom dock (the editable preview of the selected match).
//!
//! Single-instance live state, created once in `App::build` and shared by **both**
//! docks by `.clone()` — the model keys everything by id, so one handle drives the
//! two views. It owns:
//!
//!   * the input **signals** (query, replacement, the matching toggles, the field
//!     scopes, the six facet chips) — the widgets bind straight to these;
//!   * the [`SearchResultsModel`] (the reactive result list) and the persisted
//!     [`SearchSettingsService`] (toggles/scopes/facets + query history, per
//!     project);
//!   * the shared [`OpenDocsStore`] — so the preview edits the **same**
//!     `Rc<OpenDoc>` an editor tab does (one document, two views), refcounted
//!     symmetrically on select/deselect;
//!   * the [`DockingModel`] (by clone) + the preview dock id, so activating a
//!     result reveals the bottom band (which has no rail to reopen it by hand).
//!
//! The business API is plain methods; the debounce timer is framework plumbing
//! wired in the dock's `build` via [`wire`](SearchReplaceViewModel::wire) (it
//! needs a `BuildContext`), but *what* a search is stays here.

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use std::time::{Duration, Instant};

use teksilo::prelude::*;
use teksilo::widgets::{DockWidgetId, DockingModel};

use frontend::AppContext;
use frontend::commands::{search_management_commands, undo_redo_commands, work_commands};
use frontend::common::entities::MatchField;
use frontend::direct_access::SearchResultDto;
use frontend::search_management::{ReplaceInProjectDto, ReplaceInProjectResultDto, RunSearchDto};

use skribisto_model::SearchFacet;

/// What one dismissal removed, so it can be given back.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Dismissal {
    /// A whole item: the result rows it actually took out.
    Rows(Vec<u64>),
    /// One occurrence, by the row it is in and where it starts.
    Occurrence(u64, i64),
}

use crate::models::SearchTreeModel;

use crate::app_ids::{AppIds, HasWorkId};
use crate::models::{
    OpenDoc, OpenDocsStore, SearchPrefs, SearchResultsModel, SearchSettingsService,
};

/// How long the query/options must be quiet before a search runs — search runs
/// synchronously on the UI thread, so a burst of keystrokes must collapse into
/// one scan, not one per character.
const DEBOUNCE: Duration = Duration::from_millis(300);

#[derive(Clone)]
pub struct SearchReplaceViewModel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    results: SearchResultsModel,
    settings: SearchSettingsService,
    docs: OpenDocsStore,
    docking: DockingModel,
    /// The bottom preview band — revealed when a result is activated.
    preview_dock_id: DockWidgetId,
    /// The leading search dock — revealed by the preview's empty-state button and
    /// the View ▸ Search menu.
    search_dock_id: DockWidgetId,

    // ── inputs ──
    query: Signal<String>,
    replacement: Signal<String>,
    case_sensitive: Signal<bool>,
    whole_word: Signal<bool>,
    diacritic_sensitive: Signal<bool>,
    preserve_case: Signal<bool>,
    search_body: Signal<bool>,
    search_titles: Signal<bool>,
    search_synopsis: Signal<bool>,
    search_labels: Signal<bool>,
    /// Comment threads and their replies — see `RunSearchDto.search_comments`.
    search_comments: Signal<bool>,
    include_trashed: Signal<bool>,
    /// One two-way `Signal<bool>` per facet chip, indexed by the chip's position
    /// in [`SearchFacet::ALL`]. Individual signals (not one `HashSet`) because the
    /// chips are `ToolbarAction::toggle`s, which each bind a `Signal<bool>` and
    /// write it directly on click.
    facets: [Signal<bool>; FACET_COUNT],

    // ── outcome ──
    match_count: Signal<u64>,
    item_count: Signal<u64>,
    truncated: Signal<bool>,
    /// A non-empty query has produced a (possibly empty) result set — drives the
    /// "no matches" empty-state vs the pristine "type to search" state.
    ran: Signal<bool>,
    /// A search *failure* (not "zero matches") — shown inline, never as a per-
    /// keystroke toast.
    error: Signal<Option<String>>,

    // ── selection + preview ──
    selected_result: Signal<Option<u64>>,
    /// Which **occurrence** the writer is standing on, as `(result row, offset)`.
    ///
    /// Not the same question as [`selected_result`](Self::selected_result), and a
    /// row cannot be highlighted from that one: a result row is a whole *field*, so
    /// every occurrence inside it carries the same id. Keying the highlight on it
    /// lit up all forty hits of a scene when the writer clicked one.
    selected_occurrence: Signal<Option<(u64, i64)>>,
    preview: Signal<Option<Rc<OpenDoc>>>,
    preview_field: Signal<Option<MatchField>>,
    /// The item id whose doc the preview currently holds a store ref for — so
    /// `release` is exactly symmetric with `open` (the store is refcounted).
    preview_open_id: Rc<RefCell<Option<u64>>>,

    // ── replace review ──
    /// `SearchResult` ids the writer unticked — the fields Replace All will skip.
    excluded: Signal<HashSet<u64>>,
    /// The results as a tree. Built here rather than in the dock because the dock
    /// is a switchable tab whose content is torn down and rebuilt, and a tree that
    /// forgot which items were open every time the writer looked away would be
    /// worse than no tree.
    tree: SearchTreeModel,
    /// What each dismissal took away, newest last, so one can be given back.
    ///
    /// VS Code's dismiss cannot be undone, and its issue tracker has carried the
    /// request since 2019. A dismissal here is a *destructive-looking* gesture on a
    /// list a writer is about to replace across, so it gets the same courtesy every
    /// other destructive gesture in this application gets.
    dismissals: Signal<Vec<Dismissal>>,
    /// Single occurrences the writer unticked, as `(result row, char offset)`.
    ///
    /// Separate from [`excluded`](Self::excluded) rather than folded into it,
    /// because the two mean different things and only one of them can be complete.
    /// Unticking a row says "not this field at all", and stays true however many
    /// hits it holds — including the ones past whatever the tree lists. Unticking
    /// an occurrence can only ever speak for an occurrence somebody has seen.
    excluded_occurrences: Signal<HashSet<(u64, i64)>>,
    /// Whether the replace row is disclosed. UI state, but on the shared VM so the
    /// `Ctrl+Shift+H` intent (reveal the dock *and* open replace) can drive it.
    show_replace: Signal<bool>,

    // ── history suggestions (loaded per project) ──
    query_suggestions: Signal<Vec<String>>,
    replacement_suggestions: Signal<Vec<String>>,

    // ── debounce plumbing ──
    deadline: Rc<Cell<Option<Instant>>>,
    /// The prefs last written to `search.toml`, so a search run only re-persists
    /// when a toggle/scope/facet actually changed (not on every debounce tick).
    last_persisted: Rc<RefCell<Option<SearchPrefs>>>,
}

// The signal accessors + toggle setters are the feature's public API (bound by
// the two docks, fired from tests); wired incrementally, so not every one has a
// caller yet — same convention as `crate::binder::outline_vm`.
#[allow(dead_code)]
impl SearchReplaceViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        results: SearchResultsModel,
        settings: SearchSettingsService,
        docs: OpenDocsStore,
        docking: DockingModel,
        preview_dock_id: DockWidgetId,
        search_dock_id: DockWidgetId,
    ) -> Self {
        // Seed the inputs from the general (no-project-yet) preferences; a project
        // load re-seeds them from that project's saved override via `restore_for_project`.
        let p = settings.general();
        let app_ctx_for_tree = app_ctx.clone();
        let results_for_tree = results.clone();
        let work_id_for_tree = ids.work_id.clone();
        Self {
            app_ctx,
            ids,
            results,
            settings,
            docs,
            docking,
            preview_dock_id,
            search_dock_id,
            query: Signal::new(String::new()),
            replacement: Signal::new(String::new()),
            case_sensitive: Signal::new(p.case_sensitive),
            whole_word: Signal::new(p.whole_word),
            diacritic_sensitive: Signal::new(p.diacritic_sensitive),
            preserve_case: Signal::new(true),
            search_body: Signal::new(p.search_body),
            search_titles: Signal::new(p.search_titles),
            search_synopsis: Signal::new(p.search_synopsis),
            search_labels: Signal::new(p.search_labels),
            search_comments: Signal::new(p.search_comments),
            include_trashed: Signal::new(p.include_trashed),
            facets: facet_signals_from_codes(&p.facets),
            match_count: Signal::new(0),
            item_count: Signal::new(0),
            truncated: Signal::new(false),
            ran: Signal::new(false),
            error: Signal::new(None),
            selected_result: Signal::new(None),
            selected_occurrence: Signal::new(None),
            preview: Signal::new(None),
            preview_field: Signal::new(None),
            preview_open_id: Rc::new(RefCell::new(None)),
            excluded: Signal::new(HashSet::new()),
            tree: SearchTreeModel::new(app_ctx_for_tree, results_for_tree, work_id_for_tree),
            excluded_occurrences: Signal::new(HashSet::new()),
            dismissals: Signal::new(Vec::new()),
            show_replace: Signal::new(false),
            query_suggestions: Signal::new(Vec::new()),
            replacement_suggestions: Signal::new(Vec::new()),
            deadline: Rc::new(Cell::new(None)),
            last_persisted: Rc::new(RefCell::new(None)),
        }
    }

    // ── view handles (signals the widgets bind to) ──────────────────────────
    /// The backend this search runs against. For a surface that needs to read the
    /// project alongside the results — the preview band's margin lane resolves an
    /// item's language and house quote style through it.
    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.app_ctx.clone()
    }

    /// The app's entity ids, for the same reason as [`app_ctx`](Self::app_ctx).
    pub fn ids(&self) -> &AppIds {
        &self.ids
    }

    /// Tell the margin lane what this search is looking for.
    ///
    /// The other half of the arbiter the per-editor find banner writes to. Project
    /// search publishes **no current match**, deliberately: it found hits in forty
    /// scenes and the writer is standing on none of them, so marking one would be
    /// inventing a position.
    ///
    /// Called from the preview band, which is where this search's query and its
    /// three matching switches are already watched — and where the search is being
    /// *used*, which is what "last one used wins" has to mean.
    pub fn publish_to_lane(&self) {
        use crate::margin_lane::{LaneQuery, LaneQuerySource};
        let text = self.query_signal().get();
        if text.is_empty() {
            crate::margin_lane::clear_active_query_from(LaneQuerySource::Project);
            return;
        }
        crate::margin_lane::set_active_query(Some(LaneQuery {
            text,
            case_sensitive: self.case_sensitive_signal().get(),
            whole_word: self.whole_word_signal().get(),
            diacritic_sensitive: self.diacritic_sensitive_signal().get(),
            source: LaneQuerySource::Project,
            current: None,
        }));
    }

    pub fn query_signal(&self) -> Signal<String> {
        self.query.clone()
    }
    pub fn replacement_signal(&self) -> Signal<String> {
        self.replacement.clone()
    }
    pub fn case_sensitive_signal(&self) -> Signal<bool> {
        self.case_sensitive.clone()
    }
    pub fn whole_word_signal(&self) -> Signal<bool> {
        self.whole_word.clone()
    }
    pub fn diacritic_sensitive_signal(&self) -> Signal<bool> {
        self.diacritic_sensitive.clone()
    }
    pub fn preserve_case_signal(&self) -> Signal<bool> {
        self.preserve_case.clone()
    }
    pub fn search_body_signal(&self) -> Signal<bool> {
        self.search_body.clone()
    }
    pub fn search_titles_signal(&self) -> Signal<bool> {
        self.search_titles.clone()
    }
    pub fn search_synopsis_signal(&self) -> Signal<bool> {
        self.search_synopsis.clone()
    }
    pub fn search_labels_signal(&self) -> Signal<bool> {
        self.search_labels.clone()
    }

    pub fn search_comments_signal(&self) -> Signal<bool> {
        self.search_comments.clone()
    }
    pub fn include_trashed_signal(&self) -> Signal<bool> {
        self.include_trashed.clone()
    }
    /// The two-way toggle signal for facet chip `f` — bind it to a
    /// `ToolbarAction::toggle`.
    pub fn facet_signal(&self, f: SearchFacet) -> Signal<bool> {
        self.facets[facet_index(f)].clone()
    }
    pub fn match_count_signal(&self) -> Signal<u64> {
        self.match_count.clone()
    }
    pub fn item_count_signal(&self) -> Signal<u64> {
        self.item_count.clone()
    }
    pub fn truncated_signal(&self) -> Signal<bool> {
        self.truncated.clone()
    }
    pub fn ran_signal(&self) -> Signal<bool> {
        self.ran.clone()
    }
    pub fn error_signal(&self) -> Signal<Option<String>> {
        self.error.clone()
    }
    pub fn selected_result_signal(&self) -> Signal<Option<u64>> {
        self.selected_result.clone()
    }
    pub fn preview_signal(&self) -> Signal<Option<Rc<OpenDoc>>> {
        self.preview.clone()
    }
    pub fn preview_field_signal(&self) -> Signal<Option<MatchField>> {
        self.preview_field.clone()
    }
    pub fn excluded_signal(&self) -> Signal<HashSet<u64>> {
        self.excluded.clone()
    }
    pub fn show_replace_signal(&self) -> Signal<bool> {
        self.show_replace.clone()
    }
    /// Disclose (or hide) the replace row — the `Ctrl+Shift+H` intent opens it.
    pub fn set_show_replace(&self, v: bool) {
        self.show_replace.set(v);
    }
    pub fn query_suggestions_signal(&self) -> Signal<Vec<String>> {
        self.query_suggestions.clone()
    }
    pub fn replacement_suggestions_signal(&self) -> Signal<Vec<String>> {
        self.replacement_suggestions.clone()
    }
    pub fn results(&self) -> SearchResultsModel {
        self.results.clone()
    }
    /// The language of one previewed item's prose, for the caret band's sentence scope — read
    /// through the same shared store, and so the same answer, the spell dictionaries get.
    pub fn preview_locale(&self, item_id: u64) -> Option<String> {
        self.docs.effective_language(item_id).first().cloned()
    }

    pub fn preview_dock_id(&self) -> DockWidgetId {
        self.preview_dock_id
    }
    /// Reveal (switch to) the leading search & replace dock — the preview's
    /// empty-state "Search" button and the View ▸ Search menu drive this.
    pub fn reveal_search(&self) {
        self.docking.reveal_dock(self.search_dock_id);
    }
    /// Reveal the Footnotes dock — the destination for a search result whose match
    /// is a footnote's body (see `select_result`): that text is not any open
    /// document's own field, so the preview band cannot show it, and the dock is
    /// where it is both shown and edited. `footnotes.show` (`app/commands/view.rs`)
    /// is the same door reached from the View menu and from a saved desk that lost
    /// the dock — reused here (via the well-known `FOOTNOTES_DOCK_ID`, not a second
    /// wiring path) rather than inventing another way in.
    pub fn reveal_footnotes(&self) {
        self.docking
            .reveal_dock(DockWidgetId::from_raw(crate::docks::FOOTNOTES_DOCK_ID));
    }
    /// The `search.toml` service's `Reloadable` hook, for registering with the
    /// app's shared `SettingsRegistry` (live cross-process reload).
    pub fn settings_reloadable(&self) -> std::rc::Rc<dyn teksilo::settings::Reloadable> {
        self.settings.as_reloadable()
    }

    // ── toggle setters ──────────────────────────────────────────────────────
    // Plain signal writes. Each re-arms the debounce (via the effect wired on the
    // signal in `wire`), and the settled search persists the whole preference set
    // if it changed (see `persist_prefs_if_changed`). The dock's checkboxes bind
    // the signals directly and write them the same way; these methods are the
    // programmatic/tested entry point. `preserve_case` is a replace option, not a
    // search input, so it neither re-runs nor persists.
    pub fn set_case_sensitive(&self, v: bool) {
        self.case_sensitive.set(v);
    }
    pub fn set_whole_word(&self, v: bool) {
        self.whole_word.set(v);
    }
    pub fn set_diacritic_sensitive(&self, v: bool) {
        self.diacritic_sensitive.set(v);
    }
    pub fn set_preserve_case(&self, v: bool) {
        self.preserve_case.set(v);
    }
    pub fn set_search_body(&self, v: bool) {
        self.search_body.set(v);
    }
    pub fn set_search_titles(&self, v: bool) {
        self.search_titles.set(v);
    }
    pub fn set_search_synopsis(&self, v: bool) {
        self.search_synopsis.set(v);
    }
    pub fn set_search_comments(&self, v: bool) {
        self.search_comments.set(v);
    }
    pub fn set_search_labels(&self, v: bool) {
        self.search_labels.set(v);
    }
    pub fn set_include_trashed(&self, v: bool) {
        self.include_trashed.set(v);
    }

    /// Flip facet chip `f`. Nothing ticked ⇒ no filter (all kinds), which is what
    /// a filter with nothing ticked means to a reader (see the backend's
    /// `wanted_facets`).
    pub fn toggle_facet(&self, f: SearchFacet) {
        let sig = &self.facets[facet_index(f)];
        sig.set(!sig.get());
    }

    pub fn is_facet_on(&self, f: SearchFacet) -> bool {
        self.facets[facet_index(f)].get()
    }

    // ── the search ──────────────────────────────────────────────────────────

    /// Build the backend DTO from the current input signals. Pure: no I/O, so it
    /// is the seam the headless tests assert the toggles/scopes/facets reach.
    pub fn build_dto(&self) -> RunSearchDto {
        RunSearchDto {
            // No open project ⇒ 0, a value no real Work ever has (ids start at 1)
            // — callers that actually run a search (`run_now`) guard on
            // `self.ids.work_id` separately and never send this placeholder to
            // the backend. Kept out of this pure/no-I/O builder so its existing
            // "no AppIds seeded" unit tests are unaffected.
            work_id: self.ids.work_id.get().unwrap_or(0),
            query: self.query.get(),
            case_sensitive: self.case_sensitive.get(),
            whole_word: self.whole_word.get(),
            diacritic_sensitive: self.diacritic_sensitive.get(),
            facets: self.facet_codes(),
            search_body: self.search_body.get(),
            search_titles: self.search_titles.get(),
            search_synopsis: self.search_synopsis.get(),
            search_labels: self.search_labels.get(),
            search_comments: self.search_comments.get(),
            include_trashed: self.include_trashed.get(),
        }
    }

    /// The matching options as a text-document [`teksilo::text_document::FindOptions`], for the bottom
    /// preview's find-highlight session (so the previewed paragraph highlights the
    /// same matches the result list found).
    pub fn find_options(&self) -> teksilo::text_document::FindOptions {
        teksilo::text_document::FindOptions {
            case_sensitive: self.case_sensitive.get(),
            whole_word: self.whole_word.get(),
            diacritic_sensitive: self.diacritic_sensitive.get(),
            ..Default::default()
        }
    }

    /// The ticked facets as backend/`search.toml` codes, in `SearchFacet::ALL`
    /// order (stable, so the persisted `search.toml` does not churn on toggles).
    pub fn facet_codes(&self) -> Vec<i64> {
        SearchFacet::ALL
            .iter()
            .filter(|&&f| self.facets[facet_index(f)].get())
            .map(|f| f.code() as i64)
            .collect()
    }

    /// Run the search **now** (called by the debounce once typing settles). Reads
    /// the store's `SearchResultsModel` fresh via the published `RunSearch` event,
    /// so this only fires the scan and records the summary counts.
    pub fn run_now(&self) {
        // No open project → nothing to search (also keeps the mock build, which
        // never seeds a real Work, inert).
        if self.ids.work_id.get().is_none() {
            return;
        }
        let dto = self.build_dto();
        let has_query = !dto.query.trim().is_empty();
        // A new result set invalidates the old per-field exclusions (the rows are
        // recreated with fresh ids on every search).
        self.excluded.set(HashSet::new());
        match search_management_commands::run_search(&self.app_ctx, &dto) {
            Ok(r) => {
                self.match_count.set(r.match_count);
                self.item_count.set(r.item_count);
                self.truncated.set(r.truncated);
                self.ran.set(has_query);
                self.error.set(None);
                self.exclude_comment_rows();
            }
            Err(e) => {
                self.match_count.set(0);
                self.item_count.set(0);
                self.truncated.set(false);
                self.ran.set(has_query);
                self.error.set(Some(e.to_string()));
            }
        }
        // Reload the result list from the store the search just wrote. Driven
        // directly (not via a backend-event subscription) because the search dock
        // is a switchable tab whose content — and any subscription registered in
        // it — is torn down when the writer flips to the binder tab; see
        // `SearchResultsModel::wire`.
        self.results.reload();
        // Nothing is standing on an occurrence of a result set that no longer
        // exists.
        self.selected_occurrence.set(None);
        // And the tree over them. Everything it had fetched is dropped with it: a
        // search re-mints every result row, so occurrences kept against the old ids
        // would answer a new query with an old query's hits.
        self.tree.reload();
        // A toggle/scope/facet change arms this same debounce, so a settled search
        // is the moment to persist the preferences — but only if they actually
        // changed, so ordinary query typing does not rewrite `search.toml`.
        self.persist_prefs_if_changed();
    }

    /// Record the current query in the open project's history (dedup, MRU) and
    /// refresh the suggestion list. Called on submit (Enter) — not per keystroke.
    pub fn commit_query(&self) {
        let (uid, path, _title) = self.project_ident();
        let q = self.query.get();
        let _ = self.settings.push_query(&uid, &path, &q);
        self.reload_suggestions(&uid);
    }

    // ── selection + preview ─────────────────────────────────────────────────

    /// Select result `result_id`: open its item's shared document into the preview
    /// (releasing whatever the preview held before — the store is refcounted), and
    /// reveal the bottom band (which has no rail to reopen it by hand).
    pub fn select_result(&self, result_id: u64) {
        let Some(row) = self.row_by_id(result_id) else {
            return;
        };
        self.apply_selected_row(result_id, &row);
    }

    /// The decision behind [`select_result`](Self::select_result), split out so it
    /// can be exercised with a hand-built row — standing up a real `SearchResult`
    /// entity (a whole open Work, a run search) just to reach this dispatch would
    /// dwarf what it is actually testing.
    fn apply_selected_row(&self, result_id: u64, row: &SearchResultDto) {
        // A footnote's body is not any open document's own field (see
        // `search::preview_dock::editable_field`) — it lives on the `Footnote`
        // entity itself, shown and edited only in the Footnotes dock. Opening
        // `row.binder_item_id`'s document would be actively wrong here, in two
        // different ways depending on the note:
        //   * **Anchored** (`binder_item_id` names the scene the citation sits in):
        //     the match is inside the note's *body*, which that scene's document
        //     never contains — the preview would open, find nothing to show, and
        //     dead-end on "no editable text" with no path to the actual note.
        //   * **Orphaned** (`binder_item_id == 0` — a real, documented state; see
        //     `work_management::load_work_uc`'s own note on why an unanchored note
        //     keeps its text and reports itself this way): `self.docs.open(0)` names
        //     no `BinderItem` at all and returns `None`, and the preview would fall
        //     back to its pristine "type to search" empty state — as if nothing had
        //     been found, when a match plainly had been.
        // Both dead ends are replaced with the one door that actually leads to the
        // match: `preview` stays `None` (so `PreviewBody` renders the footnote-aware
        // empty state instead of trying to resolve an editable field), and the
        // Footnotes dock is revealed directly.
        if row.match_field == MatchField::Footnote {
            if let Some(old) = self.preview_open_id.borrow_mut().take() {
                self.docs.release(old, self.ids.stack_id.get());
            }
            self.preview.set(None);
            self.preview_field.set(Some(MatchField::Footnote));
            self.selected_result.set(Some(result_id));
            self.reveal_footnotes();
            self.docking.reveal_dock(self.preview_dock_id);
            return;
        }
        let item_id = row.binder_item_id;
        let prev = self.preview_open_id.borrow().clone();
        if prev != Some(item_id) {
            // Release the previous preview ref before taking the new one, so a doc
            // still live in a tab is neither evicted early nor pinned forever.
            if let Some(old) = prev {
                self.docs.release(old, self.ids.stack_id.get());
            }
            let doc = self.docs.open(item_id);
            *self.preview_open_id.borrow_mut() = doc.as_ref().map(|_| item_id);
            self.preview.set(doc);
        }
        self.preview_field.set(Some(row.match_field.clone()));
        self.selected_result.set(Some(result_id));
        // Revealing the preview does NOT steal focus (`reveal_dock` contains no
        // `request_focus`), so arrow-key navigation of the result list stays intact.
        self.docking.reveal_dock(self.preview_dock_id);
    }

    /// Drop the preview's document ref and clear its signals — on project
    /// close/switch, so the store isn't left pinning a doc from a project that is
    /// no longer open.
    pub fn clear_preview(&self) {
        if let Some(old) = self.preview_open_id.borrow_mut().take() {
            self.docs.release(old, self.ids.stack_id.get());
        }
        self.preview.set(None);
        self.preview_field.set(None);
        self.selected_result.set(None);
    }

    // ── replace review ──────────────────────────────────────────────────────

    pub fn is_excluded(&self, result_id: u64) -> bool {
        self.excluded.get().contains(&result_id)
    }

    /// Tick/untick result `result_id` for Replace All.
    pub fn toggle_excluded(&self, result_id: u64) {
        self.set_excluded(result_id, !self.is_excluded(result_id));
    }

    /// Set whether result `result_id` is excluded from Replace All.
    pub fn set_excluded(&self, result_id: u64, excluded: bool) {
        let mut set = self.excluded.get();
        let changed = if excluded {
            set.insert(result_id)
        } else {
            set.remove(&result_id)
        };
        if changed {
            self.excluded.set(set);
            self.sync_dismissed();
        }
    }

    /// The results as a tree: an item, and the occurrences inside it.
    pub fn tree(&self) -> SearchTreeModel {
        self.tree.clone()
    }

    /// Which occurrence is highlighted, for the tree's rows.
    pub fn selected_occurrence_signal(&self) -> Signal<Option<(u64, i64)>> {
        self.selected_occurrence.clone()
    }

    /// Select whatever the tree row at `flat_index` points at.
    ///
    /// An occurrence selects the field it is in, which is what fills the preview.
    /// An **item** selects nothing: its chevron is what it is for, and quietly
    /// choosing the first of its fields would send a writer somewhere they did not
    /// point at.
    pub fn activate_tree_row(&self, flat_index: usize) {
        let Some(node) = self
            .tree
            .slice()
            .with_entry(flat_index, |node, _| node.clone())
        else {
            return;
        };
        if !node.is_item {
            self.selected_occurrence
                .set(Some((node.result_id, node.char_start)));
            self.select_result(node.result_id);
        }
    }

    /// Hand the tree the current dismissals, so the rows leave it.
    ///
    /// Pushed rather than pulled: the tree rebuilds from a closure that must not
    /// reach back into the view model, and a dismissal is rare where a rebuild is
    /// not.
    fn sync_dismissed(&self) {
        self.tree
            .set_dismissed(self.excluded.get(), self.excluded_occurrences.get());
    }

    // ── dismissing ──────────────────────────────────────────────────────────
    //
    // A dismissal is an exclusion that also leaves the list: the row goes, and
    // Replace All stops counting it. That is what the panel this is modelled on
    // does, and it is why the two are one gesture here rather than a tick-box and
    // a separate filter.

    /// Take one whole item out of the results.
    ///
    /// Records only the rows it *actually* removed, so giving it back cannot
    /// resurrect a row the writer had already dismissed on its own.
    pub fn dismiss_item(&self, binder_item_id: u64) {
        let already = self.excluded.get();
        let mut removed: Vec<u64> = Vec::new();
        self.results.for_each(|r| {
            if r.binder_item_id == binder_item_id && !already.contains(&r.id) {
                removed.push(r.id);
            }
        });
        if removed.is_empty() {
            return;
        }
        for id in &removed {
            self.set_excluded(*id, true);
        }
        self.push_dismissal(Dismissal::Rows(removed));
    }

    /// Take one occurrence out of the results.
    pub fn dismiss_occurrence(&self, result_id: u64, char_start: i64) {
        if self.is_occurrence_excluded(result_id, char_start) {
            return;
        }
        self.set_excluded_occurrence(result_id, char_start, true);
        self.push_dismissal(Dismissal::Occurrence(result_id, char_start));
    }

    fn push_dismissal(&self, what: Dismissal) {
        let mut stack = self.dismissals.get();
        stack.push(what);
        self.dismissals.set(stack);
    }

    /// Give back whatever the last dismissal took.
    pub fn undo_last_dismiss(&self) {
        let mut stack = self.dismissals.get();
        let Some(last) = stack.pop() else {
            return;
        };
        self.dismissals.set(stack);
        match last {
            Dismissal::Rows(ids) => {
                for id in ids {
                    self.set_excluded(id, false);
                }
            }
            Dismissal::Occurrence(row, at) => {
                self.set_excluded_occurrence(row, at, false);
            }
        }
    }

    /// Whether anything has been dismissed that could be given back.
    pub fn can_undo_dismiss_signal(&self) -> Signal<bool> {
        self.dismissals.map(|s| !s.is_empty())
    }

    /// Which result rows a scoped replace is about to rewrite, and which items
    /// they belong to.
    ///
    /// Read **before** the replace, because the replace is what makes some of them
    /// stop existing.
    pub fn scope_of(&self, target_rows: &[u64], target_items: &[u64]) -> (Vec<u64>, Vec<u64>) {
        let mut rows: Vec<u64> = target_rows.to_vec();
        let mut items: Vec<u64> = target_items.to_vec();
        self.results.for_each(|r| {
            if target_items.contains(&r.binder_item_id) && !rows.contains(&r.id) {
                rows.push(r.id);
            }
            if target_rows.contains(&r.id) && !items.contains(&r.binder_item_id) {
                items.push(r.binder_item_id);
            }
        });
        (rows, items)
    }

    /// Settle the panel after a replace that named its own scope.
    ///
    /// **Not a re-search.** A re-search is what Replace All does, and it is right
    /// there: everything moved, so everything is read again. Here one hit of one
    /// field was rewritten, and re-running the scan throws away the writer's whole
    /// position -- which rows they had open, where they had scrolled to, what they
    /// had dismissed -- to re-derive a result set that differs from the one on
    /// screen in exactly the rows named here. The backend restates those rows
    /// itself, so this reads them back and leaves the rest of the tree alone.
    ///
    /// The occurrence-level dismissals *inside* the rewritten rows do not survive,
    /// and cannot: they are keyed by character offset, and replacing a hit shifts
    /// every offset after it in the field. Keeping them would keep a set of numbers
    /// that now name different hits -- the row would come back with the wrong ones
    /// missing. Dismissals of other rows, and of whole rows, are untouched.
    pub fn settle_after_scoped_replace(&self, rows: &[u64], items: &[u64]) {
        // An open editor of a scene that was just rewritten is showing the old
        // prose until this. Replace All does it through `reload_and_rescan`; a row
        // replace reached the store and never told the editor, so the writer had to
        // close the tab and reopen it to see their own replacement.
        self.reload_touched(items);

        let mut occurrences = self.excluded_occurrences.get();
        occurrences.retain(|(row, _)| !rows.contains(row));
        self.excluded_occurrences.set(occurrences);

        self.results.reload();
        self.recount();
        self.sync_dismissed();
        self.tree.refresh_rows(items, rows);

        // Nothing is standing on an occurrence whose offset the replace just moved.
        if let Some((row, _)) = self.selected_occurrence.get()
            && rows.contains(&row)
        {
            self.selected_occurrence.set(None);
        }
    }

    /// Re-derive the summary counts from the result set as it now stands.
    ///
    /// The search itself reports them, which is right when a search has just run.
    /// After a scoped replace no search ran, and the rows are the only record of
    /// what is left -- so they are counted, the same two ways the scan counted
    /// them: every hit of every field, over however many items those fields are in.
    fn recount(&self) {
        let mut matches = 0u64;
        let mut items: HashSet<u64> = HashSet::new();
        self.results.for_each(|r| {
            matches += r.occurrence_count;
            items.insert(r.binder_item_id);
        });
        self.match_count.set(matches);
        self.item_count.set(items.len() as u64);
    }

    /// Replace **only** the occurrences of one item, leaving the rest alone.
    ///
    /// Expressed as a Replace All over everything *else* excluded, because that is
    /// what the backend takes and because routing every replacement through one
    /// call is what keeps a single-row replace undoable in the same way as a
    /// project-wide one -- one Ctrl+Z, whichever the writer used.
    pub fn replace_item(&self, binder_item_id: u64) -> anyhow::Result<ReplaceInProjectResultDto> {
        let keep: Vec<u64> = {
            let mut out = Vec::new();
            self.results.for_each(|r| {
                if r.binder_item_id == binder_item_id {
                    out.push(r.id);
                }
            });
            out
        };
        self.replace_only(&keep, &[])
    }

    /// Replace one occurrence and nothing else.
    ///
    /// Sent as the one occurrence to replace, **not** as an exclusion of the others.
    /// Excluding the others is only truthful while they can all be named, and they
    /// cannot: the tree lists at most a few hundred hits of a field that may hold
    /// thousands, so in a long scene "all the others" was a list missing everything
    /// past the cap -- and every one of those got replaced. That is what "it
    /// replaced far more than the one I clicked" was.
    pub fn replace_occurrence(
        &self,
        result_id: u64,
        char_start: i64,
    ) -> anyhow::Result<ReplaceInProjectResultDto> {
        self.replace_only(&[result_id], &[(result_id, char_start)])
    }

    /// Run the replace with everything but `rows` excluded, and inside those rows
    /// only the occurrences in `only` (empty: all of them).
    fn replace_only(
        &self,
        rows: &[u64],
        only: &[(u64, i64)],
    ) -> anyhow::Result<ReplaceInProjectResultDto> {
        let keep: HashSet<u64> = rows.iter().copied().collect();
        let mut excluded_rows: Vec<u64> = Vec::new();
        self.results.for_each(|r| {
            if !keep.contains(&r.id) {
                excluded_rows.push(r.id);
            }
        });
        // The writer's own dismissals still stand inside what is being replaced --
        // except in a row that already names what to replace, where they would be
        // saying the same thing a second time and in the weaker of the two ways.
        let named: HashSet<u64> = only.iter().map(|(row, _)| *row).collect();
        let mut occurrence_rows: Vec<u64> = Vec::new();
        let mut occurrence_starts: Vec<i64> = Vec::new();
        for (row, at) in self.excluded_occurrences.get() {
            if keep.contains(&row) && !named.contains(&row) {
                occurrence_rows.push(row);
                occurrence_starts.push(at);
            }
        }
        for id in self.excluded.get() {
            if !excluded_rows.contains(&id) {
                excluded_rows.push(id);
            }
        }
        let (only_rows, only_starts): (Vec<u64>, Vec<i64>) = only.iter().copied().unzip();
        self.run_replace(
            excluded_rows,
            occurrence_rows,
            occurrence_starts,
            only_rows,
            only_starts,
        )
    }

    /// Whether one occurrence of `result_id`, at `char_start`, is ticked out.
    ///
    /// A row that is excluded whole excludes every occurrence in it, so this
    /// answers `true` for all of them without their needing to be listed — which
    /// they may not be, and which is the reason the two sets are kept apart.
    pub fn is_occurrence_excluded(&self, result_id: u64, char_start: i64) -> bool {
        self.excluded.get().contains(&result_id)
            || self
                .excluded_occurrences
                .get()
                .contains(&(result_id, char_start))
    }

    /// Tick/untick one occurrence for Replace All.
    pub fn toggle_excluded_occurrence(&self, result_id: u64, char_start: i64) {
        let excluded = self
            .excluded_occurrences
            .get()
            .contains(&(result_id, char_start));
        self.set_excluded_occurrence(result_id, char_start, !excluded);
    }

    /// Set whether one occurrence is excluded from Replace All.
    ///
    /// Ticking an occurrence back on inside a row that is excluded whole also
    /// clears the row: the writer's last gesture wins, and leaving the row's own
    /// exclusion standing would make the tick they just made do nothing while
    /// showing that it had.
    pub fn set_excluded_occurrence(&self, result_id: u64, char_start: i64, excluded: bool) {
        if !excluded && self.excluded.get().contains(&result_id) {
            self.set_excluded(result_id, false);
        }
        let mut set = self.excluded_occurrences.get();
        let changed = if excluded {
            set.insert((result_id, char_start))
        } else {
            set.remove(&(result_id, char_start))
        };
        if changed {
            self.excluded_occurrences.set(set);
            self.sync_dismissed();
        }
    }

    /// How many of `result_id`'s listed occurrences are ticked out on their own.
    ///
    /// What a parent row's part-ticked state is drawn from, together with the row's
    /// own `occurrence_count`.
    pub fn excluded_occurrence_count(&self, result_id: u64) -> usize {
        self.excluded_occurrences
            .get()
            .iter()
            .filter(|(row, _)| *row == result_id)
            .count()
    }

    /// Whether Replace All may run: a non-empty query produced results, the scan
    /// was **not** truncated (a capped scan cannot honestly claim completeness, so
    /// a "replace all" over it would silently leave matches behind), and at least
    /// one result is still ticked.
    pub fn can_replace_all(&self) -> bool {
        self.ran.get()
            && !self.truncated.get()
            && self.item_count.get() > 0
            && self.included_count() > 0
    }

    /// Untick every comment hit, leaving prose ticked.
    ///
    /// Run after each search, because a fresh result set clears the exclusions and
    /// every row would otherwise arrive included. A comment is the writer's own
    /// record of a decision, and rewriting it changes what that record says — so
    /// renaming a character across the manuscript must be able to fix the notes that
    /// quote the old name, but only when the writer says so. They are listed, they
    /// are tickable, they simply do not go along for the ride.
    ///
    /// Deliberately not enforced in the backend: `replace_in_project` rewrites
    /// exactly the rows it is handed, and a second opinion there would make the
    /// tick box a lie.
    fn exclude_comment_rows(&self) {
        let mut set = self.excluded.get();
        self.results.for_each(|row| {
            if matches!(
                row.match_field,
                MatchField::Comment | MatchField::CommentReply
            ) {
                set.insert(row.id);
            }
        });
        self.excluded.set(set);
    }

    /// How many result rows are ticked (not excluded).
    pub fn included_count(&self) -> usize {
        let excluded = self.excluded.get();
        let mut n = 0usize;
        self.results.for_each(|r| {
            if !excluded.contains(&r.id) {
                n += 1;
            }
        });
        n
    }

    /// The binder-item ids Replace All will touch (ticked results only), deduped —
    /// the set to reload afterwards if any are open in a tab (see [`Self::reload_touched`]).
    pub fn touched_item_ids(&self) -> Vec<u64> {
        let excluded = self.excluded.get();
        let mut ids: Vec<u64> = Vec::new();
        self.results.for_each(|r| {
            if !excluded.contains(&r.id) {
                ids.push(r.binder_item_id);
            }
        });
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    /// Execute Replace All over the ticked results, on the open project's undo
    /// stack. Synchronous (the backend `replace_in_project` command is), returning
    /// its summary for the completion toast. The caller (which has an
    /// `EventContext`) surfaces the toast + its Undo, reloads open docs, and
    /// re-runs the search.
    ///
    /// **Deliberately outside the writing games**, and the one text-removing path
    /// that is. "Always forward" blocks the *reflex* — the keystroke a writer
    /// makes without deciding to — not the intent: this is a project-wide
    /// operation reached through a dock, over a list the writer ticked result by
    /// result, confirmed, reported by a toast and undoable from it in one step.
    /// A game about not fiddling with the sentence you just wrote has nothing to
    /// say about it, and gating it would turn a drafting aid into a project lock.
    /// The **in-editor** find bar is the opposite case and *is* gated — see
    /// `FindViewModel::may_replace`, which sits on the writing surface itself.
    ///
    /// If this is ever revisited, the honest fix is a confirmation naming the
    /// game, never a silent refusal: a Replace All that quietly does nothing is
    /// worse than either answer.
    pub fn replace_all(&self) -> anyhow::Result<ReplaceInProjectResultDto> {
        let work_id = self
            .ids
            .work_id
            .get()
            .ok_or_else(|| anyhow::anyhow!("replace_all: no open project"))?;
        // Occurrences inside a row that is excluded whole are dropped here rather
        // than sent: the row already says so, and sending both would have the
        // backend skip the same hit twice and leave the count of what it did
        // disagreeing with what it did.
        let whole_rows = self.excluded.get();
        let (occurrence_rows, occurrence_starts): (Vec<u64>, Vec<i64>) = self
            .excluded_occurrences
            .get()
            .into_iter()
            .filter(|(row, _)| !whole_rows.contains(row))
            .unzip();
        let _ = work_id;
        // No `only` list: Replace All is defined by what it leaves out.
        self.run_replace(
            self.excluded.get().into_iter().collect(),
            occurrence_rows,
            occurrence_starts,
            Vec::new(),
            Vec::new(),
        )
    }

    /// The one call every replacement goes through, whatever its scope.
    ///
    /// A whole-project Replace All and a single occurrence differ only in what they
    /// exclude, and routing both here is what makes the small one undoable in the
    /// same gesture as the large one -- one Ctrl+Z, and one entry in the
    /// replacement history whichever the writer used.
    fn run_replace(
        &self,
        excluded_result_ids: Vec<u64>,
        excluded_occurrence_rows: Vec<u64>,
        excluded_occurrence_starts: Vec<i64>,
        only_occurrence_rows: Vec<u64>,
        only_occurrence_starts: Vec<i64>,
    ) -> anyhow::Result<ReplaceInProjectResultDto> {
        let work_id = self
            .ids
            .work_id
            .get()
            .ok_or_else(|| anyhow::anyhow!("replace: no open project"))?;
        let dto = ReplaceInProjectDto {
            work_id,
            replacement: self.replacement.get(),
            preserve_case: self.preserve_case.get(),
            excluded_result_ids,
            // Parallel by index, as the DTO's own note records: the manifest's
            // field types are primitives, so a list of pairs is two lists.
            excluded_occurrence_rows,
            excluded_occurrence_starts,
            only_occurrence_rows,
            only_occurrence_starts,
        };
        let result = search_management_commands::replace_in_project(
            &self.app_ctx,
            self.ids.stack_id.get(),
            &dto,
        )?;
        // Record the replacement in history now that it actually ran.
        let (uid, path, _title) = self.project_ident();
        let _ = self
            .settings
            .push_replacement(&uid, &path, &dto.replacement);
        self.reload_suggestions(&uid);
        Ok(result)
    }

    /// Reload the docs among `item_ids` that are open in a tab, so an open editor
    /// of a just-replaced item shows the new text instead of the stale prose.
    pub fn reload_touched(&self, item_ids: &[u64]) {
        self.docs.reload_open(item_ids);
    }

    /// Undo the last Replace All (the toast's action), on the project's undo stack.
    pub fn undo_last_replace(&self) -> anyhow::Result<()> {
        undo_redo_commands::undo(&self.app_ctx, self.ids.stack_id.get())
    }

    // ── project lifecycle ───────────────────────────────────────────────────

    /// Re-seed the inputs from the just-opened project's saved preferences and
    /// load its query/replacement history, and drop any preview held for the
    /// previous project. Call on `LoadWork`/`NewWork`.
    pub fn restore_for_project(&self) {
        self.clear_preview();
        self.excluded.set(HashSet::new());
        self.query.set(String::new());
        self.match_count.set(0);
        self.item_count.set(0);
        self.truncated.set(false);
        self.ran.set(false);
        self.error.set(None);

        let (uid, _path, _title) = self.project_ident();
        let p = self.settings.effective_for(&uid);
        self.case_sensitive.set(p.case_sensitive);
        self.whole_word.set(p.whole_word);
        self.diacritic_sensitive.set(p.diacritic_sensitive);
        self.search_body.set(p.search_body);
        self.search_titles.set(p.search_titles);
        self.search_synopsis.set(p.search_synopsis);
        self.search_labels.set(p.search_labels);
        self.search_comments.set(p.search_comments);
        self.include_trashed.set(p.include_trashed);
        set_facets_from_codes(&self.facets, &p.facets);
        // The restored prefs are, by definition, what is on disk — cache them so
        // the first search after a load doesn't rewrite `search.toml` needlessly.
        *self.last_persisted.borrow_mut() = Some(p);
        // Clear the result list to the new project's (empty) result set — the old
        // project's rows were torn down with its store.
        self.results.reload();
        self.selected_occurrence.set(None);
        self.tree.reload();
        self.reload_suggestions(&uid);
    }

    // ── debounce wiring (framework plumbing; needs a BuildContext) ───────────

    /// Wire the reactive layer the dock's `build` owns: subscribe the result model
    /// (once), and install the debounced re-search — one arm effect per input
    /// signal plus one `frame_tick` that fires [`run_now`](Self::run_now) when the
    /// quiet window elapses. Mirrors the app's autosave timer.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.results.wire(ctx);

        let wake = ctx.wake_at_handle();

        // Any input change arms the deadline `DEBOUNCE` out. The query and every
        // toggle/scope/facet/trashed signal is an input; the replacement is NOT
        // (changing it must not re-run the search).
        let arm = {
            let deadline = self.deadline.clone();
            let wake = wake.clone();
            Rc::new(move || {
                let at = Instant::now() + DEBOUNCE;
                deadline.set(Some(at));
                wake.set(Some(at));
            })
        };
        // One arm effect per input signal, observed at its concrete type —
        // `ctx.effect` calls `Signal::observe`, which panics on a derived
        // (`map`/`zip`) signal, so these must be the concrete inputs, not a mapped
        // `Signal<()>`. The query and every toggle/scope/facet/trashed signal is an
        // input; the replacement is NOT (changing it must not re-run the search).
        macro_rules! arm_on {
            ($sig:expr) => {{
                let arm = arm.clone();
                ctx.effect(&$sig, move |_| arm());
            }};
        }
        arm_on!(self.query);
        arm_on!(self.case_sensitive);
        arm_on!(self.whole_word);
        arm_on!(self.diacritic_sensitive);
        arm_on!(self.search_body);
        arm_on!(self.search_titles);
        arm_on!(self.search_synopsis);
        arm_on!(self.search_labels);
        arm_on!(self.search_comments);
        arm_on!(self.include_trashed);
        for sig in &self.facets {
            arm_on!(sig);
        }

        let me = self.clone();
        let deadline = self.deadline.clone();
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            let Some(at) = deadline.get() else { return };
            if Instant::now() >= at {
                deadline.set(None);
                me.run_now();
            } else {
                wake.set(Some(at));
            }
        });
    }

    // ── internals ───────────────────────────────────────────────────────────

    /// This project's `(unique_id, path, title)` for keying `search.toml`. Empty
    /// `unique_id` (a brand-new unsaved project) makes the settings writes no-ops.
    ///
    /// Resolved through `self.ids.work_id` (the Phase-1 seam), not
    /// `get_all_work(ctx)`'s first entry — see `main::current_project_path`'s
    /// doc for why that stopped being a safe stand-in once the backend scoped
    /// `Work` to support more than one open project.
    fn project_ident(&self) -> (String, String, String) {
        let work = self
            .ids
            .work_id
            .get()
            .and_then(|id| work_commands::get_work(&self.app_ctx, &id).ok().flatten());
        match work {
            Some(w) => (w.unique_id, String::new(), w.title),
            None => (String::new(), String::new(), String::new()),
        }
    }

    /// The current toggles/scopes/facets as a `SearchPrefs`.
    fn current_prefs(&self) -> SearchPrefs {
        SearchPrefs {
            case_sensitive: self.case_sensitive.get(),
            whole_word: self.whole_word.get(),
            diacritic_sensitive: self.diacritic_sensitive.get(),
            search_body: self.search_body.get(),
            search_titles: self.search_titles.get(),
            search_synopsis: self.search_synopsis.get(),
            search_labels: self.search_labels.get(),
            search_comments: self.search_comments.get(),
            include_trashed: self.include_trashed.get(),
            facets: self.facet_codes(),
        }
    }

    /// Persist the current preferences as this project's override, but only if
    /// they differ from what was last written (or restored) — so a search run
    /// triggered by plain query typing does not rewrite `search.toml`.
    fn persist_prefs_if_changed(&self) {
        let prefs = self.current_prefs();
        if self.last_persisted.borrow().as_ref() == Some(&prefs) {
            return;
        }
        let (uid, path, title) = self.project_ident();
        if self
            .settings
            .set_override(&uid, &path, &title, prefs.clone())
            .is_ok()
        {
            *self.last_persisted.borrow_mut() = Some(prefs);
        }
    }

    fn reload_suggestions(&self, uid: &str) {
        self.query_suggestions.set(self.settings.query_history(uid));
        self.replacement_suggestions
            .set(self.settings.replacement_history(uid));
    }

    fn row_by_id(&self, result_id: u64) -> Option<SearchResultDto> {
        self.results.find(|r| r.id == result_id)
    }
}

/// The open Work this search runs over — `replace_flow.rs`'s
/// completion/error toasts route here via [`HasWorkId::work_id`] (see
/// `crate::toast_scope::ToastWorkExt`): a replace-all is squarely this
/// Work's own edit, never every open window's business.
impl HasWorkId for SearchReplaceViewModel {
    fn app_ids(&self) -> &AppIds {
        &self.ids
    }
}

/// The position of facet `f` in [`SearchFacet::ALL`] — the index into the
/// per-facet signal array.
/// How many facet chips there are. Derived, never written out: this array is indexed by
/// `SearchFacet::ALL`'s own order, and a literal here silently went out of bounds the day a
/// facet was added.
const FACET_COUNT: usize = SearchFacet::ALL.len();

fn facet_index(f: SearchFacet) -> usize {
    SearchFacet::ALL.iter().position(|&x| x == f).unwrap_or(0)
}

/// Fresh per-facet toggle signals seeded from a list of codes. A code that names
/// no facet (a `search.toml` written by a version whose codes differed) is
/// ignored — never resurrected as a phantom filter.
fn facet_signals_from_codes(codes: &[i64]) -> [Signal<bool>; FACET_COUNT] {
    let on = codes_to_set(codes);
    std::array::from_fn(|i| Signal::new(on.contains(&SearchFacet::ALL[i])))
}

/// Set existing per-facet signals from a list of codes (used on project restore).
fn set_facets_from_codes(sigs: &[Signal<bool>; FACET_COUNT], codes: &[i64]) {
    let on = codes_to_set(codes);
    for (i, sig) in sigs.iter().enumerate() {
        sig.set(on.contains(&SearchFacet::ALL[i]));
    }
}

/// The set of facets a list of codes names, dropping unknown codes.
fn codes_to_set(codes: &[i64]) -> HashSet<SearchFacet> {
    codes
        .iter()
        .filter_map(|&c| u64::try_from(c).ok().and_then(SearchFacet::from_code))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::widgets::DockingModel;

    fn vm() -> SearchReplaceViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let results = SearchResultsModel::new(app_ctx.clone(), ids.work_info_id.clone());
        let settings = SearchSettingsService::in_memory_default();
        let docs = OpenDocsStore::new(app_ctx.clone());
        let docking = DockingModel::new();
        SearchReplaceViewModel::new(
            app_ctx,
            ids,
            results,
            settings,
            docs,
            docking,
            DockWidgetId::fresh(),
            DockWidgetId::fresh(),
        )
    }

    /// Two fields of one item, one hit and three, so an item's counts are not the
    /// same as any single row's.
    fn seeded() -> SearchReplaceViewModel {
        let vm = vm();
        vm.results().list_model().replace_all(vec![
            SearchResultDto {
                id: 11,
                binder_item_id: 5,
                match_field: MatchField::Body,
                occurrence_count: 3,
                ..Default::default()
            },
            SearchResultDto {
                id: 12,
                binder_item_id: 5,
                match_field: MatchField::Comment,
                occurrence_count: 1,
                ..Default::default()
            },
        ]);
        vm
    }

    /// A row replace names one field; an item replace names every field of one
    /// item. Either way both sets have to be known, and known **before** the
    /// replace: a field that loses its last hit stops existing, and afterwards
    /// nothing left could name it.
    #[test]
    fn the_scope_of_a_row_replace_is_read_in_both_directions() {
        let vm = seeded();

        let (mut rows, items) = vm.scope_of(&[], &[5]);
        rows.sort_unstable();
        assert_eq!(rows, vec![11, 12], "an item is all of its matching fields");
        assert_eq!(items, vec![5]);

        let (rows, items) = vm.scope_of(&[11], &[]);
        assert_eq!(
            rows,
            vec![11],
            "and one field is one field -- replacing a hit in the prose must not \
             invalidate the offsets of the comment row beside it"
        );
        assert_eq!(items, vec![5], "but it does name the item it is in");
    }

    /// **Refreshing after a row replace is not a reload**, and the difference is
    /// the whole complaint: the writer scrolls down, opens an item, replaces one
    /// hit, and finds the tree collapsed back at the top.
    ///
    /// Asserted against `reload`, which resets both on purpose -- a new result set
    /// is a new list and the old position means nothing in it.
    #[test]
    fn refreshing_a_replaced_row_leaves_the_tree_where_the_writer_left_it() {
        use crate::models::SearchTreeKey;
        use teksilo::data::TreeDataSource;

        let vm = seeded();
        let tree = vm.tree();
        tree.reload();
        tree.set_expanded(&SearchTreeKey::Item(5), true);
        tree.scroll().set(240.0);

        tree.refresh_rows(&[5], &[11]);
        assert!(
            tree.is_expanded(&SearchTreeKey::Item(5)),
            "the item the writer had open stays open"
        );
        assert_eq!(
            tree.scroll().get(),
            240.0,
            "and the tree stays where they scrolled it"
        );

        tree.reload();
        assert_eq!(
            tree.scroll().get(),
            0.0,
            "where a genuinely new result set does start at the top"
        );
    }

    /// **The row actually leaves the tree.** The state being right is not the
    /// same claim as the row being gone, and the second is what a writer sees.
    #[test]
    fn a_dismissed_item_leaves_the_tree() {
        let vm = seeded();
        let tree = vm.tree();
        tree.reload();
        assert_eq!(
            tree.slice().visible_count(),
            1,
            "one item row, for the two fields of item 5"
        );

        vm.dismiss_item(5);
        assert_eq!(
            tree.slice().visible_count(),
            0,
            "dismissing the item takes its row out of the tree, not just out of the count"
        );

        vm.undo_last_dismiss();
        assert_eq!(tree.slice().visible_count(), 1, "and undo puts it back");
    }

    /// One field of an item going does not take the item with it: the row stays,
    /// carrying what is left.
    #[test]
    fn dismissing_one_field_leaves_the_item_behind() {
        let vm = seeded();
        let tree = vm.tree();
        tree.reload();

        vm.set_excluded(11, true);
        assert_eq!(
            tree.slice().visible_count(),
            1,
            "item 5 still has its comment field"
        );

        vm.set_excluded(12, true);
        assert_eq!(
            tree.slice().visible_count(),
            0,
            "and goes when the last of its fields does"
        );
    }

    /// **Dismissing an item takes it out of the results**, and one undo puts it
    /// back — which the panel this is modelled on cannot do, and has had an open
    /// request to do since 2019.
    #[test]
    fn a_dismissed_item_comes_back_with_one_undo() {
        let vm = seeded();
        assert!(!vm.can_undo_dismiss_signal().get());

        vm.dismiss_item(5);
        assert!(vm.is_excluded(11) && vm.is_excluded(12));
        assert!(vm.can_undo_dismiss_signal().get());

        vm.undo_last_dismiss();
        assert!(!vm.is_excluded(11) && !vm.is_excluded(12));
        assert!(
            !vm.can_undo_dismiss_signal().get(),
            "and nothing left to give back"
        );
    }

    /// Undoing gives back **only what that dismissal took**. A row the writer had
    /// already dismissed on its own stays dismissed, or one gesture would quietly
    /// undo two.
    #[test]
    fn undo_does_not_resurrect_what_was_already_gone() {
        let vm = seeded();
        vm.dismiss_occurrence(11, 40);
        vm.dismiss_item(5);

        vm.undo_last_dismiss();
        assert!(!vm.is_excluded(11), "the item came back");
        assert!(
            vm.is_occurrence_excluded(11, 40),
            "the occurrence dismissed before it did not"
        );
    }

    /// Dismissals undo in the order they were made, newest first.
    #[test]
    fn dismissals_come_back_newest_first() {
        let vm = seeded();
        vm.dismiss_occurrence(11, 40);
        vm.dismiss_occurrence(11, 91);

        vm.undo_last_dismiss();
        assert!(!vm.is_occurrence_excluded(11, 91));
        assert!(
            vm.is_occurrence_excluded(11, 40),
            "the older one still stands"
        );

        vm.undo_last_dismiss();
        assert!(!vm.is_occurrence_excluded(11, 40));
    }

    /// Dismissing the same thing twice records one dismissal, so one undo is
    /// enough and a second does not silently give back something else.
    #[test]
    fn dismissing_twice_records_once() {
        let vm = seeded();
        vm.dismiss_occurrence(11, 40);
        vm.dismiss_occurrence(11, 40);
        vm.undo_last_dismiss();
        assert!(!vm.is_occurrence_excluded(11, 40));
        assert!(!vm.can_undo_dismiss_signal().get());
    }

    /// A row refused whole already refuses everything in it. Sending its
    /// occurrences as well would have the backend skip the same hit twice and
    /// report having done less than it did.
    #[test]
    fn occurrences_inside_a_wholly_refused_row_are_not_sent_as_well() {
        let vm = seeded();
        vm.set_excluded_occurrence(11, 40, true);
        vm.set_excluded_occurrence(12, 7, true);
        vm.set_excluded(11, true);

        let whole = vm.excluded.get();
        let sent: Vec<(u64, i64)> = vm
            .excluded_occurrences
            .get()
            .into_iter()
            .filter(|(row, _)| !whole.contains(row))
            .collect();
        assert_eq!(
            sent,
            vec![(12, 7)],
            "only the row that is not refused whole"
        );
    }

    /// An occurrence ticked back on inside a row refused whole clears the row too:
    /// the writer's last gesture wins, and leaving the row refused would make the
    /// tick they just made do nothing while showing that it had.
    #[test]
    fn ticking_an_occurrence_back_on_releases_the_row_it_is_in() {
        let vm = seeded();
        vm.set_excluded(11, true);
        assert!(vm.is_occurrence_excluded(11, 40));

        vm.set_excluded_occurrence(11, 40, false);
        assert!(!vm.is_excluded(11));
        assert!(!vm.is_occurrence_excluded(11, 40));
    }

    #[test]
    fn build_dto_carries_every_toggle_scope_and_facet() {
        let vm = vm();
        vm.query.set("Aurélien".into());
        vm.set_case_sensitive(true);
        vm.set_whole_word(false);
        vm.set_diacritic_sensitive(true);
        vm.set_search_body(false);
        vm.set_search_synopsis(false);
        vm.set_include_trashed(true);
        vm.toggle_facet(SearchFacet::Scene);
        vm.toggle_facet(SearchFacet::Note);

        let dto = vm.build_dto();
        assert_eq!(dto.query, "Aurélien");
        assert!(dto.case_sensitive && dto.diacritic_sensitive && dto.include_trashed);
        assert!(!dto.whole_word && !dto.search_body && !dto.search_synopsis);
        assert!(
            dto.search_titles && dto.search_labels,
            "untouched scopes stay on"
        );
        assert_eq!(
            dto.facets,
            vec![
                SearchFacet::Scene.code() as i64,
                SearchFacet::Note.code() as i64
            ],
            "facet codes are sorted for a stable persisted order"
        );
    }

    #[test]
    fn toggling_a_facet_twice_returns_to_no_filter() {
        let vm = vm();
        assert!(vm.facet_codes().is_empty(), "nothing ticked = no filter");
        vm.toggle_facet(SearchFacet::Chapter);
        assert!(vm.is_facet_on(SearchFacet::Chapter));
        assert_eq!(vm.facet_codes(), vec![SearchFacet::Chapter.code() as i64]);
        vm.toggle_facet(SearchFacet::Chapter);
        assert!(!vm.is_facet_on(SearchFacet::Chapter));
        assert!(
            vm.facet_codes().is_empty(),
            "unticking the last chip = no filter again"
        );
    }

    #[test]
    fn facets_survive_a_code_round_trip_dropping_unknowns() {
        // A `search.toml` written by a version with an extra facet code (say 99)
        // must not resurrect a phantom filter — the unknown code is dropped, the
        // known ones survive.
        let set = codes_to_set(&[SearchFacet::Scene.code() as i64, 99, -1]);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&SearchFacet::Scene));
    }

    #[test]
    fn replace_all_is_blocked_when_the_scan_was_truncated() {
        // A capped scan cannot claim completeness, so Replace All must be off even
        // though there are results — else it would silently leave matches behind.
        let vm = vm();
        vm.ran.set(true);
        vm.item_count.set(300);
        vm.truncated.set(false);
        // (mock results model ships 3 rows, all included)
        #[cfg(feature = "mocks")]
        assert!(
            vm.can_replace_all(),
            "a complete scan with results enables it"
        );

        vm.truncated.set(true);
        assert!(
            !vm.can_replace_all(),
            "a truncated scan disables Replace All"
        );
    }

    #[test]
    fn replace_all_is_blocked_with_no_results() {
        let vm = vm();
        vm.ran.set(true);
        vm.item_count.set(0);
        vm.truncated.set(false);
        assert!(!vm.can_replace_all(), "no results, nothing to replace");
    }

    #[cfg(feature = "mocks")]
    #[test]
    fn excluding_every_result_disables_replace_all_and_drops_it_from_touched() {
        // Functional mocks parity: the exclusion set actually gates Replace All and
        // the touched-item set, exercised against the mock result rows.
        let vm = vm();
        vm.ran.set(true);
        vm.item_count.set(3);
        vm.truncated.set(false);
        let rows = vm.results.items();
        assert_eq!(rows.len(), 3, "mock results model ships three rows");
        assert!(vm.can_replace_all());
        assert_eq!(vm.included_count(), 3);

        // Untick the first row: it leaves both the included count and the touched set.
        let first = rows[0].id;
        let first_item = rows[0].binder_item_id;
        vm.toggle_excluded(first);
        assert!(vm.is_excluded(first));
        assert_eq!(vm.included_count(), 2);
        assert!(
            !vm.touched_item_ids().contains(&first_item),
            "an unticked result's item is not touched by Replace All"
        );

        // Untick the remaining two: Replace All goes off.
        vm.toggle_excluded(rows[1].id);
        vm.toggle_excluded(rows[2].id);
        assert_eq!(vm.included_count(), 0);
        assert!(!vm.can_replace_all(), "nothing ticked, nothing to replace");
    }

    fn footnote_row(id: u64, binder_item_id: u64) -> SearchResultDto {
        SearchResultDto {
            id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            binder_item_id,
            item_title: String::new(),
            match_field: MatchField::Footnote,
            comment_id: 0,
            reply_id: 0,
            footnote_id: 7,
            occurrence_count: 1,
            snippet_before: String::new(),
            snippet_match: "note text".into(),
            snippet_after: String::new(),
            trashed: false,
        }
    }

    /// **Regression for the search_preview.rs dead-end (finding 4)**: selecting a
    /// footnote hit anchored to a real scene must not try to open that scene's
    /// document — the match is in the note's body, which the scene's own document
    /// never contains — so `preview` stays empty and `preview_field` names the
    /// footnote, which is what tells `PreviewBody` to render the footnote-aware
    /// empty state instead of silently failing to resolve an editable field.
    #[test]
    fn selecting_an_anchored_footnote_result_never_opens_the_scenes_document() {
        let vm = vm();
        vm.results
            .list_model()
            .replace_all(vec![footnote_row(1, 42)]);
        vm.select_result(1);
        assert!(
            vm.preview_signal().get().is_none(),
            "a footnote's body is not the scene's own document"
        );
        assert_eq!(vm.preview_field_signal().get(), Some(MatchField::Footnote));
        assert_eq!(vm.selected_result_signal().get(), Some(1));
        assert!(
            vm.preview_open_id.borrow().is_none(),
            "nothing was opened, so nothing is pinned in the doc store"
        );
    }

    /// **Regression for the orphan dead-end (finding 5)**: an unanchored footnote's
    /// result carries `binder_item_id == 0` (a real, documented state — see
    /// `work_management::load_work_uc`) — `select_result` must take the exact same
    /// footnote path as the anchored case, not fall through to `self.docs.open(0)`,
    /// which resolves to nothing and used to leave the preview looking exactly like
    /// nothing had been selected at all.
    #[test]
    fn selecting_an_orphaned_footnotes_result_takes_the_same_path_as_an_anchored_one() {
        let vm = vm();
        vm.results
            .list_model()
            .replace_all(vec![footnote_row(2, 0)]);
        vm.select_result(2);
        assert!(vm.preview_signal().get().is_none());
        assert_eq!(
            vm.preview_field_signal().get(),
            Some(MatchField::Footnote),
            "an orphan's match still names itself as a footnote hit, \
             not nothing-selected"
        );
        assert_eq!(vm.selected_result_signal().get(), Some(2));
    }
}
