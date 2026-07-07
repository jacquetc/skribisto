//! Reactive list model over the recently-opened works — backed by a **persisted**
//! `bastyde::settings::MruList`, so the list **survives app restarts**.
//!
//! The durable source is one `MruList<RecentEntry>` at
//! `<config_dir>/recents.toml` (dedupe-by-path, capped, most-recently-opened
//! first, debounced writes). The public surface is unchanged: a
//! `bastyde::data::ListModel<RecentWorkDto>` a `ListView` binds to, plus a
//! `version` signal + `items()` snapshot for the non-`ListView` consumer (the
//! title-bar `RecentProjectsButton`).
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
    use bastyde::settings::{AppPaths, MruEntry, MruList};
    use serde::{Deserialize, Serialize};

    use frontend::AppContext;
    use frontend::commands::{work_commands, work_info_commands};
    use frontend::common::event::{Event, Origin, WorkManagementEvent};
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

    impl MruEntry for RecentEntry {
        type Key = str;
        fn key(&self) -> &str {
            &self.path
        }
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
                }),
            }
        }

        /// Subscribe (once) so the list records + refreshes on each work open.
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
            let me = self.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| me.on_open(),
            );
            // A new project is also a "recently opened" work.
            let me = self.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |_event: &Event| me.on_open(),
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

        /// Flush the shared recents MRU to disk synchronously. Call once at app
        /// shutdown so a just-opened work isn't lost inside the debounce window.
        pub fn flush_now() {
            if let Some(mru) = shared_mru() {
                if let Err(e) = mru.flush_now() {
                    eprintln!("recents MRU: flush failed: {e}");
                }
            }
        }

        /// On a successful open, record the just-opened work in the durable MRU
        /// (dedupe by path, front-inserted), then refresh the reachable view.
        fn on_open(&self) {
            if let (Some(mru), Some(entry)) = (&self.inner.mru, opened_entry(&self.inner.ctx)) {
                mru.add(entry);
            }
            self.refresh();
        }

        /// Re-derive the reachable view from the MRU, then bump `version`.
        fn refresh(&self) {
            self.inner.model.replace_all(visible_rows(&self.inner.mru));
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// The just-opened work as an MRU entry: its on-disk path + title. `None` when
    /// no work is open or it has no file path yet.
    fn opened_entry(ctx: &AppContext) -> Option<RecentEntry> {
        let path = work_info_commands::get_all_work_info(ctx)
            .ok()?
            .into_iter()
            .next()?
            .file_name?;
        let title = work_commands::get_all_work(ctx)
            .ok()
            .and_then(|works| works.into_iter().next())
            .map(|w| w.title)
            .unwrap_or_default();
        Some(RecentEntry {
            path,
            title,
            last_opened_ms: now_ms(),
            pinned: false,
        })
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
            assert_eq!(rows.len(), 1, "unreachable entry must be hidden from the UI");
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
                    language: "en".to_string(),
                    chapter_scene_mode: false,
                },
            )
            .unwrap();

            // `on_open` reads the opened work into an entry — assert that mapping.
            let entry = opened_entry(&ctx).expect("an open work yields an MRU entry");
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

        /// No persistence in the mock build.
        pub fn flush_now() {}
    }
}

pub use imp::RecentWorkListModel;
