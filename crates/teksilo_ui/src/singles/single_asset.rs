// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleAsset` — a reactive read handle over one `Asset`, a picture of the project.
//!
//! Caches the `AssetDto` (refreshed on the entity's `Updated` event, cleared on
//! `Removed`). The picture's bytes are not here and never in the store: they live in the
//! project's media directory, addressed by `content_hash`. Its one reader today is a
//! Book's cover ([`crate::book_cover`]).
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface. See
//! [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::asset::dtos::AssetDto;
    use frontend::commands::asset_commands;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<AssetDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleAsset {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleAsset {
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

        /// Point the handle at an id (or clear it) and load synchronously.
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

        /// Refresh when this picture's row changes, and clear when it is removed. Call
        /// from `build` (every build: subscriptions are scoped to the build that made them).
        pub fn wire(&self, ctx: &mut BuildContext) {
            for (event, removed) in [(EntityEvent::Updated, false), (EntityEvent::Removed, true)] {
                let s = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::Asset(event)),
                    move |event: &Event| {
                        let Some(id) = s.inner.id.get() else {
                            return;
                        };
                        if !event.ids.contains(&id) {
                            return;
                        }
                        if removed {
                            s.clear();
                        } else {
                            s.refresh();
                        }
                    },
                );
            }
        }

        pub fn dto(&self) -> Option<AssetDto> {
            self.inner.dto.get()
        }

        pub fn dto_signal(&self) -> Signal<Option<AssetDto>> {
            self.inner.dto.clone()
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
            match asset_commands::get_asset(&self.inner.ctx, &id) {
                Ok(Some(a)) => {
                    self.inner.dto.set(Some(a));
                    self.inner.error_message.set(String::new());
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Ok(None) => self.clear(),
                Err(e) => {
                    self.inner.error_message.set(e.to_string());
                    self.inner.loading_status.set(LoadingStatus::Error);
                }
            }
        }

        fn clear(&self) {
            self.inner.dto.set(None);
            self.inner.error_message.set(String::new());
            self.inner.loading_status.set(LoadingStatus::Unloaded);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::asset::dtos::AssetDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<AssetDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
    }

    #[derive(Clone)]
    pub struct SingleAsset {
        inner: Rc<Inner>,
    }

    /// A fabricated picture row. The mocks build has no media directory, so no surface
    /// can show its bytes; a reader sees the row and reports the file as missing.
    fn mock_dto(id: u64) -> AssetDto {
        AssetDto {
            id,
            content_hash: format!("mock{id}"),
            file_name: format!("picture{id}.png"),
            mime_type: "image/png".to_string(),
            width: 1600,
            height: 2400,
            ..Default::default()
        }
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleAsset {
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

        pub fn dto(&self) -> Option<AssetDto> {
            self.inner.dto.get()
        }

        pub fn dto_signal(&self) -> Signal<Option<AssetDto>> {
            self.inner.dto.clone()
        }

        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }

        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
    }
}

pub use imp::SingleAsset;
