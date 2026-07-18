// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WelcomeViewModel` — the Launcher's Welcome content.
//!
//! The business actions (open a recent/example work, pick a file, create a new
//! work) live here, not in the view's `build()` — and so does the recents
//! **search**: the query signal the `SearchField` writes into, the projection
//! that filters the list through it, and the cursor that follows.
//!
//! **Single-instance live state**, not a store-backed facade. It used to be the
//! latter (`SettingsViewModel`-shaped: only the `show_welcome` signal, the app
//! handle and the window factory — so a fresh instance was a view over the same
//! truth and the panel could rebuild it on every `build()`). Now that it owns
//! the query and the projection wired to it, a second instance would be a
//! *second* query — the field would write into one and the list would read the
//! other. So `WelcomePanel` creates it once (`Option` + `get_or_insert_with`,
//! since `SettingsStore` only exists at build time) and shares it by `.clone()`,
//! the same shape `App` uses for `EditorsViewModel`. The `show_welcome` signal is
//! still store-cached, so it stays shared per key across instances regardless.
//!
//! **Launcher-window model**: none of these methods touch the backend
//! directly any more. Loading/creating a work here — in the Launcher window,
//! before any project window's `App` exists — would race that window's
//! `LoadWork`/`NewWork` subscription and silently skip the seed flow
//! (`AppIds::seed`, `SingleWork::set_id`, the tree reload, …). Instead every
//! action here opens a **project window** carrying the action as a
//! [`PendingAction`], performed on that window's own first build once its
//! subscriptions are live (mirrors the pre-existing argv-launch mechanism),
//! then closes the Launcher — opening the new window *before* closing this
//! one, per the ordering rule in `main.rs`'s module docs.

use std::rc::Rc;

use bastyde::core::ObserverHandle;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::data::{
    ListDataSource, ListModel, SelectionMode, SelectionModel, SortFilterListModel,
};
use bastyde::prelude::*; // EventContext, Signal, tr!, FileDialogRequest/Result
use bastyde::settings::SettingsStore;
use bastyde::widgets::Toast;

use frontend::AppContext;
use frontend::direct_access::RecentWorkDto;

use crate::SHOW_WELCOME_KEY;
use crate::app::PendingAction;
use crate::models::RecentWorkListModel;
use crate::panels::new_work::NewWorkPanel;
use crate::shell::windows::ProjectWindowFactory;

/// The recents projection's one filter column. `SortFilterListModel` keys
/// predicates by column id (it is built for `TableView` headers); the recents
/// list has a single query over the whole row, so the id never leaves this file.
const RECENTS_QUERY: &str = "query";

/// Pages of the recents region, in the order [`WelcomePanel`] stacks them into
/// its `Switcher` (`crate::panels::welcome::WelcomePanel::recents_list`). Three,
/// not two: "you have no recent works" and "your search matched none of them"
/// are different facts, and showing the first when the second is true reads as
/// *the recents list is gone*.
pub const RECENTS_PAGE_EMPTY: usize = 0;
pub const RECENTS_PAGE_NO_MATCH: usize = 1;
pub const RECENTS_PAGE_LIST: usize = 2;

/// The two public links the Welcome sidebar offers under its nav — the pair
/// v1.9.x carried in its own welcome screen. They live here rather than in the
/// view because *which* addresses the app advertises is a product fact, not a
/// layout one (and the view is meant to be thin).
pub const GITHUB_URL: &str = "https://github.com/jacquetc/skribisto";
pub const DISCORD_URL: &str = "https://discord.gg/5BSkvQmyVH";

#[derive(Clone)]
pub struct WelcomeViewModel {
    show_welcome: Signal<bool>,
    app_ctx: Rc<AppContext>,
    /// Builds the project window a successful open/create/import opens,
    /// before this (Launcher) window closes.
    factory: ProjectWindowFactory,
    /// The MRU behind the recents list — the *unfiltered* truth.
    recents: RecentWorkListModel,
    /// The live search query. The `SearchField` writes into it on every
    /// keystroke; [`Self::recents_source`] is what reads it.
    search: Signal<String>,
    /// `recents` projected through `search` — what the list actually shows.
    recents_view: SortFilterListModel<RecentWorkDto>,
    recents_selection: SelectionModel,
    /// Keeps the query → filter observer attached for as long as any handle to
    /// this view-model lives (dropping an `ObserverHandle` detaches it, so a
    /// non-held one would unsubscribe at the end of `new`). `Rc` because the
    /// view-model is `Clone` and the handle is not — the same shape
    /// `BinderBinderItemsTreeModel` uses for its filter observers.
    _query_obs: Rc<ObserverHandle>,
}

