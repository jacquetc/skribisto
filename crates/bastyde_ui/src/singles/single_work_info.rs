//! `SingleWorkInfo` — a reactive handle over the open project's `WorkInfo`.
//!
//! `WorkInfo` records the on-disk identity of the open project: its `file_name`
//! and its `WorkShape` (`Zip` single-file vs exploded `Folder`). This single
//! drives the title-bar "Save as…" menu visibility (Bug 2) and the Save-as
//! default name. `shape` is `Option` — `None` means no project is open, which
//! collapses both conditional menu items.
//!
//! It refreshes on `WorkInfo` `Updated` events and on the two migrate events
//! (which flip the shape in place without changing the id). Two `mod imp`
//! variants share one public surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::work_info_commands;
    use frontend::common::entities::WorkShape;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, WorkManagementEvent,
    };
    use frontend::direct_access::UpdateWorkInfoDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        shape: Signal<Option<WorkShape>>,
        file_name: Signal<Option<String>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
        is_refreshing: Cell<bool>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleWorkInfo {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleWorkInfo {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    shape: Signal::new(None),
                    file_name: Signal::new(None),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                    is_refreshing: Cell::new(false),
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

        /// Subscribe to `WorkInfo` `Updated` (id-matched) and the two migrate
        /// events (which flip the shape in place). Call once from a long-lived
        /// widget's `build`.
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::WorkInfo(EntityEvent::Updated)),
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
            // A Save As rewrites WorkInfo.shape/file_name for the *same* id —
            // re-read it so the "Save as…" menu flips immediately after a save.
            {
                let s = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::SaveAs),
                    move |_event: &Event| {
                        if s.inner.id.get().is_some() {
                            s.refresh();
                        }
                    },
                );
            }
        }

        // ── reactive accessors ──
        pub fn shape(&self) -> Signal<Option<WorkShape>> {
            self.inner.shape.clone()
        }
        pub fn file_name(&self) -> Signal<Option<String>> {
            self.inner.file_name.clone()
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
        pub fn dirty(&self) -> Signal<bool> {
            self.inner.dirty.clone()
        }

        fn refresh(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            self.inner.loading_status.set(LoadingStatus::Loading);
            match work_info_commands::get_work_info(&self.inner.ctx, &id) {
                Ok(Some(wi)) => {
                    self.inner.is_refreshing.set(true);
                    self.inner.shape.set(Some(wi.shape));
                    self.inner.file_name.set(wi.file_name);
                    self.inner.is_refreshing.set(false);
                    self.inner.dirty.set(false);
                    self.inner.error_message.set(String::new());
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Ok(None) => self.clear(),
                Err(e) => self.fail(&e.to_string()),
            }
        }

        /// Persist `file_name` / `shape` edits (no-op when clean or unset).
        /// `WorkInfo` is non-undoable, so there is no stack.
        pub fn save(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            if !self.inner.dirty.get() {
                return;
            }
            let ctx = &*self.inner.ctx;
            let Ok(Some(cur)) = work_info_commands::get_work_info(ctx, &id) else {
                return self.fail("WorkInfo not found");
            };
            let Some(shape) = self.inner.shape.get() else {
                return;
            };
            let dto = UpdateWorkInfoDto {
                id,
                created_at: cur.created_at,
                updated_at: chrono::Utc::now(),
                file_name: self.inner.file_name.get(),
                shape,
            };
            match work_info_commands::update_work_info(ctx, &dto) {
                Ok(_) => {
                    self.inner.dirty.set(false);
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Err(e) => self.fail(&e.to_string()),
            }
        }

        fn clear(&self) {
            self.inner.is_refreshing.set(true);
            self.inner.shape.set(None);
            self.inner.file_name.set(None);
            self.inner.is_refreshing.set(false);
            self.inner.dirty.set(false);
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
    use frontend::common::entities::WorkShape;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        shape: Signal<Option<WorkShape>>,
        file_name: Signal<Option<String>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
    }

    #[derive(Clone)]
    pub struct SingleWorkInfo {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleWorkInfo {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            // A zip-shaped mock project so "Save as folder…" shows in the mock UI.
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(Some(1)),
                    shape: Signal::new(Some(WorkShape::Zip)),
                    file_name: Signal::new(Some("Mock Project.skrib".to_string())),
                    loading_status: Signal::new(LoadingStatus::Loaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                }),
            }
        }

        pub fn set_id(&self, id: Option<u64>) {
            self.inner.id.set(id);
        }
        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn shape(&self) -> Signal<Option<WorkShape>> {
            self.inner.shape.clone()
        }
        pub fn file_name(&self) -> Signal<Option<String>> {
            self.inner.file_name.clone()
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
        pub fn dirty(&self) -> Signal<bool> {
            self.inner.dirty.clone()
        }

        pub fn save(&self) {
            self.inner.dirty.set(false);
            self.inner.loading_status.set(LoadingStatus::Loaded);
        }
    }
}

pub use imp::SingleWorkInfo;

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::common::entities::WorkShape;

    use super::SingleWorkInfo;

    /// Bug 2: a zip-shaped project must drive "Save as folder" visible.
    #[test]
    fn mock_reports_zip_shape() {
        let s = SingleWorkInfo::new(Rc::new(AppContext::new()));
        assert_eq!(s.shape().get(), Some(WorkShape::Zip));
    }
}
