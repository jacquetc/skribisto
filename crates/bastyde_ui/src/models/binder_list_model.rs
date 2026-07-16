// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list of the open Work's binders — the data behind the binder
//! switcher popover (id, display name, activated-item count).
//!
//! A read-only Layer-A model, keyed on the open Work via the `work_id` signal
//! from [`AppIds`](crate::app_ids) (ids-only global state). Unlike
//! [`RecentWorkListModel`](crate::models::RecentWorkListModel) there is no
//! `ListView` consumer — the switcher is a `MenuList` popover that rebuilds
//! itself — so the model keeps no `bastyde::data::ListModel`; it exposes a
//! `Vec` snapshot ([`items`](imp::BinderListModel::items)) computed fresh from
//! the backend and a [`version`](imp::BinderListModel::version_signal) signal
//! bumped on every relevant backend event, bound at `BindingLevel::Rebuild` so
//! the switcher re-derives its list when binders are added / trashed / renamed
//! or items are added / removed (changing a count).
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface (see the
//! convention note in [`crate::models`]).

/// One binder row for the switcher popover.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinderRow {
    pub id: u64,
    pub name: String,
    /// Number of **activated** (non-trashed) items in the binder.
    pub item_count: usize,
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin,
        TrashManagementEvent, WorkManagementEvent,
    };

    use super::BinderRow;

    struct Inner {
        /// Bumped on every refresh, for `Rebuild`-bound consumers.
        version: Signal<u64>,
        /// One-shot guard: subscriptions persist across the consumer's rebuilds.
        subscribed: Cell<bool>,
        ctx: Rc<AppContext>,
        /// The open Work id (ids-only global state) — the binders' owner.
        work_id: Signal<Option<u64>>,
    }

    #[derive(Clone)]
    pub struct BinderListModel {
        inner: Rc<Inner>,
    }

    impl BinderListModel {
        pub fn new(ctx: Rc<AppContext>, work_id: Signal<Option<u64>>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    version: Signal::new(0),
                    subscribed: Cell::new(false),
                    ctx,
                    work_id,
                }),
            }
        }

        /// Subscribe (once) so the list refreshes when binders or their item
        /// counts change. Call from the consumer widget's `build`.
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
            use DirectAccessEntity::{Binder, BinderItem};
            use EntityEvent::{Created, Removed, Updated};
            let origins = [
                // Binder set / names.
                Origin::DirectAccess(Binder(Created)),
                Origin::DirectAccess(Binder(Updated)),
                Origin::DirectAccess(Binder(Removed)),
                // Item counts.
                Origin::DirectAccess(BinderItem(Created)),
                Origin::DirectAccess(BinderItem(Removed)),
                Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
                // Trash / restore (soft-delete flips `activated`).
                Origin::TrashManagement(TrashManagementEvent::TrashBinder),
                Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
                Origin::TrashManagement(TrashManagementEvent::RestoreItems),
                Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
                // Project (re)load.
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                Origin::WorkManagement(WorkManagementEvent::NewWork),
            ];
            for origin in origins {
                let me = self.clone();
                ctx.subscribe_event(origin, move |_event: &Event| me.refresh());
            }
        }

        /// Bumped on each refresh; bind at `BindingLevel::Rebuild`.
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Fresh snapshot of the open Work's activated binders, in Work order.
        pub fn items(&self) -> Vec<BinderRow> {
            query(&self.inner.ctx, &self.inner.work_id)
        }

        fn refresh(&self) {
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// The open Work's activated binders (id, name, activated-item count).
    fn query(ctx: &AppContext, work_id: &Signal<Option<u64>>) -> Vec<BinderRow> {
        let Some(work_id) = work_id.get() else {
            return Vec::new(); // no project open
        };
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        let mut out = Vec::new();
        for binder_id in binder_ids {
            let Ok(Some(binder)) = binder_commands::get_binder(ctx, &binder_id) else {
                continue;
            };
            if !binder.activated {
                continue; // trashed binders are hidden
            }
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let item_count = binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .filter(|it| it.activated)
                .count();
            out.push(BinderRow {
                id: binder_id,
                name: binder.name,
                item_count,
            });
        }
        out
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;

    use super::BinderRow;

    #[derive(Clone)]
    pub struct BinderListModel {
        version: Signal<u64>,
    }

    impl BinderListModel {
        pub fn new(_ctx: Rc<AppContext>, _work_id: Signal<Option<u64>>) -> Self {
            Self {
                version: Signal::new(0),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn version_signal(&self) -> Signal<u64> {
            self.version.clone()
        }

        /// Mirrors the mock binder tree (`Manuscript` = 5 items, `Notes` = 2).
        pub fn items(&self) -> Vec<BinderRow> {
            vec![
                BinderRow {
                    id: 1,
                    name: "Manuscript".to_string(),
                    item_count: 5,
                },
                BinderRow {
                    id: 2,
                    name: "Notes".to_string(),
                    item_count: 2,
                },
            ]
        }
    }
}

pub use imp::BinderListModel;