#[allow(dead_code)]
impl WelcomeViewModel {
    pub fn new(
        store: &SettingsStore,
        app_ctx: Rc<AppContext>,
        factory: ProjectWindowFactory,
    ) -> Self {
        let recents = RecentWorkListModel::new(app_ctx.clone());
        let search = Signal::new(String::new());
        // Single: these rows are launch targets — you open one project, so a
        // multi-select cursor would be meaningless.
        let recents_selection = SelectionModel::new(SelectionMode::Single);
        let (recents_view, query_obs) =
            project_recents(recents.list_model(), &search, recents_selection.clone());
        Self {
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            app_ctx,
            factory,
            recents,
            search,
            recents_view,
            recents_selection,
            _query_obs: Rc::new(query_obs),
        }
    }

    /// The persisted "show at startup" signal — bound by the Launcher's
    /// inline checkbox and the Settings toggle (same cached
    /// `SHOW_WELCOME_KEY` signal).
    pub fn show_welcome(&self) -> Signal<bool> {
        self.show_welcome.clone()
    }

    // ── the recents search ──

    /// Subscribe the recents model to the backend (once) — it refreshes itself
    /// on each `LoadWork`/`NewWork`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.recents.wire(ctx);
    }

    /// The live query — hand a clone to `SearchField::new`. It is two-way: the
    /// field writes every keystroke into it, and [`project_recents`]'s observer
    /// re-filters the list on the spot.
    pub fn search_query(&self) -> Signal<String> {
        self.search.clone()
    }

    /// The rows the recents list shows: the MRU seen through the query. Hand to
    /// `ListView::from_source`; resolve the indices it hands back with
    /// [`Self::recent_path`], never against the unfiltered model.
    pub fn recents_source(&self) -> SortFilterListModel<RecentWorkDto> {
        self.recents_view.clone()
    }

    /// The list's keyboard/pointer cursor (index-based, as
    /// `SortFilterListModel` prescribes for a projection: selection over it is
    /// positional, not by identity — hence the reset in [`project_recents`]).
    pub fn recents_selection(&self) -> SelectionModel {
        self.recents_selection.clone()
    }

    /// How many rows survive the current query.
    pub fn visible_recents(&self) -> usize {
        self.recents_view.len()
    }

    /// Path of the **visible** row `i`. The index `ListView::on_activate` hands
    /// back is an index into the projection, not into the MRU — with a query
    /// active the two disagree, and reading the MRU with a visible index is how
    /// you open the wrong project.
    pub fn recent_path(&self, index: usize) -> Option<String> {
        self.recents_view
            .with_item(index, |row| row.absolute_path.clone())
    }

    /// Which page the recents region shows — `RECENTS_PAGE_*`.
    pub fn recents_page(&self) -> Signal<usize> {
        recents_page_signal(
            self.recents.list_model(),
            self.recents_view.clone(),
            &self.search,
            &self.recents.version_signal(),
        )
    }

    /// Put the cursor on the top row if it is nowhere — so the Launcher opens
    /// with the most recent work under it and Enter resumes without aiming
    /// first. Guarded on "nothing selected yet" so a rebuild never yanks the
    /// highlight back to the top after the user has arrowed away from it (a
    /// *query* change does move it, deliberately — see [`project_recents`]).
    pub fn preselect_first(&self) {
        if self.recents_view.len() > 0 && self.recents_selection.selected_indices().is_empty() {
            self.recents_selection.select(0);
        }
    }

    /// Open a recent/known work by path: opens a project window carrying
    /// `PendingAction::Load(path)`, then closes the Launcher.
    ///
    /// The backup sniff (a blocking `File::open` + zip parse with no timeout —
    /// see `crate::backup::is_backup_path`) runs off the UI thread (T2-3):
    /// clicking any recent entry must never hang the app on a disconnected
    /// network/FUSE mount.
    pub fn open_work(&self, path: String, ctx: &mut EventContext) {
        let factory = self.factory.clone();
        let path_for_check = path.clone();
        ctx.spawn_local_with(
            async move {
                spawn_blocking(move || crate::backup::is_backup_path(&path_for_check))
                    .await
                    .unwrap_or(false)
            },
            move |is_backup, ectx| {
                // A backup always opens in its own instance (never as this
                // process's project) — see the backup-mode invariant.
                if is_backup {
                    ectx.request_activation_token_self(Box::new(move |tok| {
                        crate::shell::process::spawn_new_process(&path, tok);
                    }));
                    return;
                }
                // Open the project window *before* closing the Launcher — the
                // ordering rule in `main.rs`'s module docs.
                ectx.open_window(factory.window_config(PendingAction::Load(path.clone())));
                ectx.close_window();
            },
        )
        .detach();
    }

    /// Open a bundled example. Its bytes are embedded in the binary; write them
    /// to a per-user temp copy (so the read-only repo original is never mutated
    /// or saved over) and load that.
    pub fn open_example(&self, file_name: &str, bytes: &[u8], ctx: &mut EventContext) {
        match write_temp_example(file_name, bytes) {
            Ok(path) => self.open_work(path, ctx),
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(could_not_open_example(
                    error = e.to_string()
                ))));
            }
        }
    }

    /// "Open" button — native picker for an existing `.skrib`, then open a
    /// project window carrying `PendingAction::Load`. The backup sniff runs
    /// off the UI thread (T2-3), same rationale as [`Self::open_work`].
    pub fn pick_open(&self, ctx: &mut EventContext) {
        let factory = self.factory.clone();
        let req = FileDialogRequest::pick_file()
            .title("Open Skribisto work")
            .add_filter("Skribisto work", &["skrib"]);
        let _ = ctx.pick_file(req, move |res, ectx| {
            if let FileDialogResult::File(Some(path)) = res {
                let file = path.to_string_lossy().into_owned();
                let factory = factory.clone();
                let file_for_check = file.clone();
                ectx.spawn_local_with(
                    async move {
                        spawn_blocking(move || crate::backup::is_backup_path(&file_for_check))
                            .await
                            .unwrap_or(false)
                    },
                    move |is_backup, ectx2| {
                        if is_backup {
                            ectx2.request_activation_token_self(Box::new(move |tok| {
                                crate::shell::process::spawn_new_process(&file, tok);
                            }));
                            return;
                        }
                        ectx2.open_window(factory.window_config(PendingAction::Load(file.clone())));
                        ectx2.close_window();
                    },
                )
                .detach();
            }
        });
    }

    /// "New Work" button — present the New Work modal directly in the
    /// Launcher window. There is no `App`/`work.new` global action to
    /// dispatch an intent to here (that action only exists inside an
    /// already-open project window's tree), so this builds
    /// [`NewWorkPanel::new_for_launcher`] directly: submitting the form opens
    /// a project window carrying `PendingAction::New`, then closes the
    /// Launcher — see `NewWorkViewModel::create`.
    pub fn new_work(&self, ctx: &mut EventContext) {
        let app_ctx = self.app_ctx.clone();
        let factory = self.factory.clone();
        ctx.present_modal(
            ModalRequest::deferred(move |t| {
                t.add(NewWorkPanel::new_for_launcher(
                    app_ctx.clone(),
                    factory.clone(),
                ))
            })
            .presentation(ModalPresentation::InTree)
            .title("New Work")
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
            .size(600, 680),
        );
    }

    /// The sidebar's GitHub / Discord links — hand the URL to the OS default
    /// handler, i.e. the user's browser.
    ///
    /// `that_detached`, not `that`: the latter keeps the spawned opener as a
    /// child process and waits for it to exit, and the desktop opener a browser
    /// is launched through does not reliably return before the browser itself
    /// does — on a cold start that is the UI thread stalled for seconds.
    /// Detaching hands the child to the OS and returns now.
    ///
    /// A launch that fails is reported: a click that silently does nothing reads
    /// as a dead button, and a launcher sidebar has no other surface to notice
    /// it on.
    pub fn open_link(&self, url: &str, ctx: &mut EventContext) {
        if let Err(e) = open::that_detached(url) {
            ctx.show_toast(Toast::error(tr!(could_not_open_link(
                url = url.to_string(),
                error = e.to_string()
            ))));
        }
    }
}

