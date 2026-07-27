// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleSmartPunctuation` — a reactive handle over the open `Work`'s
//! punctuation house style.
//!
//! Same shape as [`SingleWork`](crate::singles::SingleWork): one entity by id,
//! each field a `Signal`, edits written back through `save()` on the per-`Work`
//! undo stack. Two `mod imp` variants (real backend / fabricated mock) share an
//! identical public surface — no `#[cfg]` leaks into consumers. See
//! [`crate::singles`].
//!
//! ## Where its id comes from
//!
//! `Work.smart_punctuation`, not an id the UI picks. The row is minted by
//! `new_work`/`load_work` before the `Work` itself, so by the time anything can
//! point this handle at a project the id is already there — `set_id(None)` here
//! means "no project open", never "this project has no punctuation row".

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::smart_punctuation_commands as cmd;
    use frontend::common::entities::QuoteStyle;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::UpdateSmartPunctuationDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        override_app_default: Signal<bool>,
        dashes: Signal<bool>,
        ellipsis: Signal<bool>,
        quotes: Signal<bool>,
        quote_style: Signal<QuoteStyle>,
        pre_punctuation_spacing: Signal<bool>,
        dialogue_marker: Signal<bool>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
        is_refreshing: Cell<bool>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleSmartPunctuation {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleSmartPunctuation {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    override_app_default: Signal::new(false),
                    dashes: Signal::new(false),
                    ellipsis: Signal::new(false),
                    quotes: Signal::new(false),
                    quote_style: Signal::new(QuoteStyle::default()),
                    pre_punctuation_spacing: Signal::new(false),
                    dialogue_marker: Signal::new(false),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                    is_refreshing: Cell::new(false),
                    ctx,
                }),
            }
        }

        pub fn set_id(&self, id: Option<u64>) {
            // Re-pointing to the row already loaded is a no-op: a fresh `refresh`
            // would re-read it and re-fire every flag signal, re-pushing the
            // whole house style to every open editor for no change. The driving
            // effect fires on each Work save, so without this the punctuation is
            // recomputed on every keystroke-triggered autosave. Genuine content
            // edits arrive through `wire`'s `Updated` subscription, not here.
            if self.inner.id.get() == id {
                return;
            }
            self.inner.id.set(id);
            match id {
                Some(_) => self.refresh(),
                None => self.clear(),
            }
        }

        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }

        /// Auto-refresh when the row changes elsewhere — an undo, or the other
        /// settings surface. Call once from a long-lived widget's `build`.
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::SmartPunctuation(EntityEvent::Updated)),
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
        pub fn override_app_default(&self) -> Signal<bool> {
            self.inner.override_app_default.clone()
        }
        pub fn dashes(&self) -> Signal<bool> {
            self.inner.dashes.clone()
        }
        pub fn ellipsis(&self) -> Signal<bool> {
            self.inner.ellipsis.clone()
        }
        pub fn quotes(&self) -> Signal<bool> {
            self.inner.quotes.clone()
        }
        pub fn quote_style(&self) -> Signal<QuoteStyle> {
            self.inner.quote_style.clone()
        }
        pub fn pre_punctuation_spacing(&self) -> Signal<bool> {
            self.inner.pre_punctuation_spacing.clone()
        }
        pub fn dialogue_marker(&self) -> Signal<bool> {
            self.inner.dialogue_marker.clone()
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

        // ── writes ──
        pub fn set_override_app_default(&self, v: bool) {
            self.mark_dirty();
            self.inner.override_app_default.set(v);
        }
        pub fn set_dashes(&self, v: bool) {
            self.mark_dirty();
            self.inner.dashes.set(v);
        }
        pub fn set_ellipsis(&self, v: bool) {
            self.mark_dirty();
            self.inner.ellipsis.set(v);
        }
        pub fn set_quotes(&self, v: bool) {
            self.mark_dirty();
            self.inner.quotes.set(v);
        }
        pub fn set_quote_style(&self, v: QuoteStyle) {
            self.mark_dirty();
            self.inner.quote_style.set(v);
        }
        pub fn set_pre_punctuation_spacing(&self, v: bool) {
            self.mark_dirty();
            self.inner.pre_punctuation_spacing.set(v);
        }
        pub fn set_dialogue_marker(&self, v: bool) {
            self.mark_dirty();
            self.inner.dialogue_marker.set(v);
        }

        fn mark_dirty(&self) {
            if !self.inner.is_refreshing.get() {
                self.inner.dirty.set(true);
            }
        }

        /// Persist to the backend on the per-`Work` undo stack, so a writer who
        /// flips a switch by accident gets it back with Ctrl+Z like any other
        /// project edit.
        pub fn save(&self, stack_id: Option<u64>) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            if !self.inner.dirty.get() {
                return;
            }
            self.inner.loading_status.set(LoadingStatus::Loading);
            let ctx = &*self.inner.ctx;
            // Read back first, to carry `created_at` through: a scalar update
            // overwrites every field, so anything this editor does not own has
            // to be re-supplied rather than defaulted.
            let existing = match cmd::get_smart_punctuation(ctx, &id) {
                Ok(Some(sp)) => sp,
                _ => return self.fail("Punctuation settings not found"),
            };
            let dto = UpdateSmartPunctuationDto {
                id,
                created_at: existing.created_at,
                updated_at: chrono::Utc::now(),
                override_app_default: self.inner.override_app_default.get(),
                dashes: self.inner.dashes.get(),
                ellipsis: self.inner.ellipsis.get(),
                quotes: self.inner.quotes.get(),
                quote_style: self.inner.quote_style.get(),
                pre_punctuation_spacing: self.inner.pre_punctuation_spacing.get(),
                dialogue_marker: self.inner.dialogue_marker.get(),
            };
            match cmd::update_smart_punctuation(ctx, stack_id, &dto) {
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
            match cmd::get_smart_punctuation(&self.inner.ctx, &id) {
                Ok(Some(sp)) => {
                    self.inner.is_refreshing.set(true);
                    self.inner.override_app_default.set(sp.override_app_default);
                    self.inner.dashes.set(sp.dashes);
                    self.inner.ellipsis.set(sp.ellipsis);
                    self.inner.quotes.set(sp.quotes);
                    self.inner.quote_style.set(sp.quote_style);
                    self.inner
                        .pre_punctuation_spacing
                        .set(sp.pre_punctuation_spacing);
                    self.inner.dialogue_marker.set(sp.dialogue_marker);
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
            self.inner.override_app_default.set(false);
            self.inner.dashes.set(false);
            self.inner.ellipsis.set(false);
            self.inner.quotes.set(false);
            self.inner.quote_style.set(QuoteStyle::default());
            self.inner.pre_punctuation_spacing.set(false);
            self.inner.dialogue_marker.set(false);
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
    use frontend::common::entities::QuoteStyle;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        override_app_default: Signal<bool>,
        dashes: Signal<bool>,
        ellipsis: Signal<bool>,
        quotes: Signal<bool>,
        quote_style: Signal<QuoteStyle>,
        pre_punctuation_spacing: Signal<bool>,
        dialogue_marker: Signal<bool>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
        is_refreshing: Cell<bool>,
    }

    #[derive(Clone)]
    pub struct SingleSmartPunctuation {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)]
    impl SingleSmartPunctuation {
        /// A mock project with an active house style.
        ///
        /// Deliberately **not** the entity's all-off default: the settings pane's
        /// mock tests need a row where the override is on and the sub-switches
        /// are meaningfully mixed, or they would only ever exercise the disabled
        /// half of the pane.
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(Some(1)),
                    override_app_default: Signal::new(true),
                    dashes: Signal::new(true),
                    ellipsis: Signal::new(true),
                    quotes: Signal::new(true),
                    quote_style: Signal::new(QuoteStyle::Guillemets),
                    // Off, so at least one sub-switch differs from the rest.
                    pre_punctuation_spacing: Signal::new(false),
                    dialogue_marker: Signal::new(false),
                    loading_status: Signal::new(LoadingStatus::Loaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                    is_refreshing: Cell::new(false),
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

        pub fn override_app_default(&self) -> Signal<bool> {
            self.inner.override_app_default.clone()
        }
        pub fn dashes(&self) -> Signal<bool> {
            self.inner.dashes.clone()
        }
        pub fn ellipsis(&self) -> Signal<bool> {
            self.inner.ellipsis.clone()
        }
        pub fn quotes(&self) -> Signal<bool> {
            self.inner.quotes.clone()
        }
        pub fn quote_style(&self) -> Signal<QuoteStyle> {
            self.inner.quote_style.clone()
        }
        pub fn pre_punctuation_spacing(&self) -> Signal<bool> {
            self.inner.pre_punctuation_spacing.clone()
        }
        pub fn dialogue_marker(&self) -> Signal<bool> {
            self.inner.dialogue_marker.clone()
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

        pub fn set_override_app_default(&self, v: bool) {
            self.mark_dirty();
            self.inner.override_app_default.set(v);
        }
        pub fn set_dashes(&self, v: bool) {
            self.mark_dirty();
            self.inner.dashes.set(v);
        }
        pub fn set_ellipsis(&self, v: bool) {
            self.mark_dirty();
            self.inner.ellipsis.set(v);
        }
        pub fn set_quotes(&self, v: bool) {
            self.mark_dirty();
            self.inner.quotes.set(v);
        }
        pub fn set_quote_style(&self, v: QuoteStyle) {
            self.mark_dirty();
            self.inner.quote_style.set(v);
        }
        pub fn set_pre_punctuation_spacing(&self, v: bool) {
            self.mark_dirty();
            self.inner.pre_punctuation_spacing.set(v);
        }
        pub fn set_dialogue_marker(&self, v: bool) {
            self.mark_dirty();
            self.inner.dialogue_marker.set(v);
        }

        fn mark_dirty(&self) {
            if !self.inner.is_refreshing.get() {
                self.inner.dirty.set(true);
            }
        }

        /// Accepts the write and clears the dirty flag, so a pane under mocks
        /// behaves like one over a working backend rather than looking stuck.
        pub fn save(&self, _stack_id: Option<u64>) {
            self.inner.dirty.set(false);
            self.inner.loading_status.set(LoadingStatus::Loaded);
        }
    }
}

pub use imp::SingleSmartPunctuation;
