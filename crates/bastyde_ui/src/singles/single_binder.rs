// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleBinder` — a reactive **read** handle over one `Binder`.
//!
//! Caches the full `BinderDto` reactively (refreshed on `Binder` `Updated`
//! events) and exposes it via [`dto`](imp::SingleBinder::dto) + a `name` signal.
//! Like [`SingleBinderItem`](crate::singles::SingleBinderItem) it is read-only —
//! binder renames are undoable tree mutations driven by `OutlineViewModel`'s
//! commands; this single is the source of the binder's current state when those
//! commands build their update DTO. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::binder_commands;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::BinderDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<BinderDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleBinder {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleBinder {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    dto: Signal::new(None),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                    ctx,
                }),
            }
        }

        pub fn set_id(&self, id: Option<u64>) {
            self.inner.id.set(id);
            match id {
                Some(_) => self.refresh(),
                None => self.clear(),
            }
        }

        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }

        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Binder(EntityEvent::Updated)),
                move |event: &Event| {
                    if s.inner
                        .id
                        .get()
                        .map(|id| event.ids.contains(&id))
                        .unwrap_or(false)
                    {
                        s.refresh();
                    }
                },
            );
        }

        pub fn dto(&self) -> Option<BinderDto> {
            self.inner.dto.get()
        }
        pub fn dto_signal(&self) -> Signal<Option<BinderDto>> {
            self.inner.dto.clone()
        }
        pub fn name(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.name.clone()).unwrap_or_default())
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }

        fn refresh(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            self.inner.loading_status.set(LoadingStatus::Loading);
            match binder_commands::get_binder(&self.inner.ctx, &id) {
                Ok(Some(b)) => {
                    self.inner.dto.set(Some(b));
                    self.inner.error_message.set(String::new());
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Ok(None) => self.clear(),
                Err(e) => self.fail(&e.to_string()),
            }
        }

        fn clear(&self) {
            self.inner.dto.set(None);
            self.inner.error_message.set(String::new());
            self.inner.loading_status.set(LoadingStatus::Unloaded);
        }

        fn fail(&self, msg: &str) {
            self.inner.error_message.set(msg.to_string());
            self.inner.loading_status.set(LoadingStatus::Error);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::direct_access::BinderDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<BinderDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
    }

    #[derive(Clone)]
    pub struct SingleBinder {
        inner: Rc<Inner>,
    }

    fn mock_dto(id: u64) -> BinderDto {
        BinderDto {
            id,
            // DETERMINISTIC, not `new_uid()`: `mock_dto` is called on demand,
            // so a fresh random uid each call would make the same row change
            // identity between refreshes — anything keyed by uid would treat
            // every refresh as a new row. Offset so a mock binder and a mock
            // item with the same id never collide.
            uid: common::uid::fixture_uid(1_000_000 + id),
            name: "Mock Binder".to_string(),
            activated: true,
            ..Default::default()
        }
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleBinder {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    dto: Signal::new(None),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                }),
            }
        }

        pub fn set_id(&self, id: Option<u64>) {
            self.inner.id.set(id);
            match id {
                Some(i) => {
                    self.inner.dto.set(Some(mock_dto(i)));
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                None => {
                    self.inner.dto.set(None);
                    self.inner.loading_status.set(LoadingStatus::Unloaded);
                }
            }
        }
        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn dto(&self) -> Option<BinderDto> {
            self.inner.dto.get()
        }
        pub fn dto_signal(&self) -> Signal<Option<BinderDto>> {
            self.inner.dto.clone()
        }
        pub fn name(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.name.clone()).unwrap_or_default())
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
    }
}

pub use imp::SingleBinder;