/// Project a recents `ListModel` through a live query.
///
/// The returned [`SortFilterListModel`] is the framework's own filter
/// projection — the list binds to it and re-renders on its `Reset`, so nothing
/// here has to rebuild a widget or maintain a parallel filtered `Vec`.
///
/// **Why the filter is pushed imperatively** instead of the tidier
/// `view.filters_signal(search.map(|q| ..))`: `SortFilterListModel::filters_signal`
/// *observes* the signal it is handed, and `Signal::observe` panics on a
/// **derived** signal (`try_observe` → `SignalAccessError::ReadOnly`) — which is
/// all `search.map(..)` can ever produce. So we observe the mutable query itself
/// and call `set_filter`, which is the same one-way flow, minus the panic.
///
/// The returned `ObserverHandle` **must be held**: dropping it detaches the
/// observer, and the field goes back to typing into a signal nobody reads.
fn project_recents(
    rows: ListModel<RecentWorkDto>,
    search: &Signal<String>,
    selection: SelectionModel,
) -> (SortFilterListModel<RecentWorkDto>, ObserverHandle) {
    let view = SortFilterListModel::new(rows).with_predicate(RECENTS_QUERY, |text| {
        let needle = text.trim().to_lowercase();
        Box::new(move |row: &RecentWorkDto| recent_matches(row, &needle))
    });

    let filter_view = view.clone();
    let handle = search.observe(move |query| {
        // An all-whitespace query is not a query. `set_filter` with empty text
        // *removes* the column's filter, which is exactly "show everything".
        filter_view.set_filter(RECENTS_QUERY, query.trim());
        // Selection over a projection is positional (`SortFilterListModel::Key
        // = usize`, deliberately), so the rows under the cursor just changed
        // identity: visible row 3 is a different work now, or is gone. Leaving
        // the old index selected would aim the highlight — and Enter — at
        // whatever slid into it. Put the cursor on the top match, the way every
        // search list does.
        if filter_view.len() > 0 {
            selection.select(0);
        } else {
            selection.clear();
        }
    });
    (view, handle)
}

