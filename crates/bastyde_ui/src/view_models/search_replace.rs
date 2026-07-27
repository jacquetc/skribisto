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

use bastyde::prelude::*;
use bastyde::widgets::{DockWidgetId, DockingModel};

use frontend::AppContext;
use frontend::commands::{search_management_commands, undo_redo_commands, work_commands};
use frontend::common::entities::MatchField;
use frontend::direct_access::SearchResultDto;
use frontend::search_management::{ReplaceInProjectDto, ReplaceInProjectResultDto, RunSearchDto};

use skribisto_model::SearchFacet;

use crate::app_ids::AppIds;
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
    include_trashed: Signal<bool>,
    /// One two-way `Signal<bool>` per facet chip, indexed by the chip's position
    /// in [`SearchFacet::ALL`]. Individual signals (not one `HashSet`) because the
    /// chips are `ToolbarAction::toggle`s, which each bind a `Signal<bool>` and
    /// write it directly on click.
    facets: [Signal<bool>; 6],

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
    preview: Signal<Option<Rc<OpenDoc>>>,
    preview_field: Signal<Option<MatchField>>,
    /// The item id whose doc the preview currently holds a store ref for — so
    /// `release` is exactly symmetric with `open` (the store is refcounted).
    preview_open_id: Rc<RefCell<Option<u64>>>,

    // ── replace review ──
    /// `SearchResult` ids the writer unticked — the fields Replace All will skip.
    excluded: Signal<HashSet<u64>>,
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
// caller yet — same convention as `view_models::outline`.
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
            include_trashed: Signal::new(p.include_trashed),
            facets: facet_signals_from_codes(&p.facets),
            match_count: Signal::new(0),
            item_count: Signal::new(0),
            truncated: Signal::new(false),
            ran: Signal::new(false),
            error: Signal::new(None),
            selected_result: Signal::new(None),
            preview: Signal::new(None),
            preview_field: Signal::new(None),
            preview_open_id: Rc::new(RefCell::new(None)),
            excluded: Signal::new(HashSet::new()),
            show_replace: Signal::new(false),
            query_suggestions: Signal::new(Vec::new()),
            replacement_suggestions: Signal::new(Vec::new()),
            deadline: Rc::new(Cell::new(None)),
            last_persisted: Rc::new(RefCell::new(None)),
        }
    }

    // ── view handles (signals the widgets bind to) ──────────────────────────
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
    pub fn preview_dock_id(&self) -> DockWidgetId {
        self.preview_dock_id
    }
    /// Reveal (switch to) the leading search & replace dock — the preview's
    /// empty-state "Search" button and the View ▸ Search menu drive this.
    pub fn reveal_search(&self) {
        self.docking.reveal_dock(self.search_dock_id);
    }
    /// The `search.toml` service's `Reloadable` hook, for registering with the
    /// app's shared `SettingsRegistry` (live cross-process reload).
    pub fn settings_reloadable(&self) -> std::rc::Rc<dyn bastyde::settings::Reloadable> {
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
            include_trashed: self.include_trashed.get(),
        }
    }

    /// The matching options as a text-document [`FindOptions`], for the bottom
    /// preview's find-highlight session (so the previewed paragraph highlights the
    /// same matches the result list found).
    pub fn find_options(&self) -> bastyde::text_document::FindOptions {
        bastyde::text_document::FindOptions {
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
        }
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

    /// How many result rows are ticked (not excluded).
    pub fn included_count(&self) -> usize {
        let excluded = self.excluded.get();
        self.results
            .items()
            .iter()
            .filter(|r| !excluded.contains(&r.id))
            .count()
    }

    /// The binder-item ids Replace All will touch (ticked results only), deduped —
    /// the set to reload afterwards if any are open in a tab (see [`reload_touched`]).
    pub fn touched_item_ids(&self) -> Vec<u64> {
        let excluded = self.excluded.get();
        let mut ids: Vec<u64> = self
            .results
            .items()
            .iter()
            .filter(|r| !excluded.contains(&r.id))
            .map(|r| r.binder_item_id)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    /// Execute Replace All over the ticked results, on the open project's undo
    /// stack. Synchronous (the backend `replace_in_project` command is), returning
    /// its summary for the completion toast. The caller (which has an
    /// `EventContext`) surfaces the toast + its Undo, reloads open docs, and
    /// re-runs the search.
    pub fn replace_all(&self) -> anyhow::Result<ReplaceInProjectResultDto> {
        let work_id = self
            .ids
            .work_id
            .get()
            .ok_or_else(|| anyhow::anyhow!("replace_all: no open project"))?;
        let dto = ReplaceInProjectDto {
            work_id,
            replacement: self.replacement.get(),
            preserve_case: self.preserve_case.get(),
            excluded_result_ids: self.excluded.get().into_iter().collect(),
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
        self.include_trashed.set(p.include_trashed);
        set_facets_from_codes(&self.facets, &p.facets);
        // The restored prefs are, by definition, what is on disk — cache them so
        // the first search after a load doesn't rewrite `search.toml` needlessly.
        *self.last_persisted.borrow_mut() = Some(p);
        // Clear the result list to the new project's (empty) result set — the old
        // project's rows were torn down with its store.
        self.results.reload();
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
        self.results.items().into_iter().find(|r| r.id == result_id)
    }
}

/// The position of facet `f` in [`SearchFacet::ALL`] — the index into the
/// per-facet signal array.
fn facet_index(f: SearchFacet) -> usize {
    SearchFacet::ALL.iter().position(|&x| x == f).unwrap_or(0)
}

/// Fresh per-facet toggle signals seeded from a list of codes. A code that names
/// no facet (a `search.toml` written by a version whose codes differed) is
/// ignored — never resurrected as a phantom filter.
fn facet_signals_from_codes(codes: &[i64]) -> [Signal<bool>; 6] {
    let on = codes_to_set(codes);
    std::array::from_fn(|i| Signal::new(on.contains(&SearchFacet::ALL[i])))
}

/// Set existing per-facet signals from a list of codes (used on project restore).
fn set_facets_from_codes(sigs: &[Signal<bool>; 6], codes: &[i64]) {
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
    use bastyde::widgets::DockingModel;

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
}
