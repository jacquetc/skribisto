//! Reactive list model over the recently-opened works (`RecentWork`).
//!
//! Owns a `bastyde::data::ListModel<RecentWorkDto>` — the **simple** in-memory
//! reactive list model — as its single source of truth. A `ListView` bound to
//! it gets `Role::List` / `Role::ListItem` accessibility, keyboard navigation
//! and incremental updates *for free*, instead of a hand-rolled row loop that
//! drops all of that. The real variant self-subscribes to `LoadWork` and
//! `replace_all`s the model on each open; the mock variant ships one fabricated
//! row.
//!
//! Two extra views over that one model serve the non-`ListView` consumer (the
//! title-bar `RecentProjectsButton`, a `MenuList` popover that rebuilds itself
//! rather than observing the model):
//! - [`version_signal`](RecentWorkListModel::version_signal) — bumped on every
//!   refresh, bound at `BindingLevel::Rebuild` to re-derive that popover.
//! - [`items`](RecentWorkListModel::items) — a `Vec` snapshot of the model.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface (the model is
//! small, so it uses the full two-`mod imp` shape; see the convention note in
//! `models.rs`).

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::data::ListModel;
    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::recent_work_commands;
    use frontend::common::event::{Event, Origin, WorkManagementEvent};
    use frontend::direct_access::RecentWorkDto;

    struct Inner {
        /// Single source of truth — the reactive list a `ListView` binds to.
        model: ListModel<RecentWorkDto>,
        /// Bumped on every refresh, for `Rebuild`-bound (non-`ListView`) consumers.
        version: Signal<u64>,
        /// One-shot guard: subscriptions persist across the consumer's rebuilds,
        /// so subscribe exactly once.
        subscribed: Cell<bool>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct RecentWorkListModel {
        inner: Rc<Inner>,
    }

    impl RecentWorkListModel {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            // Populate eagerly so the model is never observed empty: by the time
            // these widgets are constructed (in `App::build`) the lifecycle has
            // already seeded the `RecentWork` list into the store.
            let model = ListModel::from_vec(query_sorted(&ctx));
            Self {
                inner: Rc::new(Inner {
                    model,
                    version: Signal::new(0),
                    subscribed: Cell::new(false),
                    ctx,
                }),
            }
        }

        /// Subscribe (once) so the list refreshes on each work load. Call from
        /// the consumer widget's `build`. Refresh runs only on the *event*, never
        /// during a build, so it can't churn a `ListView` observing the model.
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
            let me = self.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| me.refresh(),
            );
        }

        /// The reactive model to bind a `ListView` to.
        pub fn list_model(&self) -> ListModel<RecentWorkDto> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh; bind at `BindingLevel::Rebuild` for consumers
        /// that rebuild rather than observe the model (the dropdown button).
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// `Vec` snapshot of the works, most-recently-opened first — for
        /// non-`ListView` consumers that build their own rows.
        pub fn items(&self) -> Vec<RecentWorkDto> {
            snapshot(&self.inner.model)
        }

        /// Re-query the backend and replace the model contents (most-recent
        /// first), then bump `version` for the rebuild-driven consumers.
        fn refresh(&self) {
            self.inner.model.replace_all(query_sorted(&self.inner.ctx));
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// Recent works from the backend, most-recently-opened first.
    fn query_sorted(ctx: &AppContext) -> Vec<RecentWorkDto> {
        let mut recents = recent_work_commands::get_all_recent_work(ctx).unwrap_or_default();
        recents.sort_by_key(|b| std::cmp::Reverse(b.last_opened_at));
        recents
    }

    /// Materialize the model's current rows into a `Vec`.
    fn snapshot(model: &ListModel<RecentWorkDto>) -> Vec<RecentWorkDto> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |d| d.clone()))
            .collect()
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
    }
}

pub use imp::RecentWorkListModel;