/// Does a recent work match the query? Case-insensitive substring over the two
/// things the row actually shows: its **title** and its **path** — the path
/// being what tells two `Draft.skrib`s in different folders apart. Same rule as
/// the outline tree's search (`BinderBinderItemsTreeModel`), so "find by typing"
/// means one thing across the app.
fn recent_matches(row: &RecentWorkDto, needle: &str) -> bool {
    row.title.to_lowercase().contains(needle) || row.absolute_path.to_lowercase().contains(needle)
}

/// The recents region's page index: nothing recorded / nothing matched / the
/// list. See `RECENTS_PAGE_*`.
///
/// **Derived, and that is what makes it correct.** A derived signal recomputes
/// lazily when a source is dirty — i.e. at bind time, *after* the query
/// observer in [`project_recents`] has already re-run the filter — so reading
/// `view.len()` inside sees the new projection, not the old one. Its two sources
/// are the only roots a row count can move on: the query, and the MRU's refresh
/// counter (bumped on every `LoadWork`/`NewWork`).
fn recents_page_signal(
    rows: ListModel<RecentWorkDto>,
    view: SortFilterListModel<RecentWorkDto>,
    search: &Signal<String>,
    version: &Signal<u64>,
) -> Signal<usize> {
    search.zip(version).map(move |_| {
        if rows.is_empty() {
            RECENTS_PAGE_EMPTY
        } else if view.len() == 0 {
            RECENTS_PAGE_NO_MATCH
        } else {
            RECENTS_PAGE_LIST
        }
    })
}

