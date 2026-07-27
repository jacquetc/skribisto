// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the recently-opened works — backed by a **persisted**
//! `bastyde::settings::MruList`, so the list **survives app restarts**.
//!
//! The durable source is one `MruList<RecentEntry>` at
//! `<config_dir>/recents.toml` (dedupe-by-path, capped, most-recently-opened
//! first, debounced writes). The public surface is unchanged: a
//! `bastyde::data::ListModel<RecentWorkDto>` a `ListView` binds to, plus a
//! `version` signal + `items()` snapshot for the non-`ListView` consumer (the
//! title-bar `ProjectSwitcherButton`).
//!
//! The bound `ListModel` is a **reachable-only view**: an entry whose file/folder
//! no longer exists is **hidden from the UI but kept in the MRU**, so a project on
//! a temporarily-detached drive (or a not-yet-saved new work) reappears once its
//! path is reachable again — it is never silently dropped.
//!
//! The two consumers each build their own `RecentWorkListModel`, so the `MruList`
//! is a process-wide singleton (thread-local, `Rc`-shared clones); opening it
//! twice would diverge on flush. On each successful open the just-opened work is
//! recorded via `MruList::add`; `flush_now()` is called once at app shutdown.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface (the real one
//! is `MruList`-backed; the mock ships one fabricated row).

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::{Cell, RefCell};
    use std::path::Path;
    use std::rc::Rc;

    use bastyde::data::ListModel;
    use bastyde::prelude::*;
    use bastyde::settings::{AppPaths, Keyed, MruEntry, MruList};
    use serde::{Deserialize, Serialize};

    use frontend::AppContext;
    use frontend::commands::{work_commands, work_info_commands};
    use frontend::common::event::{Event, Origin, WorkManagementEvent};
    use frontend::common::types::EntityId;
    use frontend::direct_access::RecentWorkDto;

    /// Max unpinned recents kept on disk.
    const MAX_RECENTS: usize = 30;

    /// One durable recent-works entry (persisted as TOML), keyed by absolute path.
    #[derive(Clone, Serialize, Deserialize)]
    struct RecentEntry {
        path: String,
        title: String,
        #[serde(default)]
        last_opened_ms: i64,
        #[serde(default)]
        pinned: bool,
    }

    impl Keyed for RecentEntry {
        type Key = String;
        fn key(&self) -> String {
            self.path.clone()
        }
    }

    impl MruEntry for RecentEntry {
        fn is_pinned(&self) -> bool {
            self.pinned
        }
        fn set_pinned(&mut self, pinned: bool) {
            self.pinned = pinned;
        }
        fn touch(&mut self) {
            self.last_opened_ms = now_ms();
        }
    }

    // One MruList per process, shared by every `RecentWorkListModel` (the title-bar
    // button and the Welcome panel each build their own). Two independent
    // `MruList::open`s over the same file would diverge on flush, so open once and
    // hand out cheap `Rc`-shared clones. The UI is single-threaded, so a
    // thread-local is the natural home.
    thread_local! {
        static SHARED_MRU: RefCell<Option<MruList<RecentEntry>>> = const { RefCell::new(None) };
    }

    /// The shared, lazily-opened recents MRU — `None` when the config dir is
    /// unavailable or the file can't be opened (recents degrade to empty).
    fn shared_mru() -> Option<MruList<RecentEntry>> {
        SHARED_MRU.with(|cell| {
            if cell.borrow().is_none() {
                let opened = AppPaths::new("eu", "skribisto", "Skribisto").and_then(|paths| {
                    MruList::open(&paths, "recents", MAX_RECENTS)
                        .map_err(|e| eprintln!("recents MRU: open failed: {e}"))
                        .ok()
                });
                *cell.borrow_mut() = opened;
            }
            cell.borrow().clone()
        })
    }

    fn now_ms() -> i64 {
        chrono::Utc::now().timestamp_millis()
    }

    struct Inner {
        /// Durable source of truth (`None` when the MRU couldn't be opened).
        mru: Option<MruList<RecentEntry>>,
        /// Reachable-only view the UI binds to (unreachable entries stay in the
        /// MRU but are hidden here).
        model: ListModel<RecentWorkDto>,
        /// Bumped on every refresh, for `Rebuild`-bound (non-`ListView`) consumers.
        version: Signal<u64>,
        /// One-shot guard: subscriptions persist across the consumer's rebuilds.
        subscribed: Cell<bool>,
        ctx: Rc<AppContext>,
        /// The main-thread async executor (T2-3): `on_open`'s backup sniff (a
        /// blocking `File::open` + zip parse — see `crate::backup::is_backup_path`)
        /// must never run on the UI thread, since building this list runs on every
        /// `LoadWork`/`NewWork`. Populated once in `wire` (`None` only if
        /// `install_async()` wasn't called at startup, which would be an app bug —
        /// `on_open` falls back to a synchronous check rather than panicking).
        async_rt: RefCell<Option<AsyncRuntimeHandle>>,
    }

    #[derive(Clone)]
    pub struct RecentWorkListModel {
        inner: Rc<Inner>,
    }

    impl RecentWorkListModel {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            // Populate eagerly from the persisted MRU so the list shows the saved
            // recents on the very first frame — before any work is opened.
            let mru = shared_mru();
            let model = ListModel::from_vec(visible_rows(&mru));
            Self {
                inner: Rc::new(Inner {
                    mru,
                    model,
                    version: Signal::new(0),
                    subscribed: Cell::new(false),
                    ctx,
                    async_rt: RefCell::new(None),
                }),
            }
        }

        /// Subscribe (once) so the list records + refreshes on each work open.
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
            *self.inner.async_rt.borrow_mut() = ctx.app_state::<AsyncRuntimeHandle>().cloned();
            let me = self.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |event: &Event| {
                    if let Some(&work_id) = event.ids.first() {
                        me.on_open(work_id);
                    }
                },
            );
            // A new project is also a "recently opened" work.
            let me = self.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |event: &Event| {
                    if let Some(&work_id) = event.ids.first() {
                        me.on_open(work_id);
                    }
                },
            );
        }

        /// The reactive (reachable-only) model to bind a `ListView` to.
        pub fn list_model(&self) -> ListModel<RecentWorkDto> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh; bind at `BindingLevel::Rebuild` for consumers
        /// that rebuild rather than observe the model (the dropdown button).
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// `Vec` snapshot of the reachable works, most-recently-opened first.
        pub fn items(&self) -> Vec<RecentWorkDto> {
            snapshot(&self.inner.model)
        }

        /// One-shot read of **every** raw recents entry's path — including ones
        /// currently hidden from the UI because their file isn't reachable right
        /// now (see the module doc: "hidden but kept"). Used only by `main.rs`'s
        /// one-time `window_state.toml` reconciliation sweep, which must not treat
        /// a project on a temporarily-unmounted drive as orphaned just because
        /// [`Self::items`]'s UI-filtered view doesn't show it right now.
        ///
        /// Opens a short-lived `MruList` handle, reads it, and drops it
        /// immediately — safe even though [`SHARED_MRU`]'s own "don't open twice"
        /// note warns against two *live* handles diverging: that's about
        /// concurrent divergence, not a one-shot read taken (and dropped) before
        /// the real, long-lived handle (`shared_mru()`) is ever opened for this
        /// process — `main.rs` calls this before constructing any
        /// `RecentWorkListModel` for real.
        pub(crate) fn all_raw_paths() -> Vec<String> {
            let Some(paths) = AppPaths::new("eu", "skribisto", "Skribisto") else {
                return Vec::new();
            };
            match MruList::<RecentEntry>::open(&paths, "recents", MAX_RECENTS) {
                Ok(mru) => raw_paths(&mru),
                Err(e) => {
                    eprintln!("recents MRU: one-shot open for window-state prune failed: {e}");
                    Vec::new()
                }
            }
        }

        /// Flush the shared recents MRU to disk **and release it**, synchronously.
        /// Call once at app shutdown, so a just-opened work isn't lost inside the
        /// debounce window.
        ///
        /// Taking the `MruList` out of the thread-local — rather than leaving it
        /// to the thread-local's own destructor — is what keeps exit *possible*.
        /// Dropping an `MruList` blocks until the shared settings-writer thread
        /// acks its final flush, and on Windows a thread-local's destructor runs
        /// inside `DLL_PROCESS_DETACH`, which `ExitProcess` reaches only after it
        /// has already killed that writer thread: the ack would never arrive and
        /// the process would hang forever, unkillable, with its window still on
        /// screen. Called from `main`, the writer is alive and acks immediately.
        pub fn shutdown() {
            let mru = SHARED_MRU.with(|cell| cell.borrow_mut().take());
            if let Some(mru) = mru
                && let Err(e) = mru.flush_now()
            {
                eprintln!("recents MRU: flush failed: {e}");
            }
            // Dropped here, on a live app — not during process teardown.
        }

        /// On a successful open, record the just-opened work in the durable MRU
        /// (dedupe by path, front-inserted), then refresh the reachable view.
        ///
        /// The backup sniff (T2-3) runs off the UI thread via the main-thread
        /// async executor: this fires on every `LoadWork`/`NewWork`, so it must
        /// never block. `mru.add` + `refresh` need no `EventContext` (no ambient
        /// op), so a fire-and-forget `spawn_local` is enough — no
        /// `spawn_local_with` completion hop required.
        fn on_open(&self, work_id: EntityId) {
            let Some(entry) = opened_entry(&self.inner.ctx, work_id) else {
                self.refresh();
                return;
            };
            let Some(mru) = self.inner.mru.clone() else {
                self.refresh();
                return;
            };
            match self.inner.async_rt.borrow().clone() {
                Some(rt) => {
                    let me = self.clone();
                    rt.spawn_local(async move {
                        let path = entry.path.clone();
                        // A backup file is never added to "Recent" — it's a
                        // point-in-time copy opened in its own window, not a
                        // project you return to.
                        let is_backup =
                            spawn_blocking(move || crate::backup::is_backup_path(&path))
                                .await
                                .unwrap_or(false);
                        if !is_backup {
                            mru.add(entry);
                        }
                        me.refresh();
                    })
                    .detach();
                }
                None => {
                    // `install_async()` wasn't called at startup (an app bug, not
                    // a normal runtime state) — fall back to a synchronous check
                    // rather than silently dropping the "recently opened" record.
                    if !crate::backup::is_backup_path(&entry.path) {
                        mru.add(entry);
                    }
                    self.refresh();
                }
            }
        }

        /// Re-derive the reachable view from the MRU, then bump `version`.
        ///
        /// `reconcile_by_key` (keyed on `absolute_path`, this list's stable
        /// identity — see the module doc) rather than `replace_all`: the latter
        /// emits `DataChange::Reset`, which would blow away the Welcome list's
        /// row-level state (selection, in-flight open) every time *any* window
        /// records an open — including a peer process's, once the settings
        /// watcher or another local refresh trigger fires this. Reconciling
        /// emits only the granular inserts/removes/moves/updates the new
        /// snapshot actually needs, so an unrelated peer-driven refresh doesn't
        /// reset the list out from under the user.
        fn refresh(&self) {
            self.inner
                .model
                .reconcile_by_key(visible_rows(&self.inner.mru), |dto| {
                    dto.absolute_path.clone()
                });
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// The just-opened work as an MRU entry: its on-disk path + title. `None` when
    /// `work_id` (the id carried by the `LoadWork`/`NewWork` event that triggered
    /// this — see `wire`) has no file path yet.
    ///
    /// Resolved by `work_id`, not `get_all_work_info(ctx)`'s first entry:
    /// `WorkInfoDto.work` is this row's own backlink, so filtering on it (rather
    /// than trusting store-iteration order) stays correct once more than one
    /// Work/WorkInfo can be open at a time — unlike `AppIds.work_id`, `work_id`
    /// here comes straight off the event that just fired, so it can never be
    /// stale relative to some *other* subscriber's re-seeding order.
    fn opened_entry(ctx: &AppContext, work_id: EntityId) -> Option<RecentEntry> {
        let path = work_info_commands::get_all_work_info(ctx)
            .ok()?
            .into_iter()
            .find(|wi| wi.work == Some(work_id))?
            .file_name?;
        let title = work_commands::get_work(ctx, &work_id)
            .ok()
            .flatten()
            .map(|w| w.title)
            .unwrap_or_default();
        Some(RecentEntry {
            path,
            title,
            last_opened_ms: now_ms(),
            pinned: false,
        })
    }

    /// Pure: every raw entry's path from an already-opened MRU, in whatever order
    /// the model holds them — **not** filtered by reachability (unlike
    /// [`visible_rows`]). Factored out from [`RecentWorkListModel::all_raw_paths`]
    /// so it's testable against a temp-file `MruList` (see the module's other
    /// tests) without touching the real recents file.
    fn raw_paths(mru: &MruList<RecentEntry>) -> Vec<String> {
        let model = mru.model();
        (0..model.len())
            .filter_map(|i| model.with_item(i, |e| e.path.clone()))
            .collect()
    }

    /// Reachable recents as UI rows, most-recently-opened first (MRU order).
    /// Entries whose file/folder no longer exists are hidden here but remain in
    /// the MRU (see the module doc).
    fn visible_rows(mru: &Option<MruList<RecentEntry>>) -> Vec<RecentWorkDto> {
        let Some(mru) = mru else {
            return Vec::new();
        };
        let model = mru.model();
        (0..model.len())
            .filter_map(|i| model.with_item(i, |e| e.clone()))
            .filter(|e| Path::new(&e.path).exists())
            .map(to_dto)
            .collect()
    }

    fn to_dto(e: RecentEntry) -> RecentWorkDto {
        RecentWorkDto {
            id: 0,
            title: e.title,
            absolute_path: e.path,
            last_opened_at: chrono::DateTime::from_timestamp_millis(e.last_opened_ms)
                .unwrap_or_default(),
            ..Default::default()
        }
    }

    /// Materialize the model's current rows into a `Vec`.
    fn snapshot(model: &ListModel<RecentWorkDto>) -> Vec<RecentWorkDto> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |d| d.clone()))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use frontend::commands::{handling_app_lifecycle_commands, work_management_commands};
        use frontend::work_management::{NewWorkDto, NewWorkTemplate};
        use std::time::Duration;

        fn tmp(name: &str) -> std::path::PathBuf {
            std::env::temp_dir().join(format!("skribisto_test_{}_{name}", std::process::id()))
        }

        fn entry(path: &str, title: &str, ms: i64) -> RecentEntry {
            RecentEntry {
                path: path.to_string(),
                title: title.to_string(),
                last_opened_ms: ms,
                pinned: false,
            }
        }

        /// The MRU must not still be sitting in the thread-local when `main`
        /// returns. Dropping an `MruList` blocks until the shared settings-writer
        /// thread acks its final flush, and a thread-local's destructor runs
        /// during process teardown — on Windows inside `DLL_PROCESS_DETACH`,
        /// after `ExitProcess` has already killed that writer. The ack never
        /// comes and the app hangs forever, unkillable. `shutdown` is what keeps
        /// the drop inside `main`, where the writer can still answer.
        #[test]
        fn shutdown_releases_the_thread_local_mru() {
            let toml = tmp("recents_shutdown.toml");
            let _ = std::fs::remove_file(&toml);
            let mru = MruList::open_at(toml, 30, Duration::ZERO).unwrap();
            SHARED_MRU.with(|cell| *cell.borrow_mut() = Some(mru));

            RecentWorkListModel::shutdown();

            SHARED_MRU.with(|cell| {
                assert!(
                    cell.borrow().is_none(),
                    "shutdown must take the MRU out of the thread-local, or its \
                     writer's drop lands in a TLS destructor at process teardown \
                     and wedges the exit"
                )
            });
        }

        #[test]
        fn unreachable_entries_are_hidden_but_kept_in_the_mru() {
            // One reachable project file, one missing path.
            let reachable = tmp("reachable.skrib");
            std::fs::write(&reachable, b"x").unwrap();
            let missing = tmp("missing.skrib");
            let _ = std::fs::remove_file(&missing);

            let toml = tmp("recents_hide.toml");
            let _ = std::fs::remove_file(&toml);
            let mru = MruList::open_at(toml.clone(), 30, Duration::ZERO).unwrap();
            mru.add(entry(&missing.to_string_lossy(), "Gone", 1));
            mru.add(entry(&reachable.to_string_lossy(), "Here", 2));

            let rows = visible_rows(&Some(mru.clone()));
            // Only the reachable entry is visible…
            assert_eq!(
                rows.len(),
                1,
                "unreachable entry must be hidden from the UI"
            );
            assert_eq!(rows[0].absolute_path, reachable.to_string_lossy());
            // …but the unreachable one stays in the durable MRU.
            assert_eq!(
                mru.model().len(),
                2,
                "unreachable entry must remain in the MRU, not be dropped"
            );

            let _ = std::fs::remove_file(&reachable);
            let _ = std::fs::remove_file(&toml);
        }

        #[test]
        fn raw_paths_includes_unreachable_entries_unlike_visible_rows() {
            // One reachable, one missing — mirrors
            // `unreachable_entries_are_hidden_but_kept_in_the_mru` above, but
            // asserts the OPPOSITE property for the raw reader: F4b's window-state
            // prune must not mistake "temporarily unreachable" for "orphaned".
            let reachable = tmp("raw_paths_reachable.skrib");
            std::fs::write(&reachable, b"x").unwrap();
            let missing = tmp("raw_paths_missing.skrib");
            let _ = std::fs::remove_file(&missing);

            let toml = tmp("recents_raw_paths.toml");
            let _ = std::fs::remove_file(&toml);
            let mru = MruList::open_at(toml.clone(), 30, Duration::ZERO).unwrap();
            mru.add(entry(&missing.to_string_lossy(), "Gone", 1));
            mru.add(entry(&reachable.to_string_lossy(), "Here", 2));

            let paths = raw_paths(&mru);
            assert_eq!(
                paths.len(),
                2,
                "raw_paths must include the unreachable entry too, unlike visible_rows"
            );
            assert!(paths.contains(&missing.to_string_lossy().into_owned()));
            assert!(paths.contains(&reachable.to_string_lossy().into_owned()));

            // Contrast with the UI-filtered view, which does hide the unreachable
            // one — this is exactly the difference that matters for F4b.
            let visible = visible_rows(&Some(mru.clone()));
            assert_eq!(visible.len(), 1, "visible_rows still hides the missing one");

            let _ = std::fs::remove_file(&reachable);
            let _ = std::fs::remove_file(&toml);
        }

        #[test]
        fn add_dedupes_by_path_and_moves_to_front() {
            let toml = tmp("recents_dedup.toml");
            let _ = std::fs::remove_file(&toml);
            let mru = MruList::open_at(toml.clone(), 30, Duration::ZERO).unwrap();
            mru.add(entry("/a.skrib", "A", 1));
            mru.add(entry("/b.skrib", "B", 2));
            mru.add(entry("/a.skrib", "A-again", 3));

            // key() == path, so re-adding /a dedupes and moves it to the front.
            assert_eq!(mru.model().len(), 2, "same path must not duplicate");
            let front = mru.model().with_item(0, |e| e.path.clone()).unwrap();
            assert_eq!(front, "/a.skrib", "re-added entry moves to most-recent");

            let _ = std::fs::remove_file(&toml);
        }

        // End-to-end, GUI-free proof of *cross-restart* recents: open a work in the
        // backend, derive the MRU entry the way `on_open` does, persist it, then
        // reopen the file from disk (simulating a fresh launch) and confirm the
        // recent is still there and visible.
        #[test]
        fn opened_work_persists_across_a_simulated_restart() {
            let proj = tmp("proj.skrib");
            std::fs::write(&proj, b"x").unwrap(); // reachable on disk
            let toml = tmp("recents_restart.toml");
            let _ = std::fs::remove_file(&toml);

            // Open a work through the real backend, exactly as the app does.
            let ctx = frontend::AppContext::new();
            handling_app_lifecycle_commands::initialize_app(&ctx).unwrap();
            work_management_commands::new_work(
                &ctx,
                &NewWorkDto {
                    file_name: proj.to_string_lossy().into_owned(),
                    is_folder: false,
                    template_kind: NewWorkTemplate::Novel,
                    labels: vec![],
                    language: vec!["en".to_string()],
                    author_name: String::new(),
                    chapter_scene_mode: false,
                },
            )
            .unwrap();

            // `on_open` reads the opened work into an entry — assert that mapping.
            // `wire`'s real subscriber gets `work_id` off the `NewWork` event itself;
            // here (no event plumbing in a headless test) fetch the one Work the
            // fresh in-memory store holds, exactly as that event's payload would.
            let work_id = work_commands::get_all_work(&ctx).unwrap().into_iter().next().unwrap().id;
            let entry = opened_entry(&ctx, work_id).expect("an open work yields an MRU entry");
            assert_eq!(entry.path, proj.to_string_lossy());

            // Persist it, then drop the list (simulates process exit).
            {
                let mru = MruList::open_at(toml.clone(), 30, Duration::ZERO).unwrap();
                mru.add(entry);
                mru.flush_now().unwrap();
            }

            // Reopen from disk (simulates the next launch): the recent survived and
            // is visible because its file is still reachable.
            let mru = MruList::open_at(toml.clone(), 30, Duration::ZERO).unwrap();
            let rows = visible_rows(&Some(mru));
            assert_eq!(rows.len(), 1, "recent must persist across a restart");
            assert_eq!(rows[0].absolute_path, proj.to_string_lossy());

            let _ = std::fs::remove_file(&proj);
            let _ = std::fs::remove_file(&toml);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::data::ListModel;
    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::direct_access::RecentWorkDto;

    #[derive(Clone)]
    pub struct RecentWorkListModel {
        /// Single source of truth, mirroring the real variant — one fabricated
        /// row so the mock title bar and Welcome list render.
        model: ListModel<RecentWorkDto>,
        version: Signal<u64>,
    }

    impl RecentWorkListModel {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            Self {
                model: ListModel::from_vec(vec![RecentWorkDto {
                    id: 1,
                    title: "Mock Project".to_string(),
                    absolute_path: "/mock/Mock Project.skrib".to_string(),
                    ..Default::default()
                }]),
                version: Signal::new(0),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn list_model(&self) -> ListModel<RecentWorkDto> {
            self.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.version.clone()
        }

        pub fn items(&self) -> Vec<RecentWorkDto> {
            (0..self.model.len())
                .filter_map(|i| self.model.with_item(i, |d| d.clone()))
                .collect()
        }

        /// No persisted recents file in the mock build — nothing to protect from
        /// the window-state prune (`main.rs`), so nothing to report.
        pub(crate) fn all_raw_paths() -> Vec<String> {
            Vec::new()
        }

        /// No persistence in the mock build.
        pub fn shutdown() {}
    }
}

pub use imp::RecentWorkListModel;
