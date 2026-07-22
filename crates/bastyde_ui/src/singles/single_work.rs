// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleWork` — a reactive handle over the open `Work` entity.
//!
//! Holds one `Work` by id; exposes `title` / `author_name` / `dict_language` as
//! `Signal`s plus `loading_status` / `error_message` / `dirty`; refreshes from
//! the backend on the entity's `Updated` events; writes field edits back via
//! `save()` (on the shared per-`Work` undo stack, so metadata edits are
//! undoable). Two `mod imp` variants (real backend / fabricated mock) share an
//! identical public surface — no `#[cfg]` leaks into consumers. See
//! [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::work_commands;
    use frontend::common::entities::ChapterMode;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::UpdateWorkDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        title: Signal<String>,
        author_name: Signal<String>,
        dict_language: Signal<Vec<String>>,
        chapter_mode: Signal<ChapterMode>,
        /// The per-project master switch for the custom text-replacement lexicon.
        custom_replacement_rules_enabled: Signal<bool>,
        /// The stable per-project UUID (`Work.unique_id`). Read-only here — it is
        /// the key the backup settings/retention correlate a project on. Empty
        /// string when no work is loaded (or a pre-v2 project not yet healed).
        unique_id: Signal<String>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
        /// Set while `refresh`/`clear` writes the field signals, so the dirty
        /// flag isn't tripped by a programmatic load (mirrors C++ `m_isRefreshing`).
        is_refreshing: Cell<bool>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleWork {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleWork {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    title: Signal::new(String::new()),
                    author_name: Signal::new(String::new()),
                    dict_language: Signal::new(Vec::new()),
                    chapter_mode: Signal::new(ChapterMode::default()),
                    custom_replacement_rules_enabled: Signal::new(false),
                    unique_id: Signal::new(String::new()),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                    is_refreshing: Cell::new(false),
                    ctx,
                }),
            }
        }

        /// Point the handle at an entity id (or clear it) and load its data.
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

        /// Subscribe to the entity's `Updated` events so the handle auto-refreshes
        /// when its `Work` changes elsewhere. Call once from a long-lived widget's
        /// `build`; the subscription runs on the UI thread, so setting the
        /// `Rc`-based signals inside it is sound.
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Work(EntityEvent::Updated)),
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

        // ── reactive accessors ──
        pub fn title(&self) -> Signal<String> {
            self.inner.title.clone()
        }
        pub fn author_name(&self) -> Signal<String> {
            self.inner.author_name.clone()
        }
        pub fn dict_language(&self) -> Signal<Vec<String>> {
            self.inner.dict_language.clone()
        }
        pub fn chapter_mode(&self) -> Signal<ChapterMode> {
            self.inner.chapter_mode.clone()
        }
        /// Whether the per-project custom text-replacement lexicon is active.
        pub fn custom_replacement_rules_enabled(&self) -> Signal<bool> {
            self.inner.custom_replacement_rules_enabled.clone()
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
        /// The open project's stable UUID (empty when no work is loaded). Read-only —
        /// the key backup settings/retention correlate this project on.
        pub fn unique_id(&self) -> Signal<String> {
            self.inner.unique_id.clone()
        }

        // ── setters for two-way binding: mark dirty unless mid-refresh ──
        pub fn set_title(&self, v: String) {
            self.mark_dirty();
            self.inner.title.set(v);
        }
        pub fn set_author_name(&self, v: String) {
            self.mark_dirty();
            self.inner.author_name.set(v);
        }
        pub fn set_dict_language(&self, v: Vec<String>) {
            self.mark_dirty();
            self.inner.dict_language.set(v);
        }
        pub fn set_chapter_mode(&self, v: ChapterMode) {
            self.mark_dirty();
            self.inner.chapter_mode.set(v);
        }
        pub fn set_custom_replacement_rules_enabled(&self, v: bool) {
            self.mark_dirty();
            self.inner.custom_replacement_rules_enabled.set(v);
        }

        fn mark_dirty(&self) {
            if !self.inner.is_refreshing.get() {
                self.inner.dirty.set(true);
            }
        }

        /// Persist field edits to the backend (no-op when clean or unset), on the
        /// shared per-`Work` undo stack so the change is undoable.
        pub fn save(&self, stack_id: Option<u64>) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            if !self.inner.dirty.get() {
                return;
            }
            self.inner.loading_status.set(LoadingStatus::Loading);
            let ctx = &*self.inner.ctx;
            // Fetch the stored Work to preserve fields this editor doesn't own
            // (created_at, and especially the stable `unique_id` — a scalar
            // update overwrites every field, so it must be carried through).
            let existing = match work_commands::get_work(ctx, &id) {
                Ok(Some(w)) => w,
                _ => return self.fail("Work not found"),
            };
            let dto = UpdateWorkDto {
                id,
                created_at: existing.created_at,
                updated_at: chrono::Utc::now(),
                title: self.inner.title.get(),
                author_name: self.inner.author_name.get(),
                dict_language: self.inner.dict_language.get(),
                unique_id: existing.unique_id,
                chapter_mode: self.inner.chapter_mode.get(),
                custom_replacement_rules_enabled: self
                    .inner
                    .custom_replacement_rules_enabled
                    .get(),
            };
            match work_commands::update_work(ctx, stack_id, &dto) {
                Ok(_) => {
                    self.inner.dirty.set(false);
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Err(e) => self.fail(&e.to_string()),
            }
        }

        fn refresh(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            self.inner.loading_status.set(LoadingStatus::Loading);
            match work_commands::get_work(&self.inner.ctx, &id) {
                Ok(Some(w)) => {
                    self.inner.is_refreshing.set(true);
                    self.inner.title.set(w.title);
                    self.inner.author_name.set(w.author_name);
                    self.inner.dict_language.set(w.dict_language);
                    self.inner.chapter_mode.set(w.chapter_mode);
                    self.inner
                        .custom_replacement_rules_enabled
                        .set(w.custom_replacement_rules_enabled);
                    self.inner.unique_id.set(w.unique_id);
                    self.inner.is_refreshing.set(false);
                    self.inner.dirty.set(false);
                    self.inner.error_message.set(String::new());
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Ok(None) => self.clear(),
                Err(e) => self.fail(&e.to_string()),
            }
        }

        fn clear(&self) {
            self.inner.is_refreshing.set(true);
            self.inner.title.set(String::new());
            self.inner.author_name.set(String::new());
            self.inner.dict_language.set(Vec::new());
            self.inner.chapter_mode.set(ChapterMode::default());
            self.inner.custom_replacement_rules_enabled.set(false);
            self.inner.unique_id.set(String::new());
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
    use frontend::common::entities::ChapterMode;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        title: Signal<String>,
        author_name: Signal<String>,
        dict_language: Signal<Vec<String>>,
        chapter_mode: Signal<ChapterMode>,
        custom_replacement_rules_enabled: Signal<bool>,
        unique_id: Signal<String>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
    }

    #[derive(Clone)]
    pub struct SingleWork {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleWork {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            // Fabricated, always-loaded data so the mock UI renders with no backend.
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(Some(1)),
                    title: Signal::new("Mock Work".to_string()),
                    author_name: Signal::new("Mock Author".to_string()),
                    dict_language: Signal::new(vec!["en".to_string()]),
                    chapter_mode: Signal::new(ChapterMode::default()),
                    custom_replacement_rules_enabled: Signal::new(false),
                    unique_id: Signal::new("mock-work-uid-1".to_string()),
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

        pub fn title(&self) -> Signal<String> {
            self.inner.title.clone()
        }
        pub fn author_name(&self) -> Signal<String> {
            self.inner.author_name.clone()
        }
        pub fn dict_language(&self) -> Signal<Vec<String>> {
            self.inner.dict_language.clone()
        }
        pub fn chapter_mode(&self) -> Signal<ChapterMode> {
            self.inner.chapter_mode.clone()
        }
        pub fn custom_replacement_rules_enabled(&self) -> Signal<bool> {
            self.inner.custom_replacement_rules_enabled.clone()
        }
        pub fn unique_id(&self) -> Signal<String> {
            self.inner.unique_id.clone()
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

        pub fn set_title(&self, v: String) {
            self.inner.title.set(v);
        }
        pub fn set_author_name(&self, v: String) {
            self.inner.author_name.set(v);
        }
        pub fn set_dict_language(&self, v: Vec<String>) {
            self.inner.dict_language.set(v);
        }
        pub fn set_chapter_mode(&self, v: ChapterMode) {
            self.inner.chapter_mode.set(v);
        }
        pub fn set_custom_replacement_rules_enabled(&self, v: bool) {
            self.inner.custom_replacement_rules_enabled.set(v);
        }

        pub fn save(&self, _stack_id: Option<u64>) {
            self.inner.dirty.set(false);
            self.inner.loading_status.set(LoadingStatus::Loaded);
        }
    }
}

pub use imp::SingleWork;

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use std::rc::Rc;

    use frontend::AppContext;

    use super::SingleWork;
    use crate::singles::LoadingStatus;

    #[test]
    fn mock_set_id_is_loaded_with_title() {
        let s = SingleWork::new(Rc::new(AppContext::new()));
        s.set_id(Some(1));
        assert_eq!(s.loading_status().get(), LoadingStatus::Loaded);
        assert!(!s.title().get().is_empty());
    }
}