/// Write an embedded example's bytes to a per-user temp dir (always rewritten,
/// so a stale/partial copy never blocks a fresh open) and return its path.
fn write_temp_example(file_name: &str, bytes: &[u8]) -> std::io::Result<String> {
    let mut dir = std::env::temp_dir();
    dir.push("skribisto-examples");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(file_name);
    std::fs::write(&path, bytes)?;
    Ok(path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use crate::app::PendingExit;
    use crate::app_ids::AppIds;
    use crate::models::BackupSettingsService;
    use crate::singles::{SingleWork, SingleWorkInfo};
    use crate::view_models::{
        BackupSchedulerViewModel, BackupSettingsViewModel, ExportViewModel, OutlineViewModel,
        SaveAsViewModel,
    };

    /// A minimal, fully in-memory `ProjectWindowFactory` — enough plumbing to
    /// construct a `WelcomeViewModel` in a test; these particular tests never
    /// exercise the factory's `window_config`.
    fn test_factory(app_ctx: Rc<AppContext>) -> ProjectWindowFactory {
        let ids = AppIds::new();
        let outline = OutlineViewModel::new_default(app_ctx.clone(), ids.clone());
        let export = ExportViewModel::new(app_ctx.clone(), ids.clone());
        let single_work = SingleWork::new(app_ctx.clone());
        let single_work_info = SingleWorkInfo::new(app_ctx.clone());
        let backup_mode = Signal::new(false);
        let backup_context = Signal::new(None);
        let save_as_vm = SaveAsViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            single_work.clone(),
            backup_mode.clone(),
            backup_context.clone(),
        );
        let backup_settings =
            BackupSettingsViewModel::new(BackupSettingsService::in_memory_default());
        let backup_scheduler = BackupSchedulerViewModel::new(
            app_ctx.clone(),
            backup_settings,
            single_work.clone(),
            single_work_info.clone(),
            backup_mode.clone(),
        );
        ProjectWindowFactory::new(
            app_ctx,
            outline,
            export,
            single_work,
            single_work_info,
            Signal::new(false), // autosave_menu
            Signal::new(true),  // spellcheck_menu (default on)
            save_as_vm,
            backup_mode,
            backup_context,
            Signal::new(false),
            Signal::new(PendingExit::None),
            backup_scheduler,
            Rc::new(RefCell::new(None)),
        )
    }

    #[test]
    fn welcome_show_default_on_and_persists() {
        // A real TOML store at a unique temp path (no in-memory store exists).
        let path = std::env::temp_dir().join(format!(
            "skribisto-welcome-test-{}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let store = SettingsStore::open(path.clone()).expect("open settings store");
        let app_ctx = Rc::new(AppContext::new());

        let vm = WelcomeViewModel::new(&store, app_ctx.clone(), test_factory(app_ctx.clone()));
        assert!(vm.show_welcome().get(), "defaults to on");

        vm.show_welcome().set(false);
        // A second facade over the same store observes the change (same cached
        // signal per key) — the store-backed-facade invariant.
        let vm2 = WelcomeViewModel::new(&store, app_ctx.clone(), test_factory(app_ctx));
        assert!(
            !vm2.show_welcome().get(),
            "toggle persists across instances"
        );

        let _ = std::fs::remove_file(&path);
    }

    // ── the recents search ───────────────────────────────────────────────────
    //
    // Exercised through the free functions rather than through a whole
    // `WelcomeViewModel`, so the rows are fabricated here: the real
    // `RecentWorkListModel` reads the *user's* persisted MRU file, which would
    // make "does this query match" depend on whatever the person running the
    // tests happens to have opened.

    fn recent(title: &str, path: &str) -> RecentWorkDto {
        RecentWorkDto {
            title: title.to_string(),
            absolute_path: path.to_string(),
            ..Default::default()
        }
    }

    /// Three works, one of which ("Faux-semblants") is only findable by title,
    /// one ("Draft") only by the folder it sits in.
    fn sample_rows() -> ListModel<RecentWorkDto> {
        ListModel::from_vec(vec![
            recent("Faux-semblants", "/home/w/novels/Faux-semblants.skrib"),
            recent("Starforgers", "/home/w/sci-fi/Starforgers.skrib"),
            recent("Draft", "/home/w/novels/Draft.skrib"),
        ])
    }

    fn wired(
        rows: ListModel<RecentWorkDto>,
    ) -> (
        Signal<String>,
        SortFilterListModel<RecentWorkDto>,
        SelectionModel,
        ObserverHandle,
    ) {
        let search = Signal::new(String::new());
        let selection = SelectionModel::new(SelectionMode::Single);
        let (view, obs) = project_recents(rows, &search, selection.clone());
        (search, view, selection, obs)
    }

    fn titles(view: &SortFilterListModel<RecentWorkDto>) -> Vec<String> {
        (0..view.len())
            .filter_map(|i| view.with_item(i, |r| r.title.clone()))
            .collect()
    }

    /// The bug this whole change exists for: the field wrote into a signal
    /// nobody read, so typing filtered nothing.
    #[test]
    fn typing_filters_the_recents_list() {
        let (search, view, _sel, _obs) = wired(sample_rows());
        assert_eq!(view.len(), 3, "no query = every recent work");

        search.set("star".to_string());
        assert_eq!(titles(&view), ["Starforgers"], "case-insensitive on title");

        // Matched on the *path* only — the row shows it, so it is searchable.
        search.set("sci-fi".to_string());
        assert_eq!(titles(&view), ["Starforgers"]);

        search.set("novels".to_string());
        assert_eq!(titles(&view), ["Faux-semblants", "Draft"]);

        search.set("zzz".to_string());
        assert!(view.len() == 0, "a query nothing matches empties the list");

        search.set(String::new());
        assert_eq!(view.len(), 3, "clearing the field restores every row");
    }

    /// An all-whitespace query is not a query — it must not empty the list.
    #[test]
    fn a_blank_query_is_no_filter() {
        let (search, view, _sel, _obs) = wired(sample_rows());
        search.set("   ".to_string());
        assert_eq!(view.len(), 3);
        // …and a padded one still finds its row.
        search.set("  star  ".to_string());
        assert_eq!(titles(&view), ["Starforgers"]);
    }

    /// The cursor is positional over the projection, so it has to move with it —
    /// otherwise Enter opens whatever row slid under the stale index. Here: park
    /// the cursor on row 2 ("Draft"), then filter down to one row; if the index
    /// stayed at 2 it would point past the end of the list.
    #[test]
    fn the_cursor_follows_the_filtered_rows() {
        let (search, view, selection, _obs) = wired(sample_rows());
        selection.select(2);
        assert_eq!(selection.selected_indices(), [2]);

        search.set("star".to_string());
        assert_eq!(
            selection.selected_indices(),
            [0],
            "the cursor lands on the top match, in range"
        );
        assert_eq!(
            view.with_item(0, |r| r.title.clone()).as_deref(),
            Some("Starforgers"),
            "…and row 0 really is the match — the index the list hands back \
             resolves against the projection"
        );

        search.set("zzz".to_string());
        assert!(
            selection.selected_indices().is_empty(),
            "nothing matched: no cursor to have"
        );
    }

    /// "No recent works yet" and "nothing matched your search" are different
    /// facts; the region shows a different page for each, and the list for
    /// neither.
    #[test]
    fn the_page_tells_empty_apart_from_no_match() {
        let rows = sample_rows();
        let (search, view, _sel, _obs) = wired(rows.clone());
        let version = Signal::new(0u64);
        let page = recents_page_signal(rows.clone(), view, &search, &version);

        assert_eq!(page.get(), RECENTS_PAGE_LIST);

        search.set("zzz".to_string());
        assert_eq!(page.get(), RECENTS_PAGE_NO_MATCH, "matched nothing");

        search.set(String::new());
        assert_eq!(page.get(), RECENTS_PAGE_LIST);

        rows.clear();
        assert_eq!(
            page.get(),
            RECENTS_PAGE_EMPTY,
            "no recents at all — regardless of the query"
        );
        search.set("star".to_string());
        assert_eq!(page.get(), RECENTS_PAGE_EMPTY);
    }
}
