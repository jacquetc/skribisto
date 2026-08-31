// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleDictWord` — a reactive read+write handle over one `DictWord`.
//!
//! Caches the `DictWordDto` reactively (refreshed on the entity's `Updated`
//! event) and exposes its `word`; the one write is [`rename`](imp::SingleDictWord::rename),
//! which the Settings pane's inline word editor commits through. Collection adds
//! and removes are [`DictWordListModel`](crate::models::DictWordListModel)'s job;
//! this is the per-entity read/write half (the precedent is `SingleContent`).
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface. See
//! [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::dict_word_commands;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::{DictWordDto, UpdateDictWordDto};

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<DictWordDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleDictWord {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleDictWord {
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

        /// Point the handle at an id (or clear it) and load synchronously — usable
        /// both as a bound handle and a one-shot write probe.
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

        /// Auto-refresh when this `DictWord` changes elsewhere. Call from `build`
        /// (every build — see [`SingleBinderItem::wire`](crate::singles::SingleBinderItem)).
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::DictWord(EntityEvent::Updated)),
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

        pub fn dto(&self) -> Option<DictWordDto> {
            self.inner.dto.get()
        }

        /// The word as a reactive signal (empty when unloaded).
        pub fn word(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.word.clone()).unwrap_or_default())
        }

        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }

        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }

        /// Rename the word (a scalar-only update of the full DTO). A no-op when the
        /// text is unchanged. Undoable on `stack`.
        pub fn rename(&self, new_word: &str, stack: Option<u64>) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleDictWord: no id");
            };
            let Some(dto) = self.dto() else {
                anyhow::bail!("SingleDictWord: word {id} not loaded");
            };
            if dto.word == new_word {
                return Ok(());
            }
            let update = UpdateDictWordDto {
                id,
                created_at: dto.created_at,
                updated_at: chrono::Utc::now(),
                word: new_word.to_string(),
            };
            dict_word_commands::update_dict_word(&self.inner.ctx, stack, &update)?;
            self.refresh();
            Ok(())
        }

        fn refresh(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            self.inner.loading_status.set(LoadingStatus::Loading);
            match dict_word_commands::get_dict_word(&self.inner.ctx, &id) {
                Ok(Some(w)) => {
                    self.inner.dto.set(Some(w));
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

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::direct_access::DictWordDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<DictWordDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
    }

    #[derive(Clone)]
    pub struct SingleDictWord {
        inner: Rc<Inner>,
    }

    fn mock_dto(id: u64) -> DictWordDto {
        DictWordDto {
            id,
            word: format!("Word{id}"),
            ..Default::default()
        }
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleDictWord {
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

        pub fn dto(&self) -> Option<DictWordDto> {
            self.inner.dto.get()
        }

        pub fn word(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.word.clone()).unwrap_or_default())
        }

        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }

        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }

        pub fn rename(&self, new_word: &str, _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.word = new_word.to_string();
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }
    }
}

pub use imp::SingleDictWord;
