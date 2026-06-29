//! Reactive list model over the recently-opened projects (`RecentWork`).
//!
//! Self-subscribes to `LoadWork` and bumps a `version` signal so a consumer
//! bound at `BindingLevel::Rebuild` re-derives its list when a project opens —
//! replacing the hand-rolled `version` + `subscribed` guard that previously
//! lived in `RecentProjectsButton`. Two `#[cfg]`-gated `mod imp` variants share
//! one public surface (the model is small, so it uses the full two-`mod imp`
//! shape; see the convention note in `models.rs`).

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::recent_work_commands;
    use frontend::common::event::{Event, Origin, WorkManagementEvent};
    use frontend::direct_access::RecentWorkDto;

    struct Inner {
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
            Self {
                inner: Rc::new(Inner {
                    version: Signal::new(0),
                    subscribed: Cell::new(false),
                    ctx,
                }),
            }
        }

        /// Subscribe (once) so the list refreshes on each project load. Call from
        /// the consumer widget's `build`.
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
            let version = self.inner.version.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| version.set(version.get().wrapping_add(1)),
            );
        }

        /// Bump this on `LoadWork`; bind it at `BindingLevel::Rebuild` to refresh.
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// The recent works, most-recently-opened first.
        pub fn items(&self) -> Vec<RecentWorkDto> {
            let mut recents =
                recent_work_commands::get_all_recent_work(&self.inner.ctx).unwrap_or_default();
            recents.sort_by_key(|b| std::cmp::Reverse(b.last_opened_at));
            recents
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::direct_access::RecentWorkDto;

    #[derive(Clone)]
    pub struct RecentWorkListModel {
        version: Signal<u64>,
    }

    impl RecentWorkListModel {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            Self {
                version: Signal::new(0),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn version_signal(&self) -> Signal<u64> {
            self.version.clone()
        }

        /// One fabricated recent project so the mock title bar renders.
        pub fn items(&self) -> Vec<RecentWorkDto> {
            vec![RecentWorkDto {
                id: 1,
                title: "Mock Project".to_string(),
                absolute_path: "/mock/Mock Project.skrib".to_string(),
                ..Default::default()
            }]
        }
    }
}

pub use imp::RecentWorkListModel;
