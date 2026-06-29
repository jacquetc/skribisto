//! `SingleBinderItem` — a reactive **read** handle over one `BinderItem`.
//!
//! Caches the full `BinderItemDto` reactively (refreshed on the entity's
//! `Updated` events) and exposes it via [`dto`](imp::SingleBinderItem::dto) +
//! mapped field signals. Unlike [`SingleWork`](crate::singles::SingleWork) it has
//! no `save()`: binder-item edits (rename, reindent, move, trash) are tree
//! mutations that must run on the per-`Work` undo stack, so they go through
//! `OutlineViewModel`'s undoable commands — this single is the **read** half,
//! used both for the open tab's `(role, sub_role)` dispatch and as the source of
//! the current item state when those commands build their update DTO. Two
//! `mod imp` variants share one public surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::binder_item_commands;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::BinderItemDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<BinderItemDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleBinderItem {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleBinderItem {
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

        /// Point the handle at an id (or clear it) and load the entity. Reads are
        /// synchronous, so the cached `dto` is current as soon as this returns —
        /// usable both as a persistent bound handle and as a one-shot probe.
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

        /// Auto-refresh when this `BinderItem` changes elsewhere. Call once from a
        /// long-lived widget's `build` for a persistently-bound handle; one-shot
        /// probes need not wire (each `set_id` reloads synchronously).
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
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

        /// The cached full DTO (clone), if loaded.
        pub fn dto(&self) -> Option<BinderItemDto> {
            self.inner.dto.get()
        }
        /// The reactive whole-entity signal — bind UI to derived views of it.
        pub fn dto_signal(&self) -> Signal<Option<BinderItemDto>> {
            self.inner.dto.clone()
        }
        /// The item's title as a reactive signal (empty when unloaded).
        pub fn title(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.title.clone()).unwrap_or_default())
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
            match binder_item_commands::get_binder_item(&self.inner.ctx, &id) {
                Ok(Some(it)) => {
                    self.inner.dto.set(Some(it));
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
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::BinderItemDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<BinderItemDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
    }

    #[derive(Clone)]
    pub struct SingleBinderItem {
        inner: Rc<Inner>,
    }

    /// A fabricated Scene item carrying the requested id (so the mock UI opens a
    /// prose tab for any clicked row).
    fn mock_dto(id: u64) -> BinderItemDto {
        BinderItemDto {
            id,
            title: "Mock Item".to_string(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            is_printable: true,
            ..Default::default()
        }
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleBinderItem {
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

        pub fn dto(&self) -> Option<BinderItemDto> {
            self.inner.dto.get()
        }
        pub fn dto_signal(&self) -> Signal<Option<BinderItemDto>> {
            self.inner.dto.clone()
        }
        pub fn title(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.title.clone()).unwrap_or_default())
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
    }
}

pub use imp::SingleBinderItem;
