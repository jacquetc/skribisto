// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleCorkboardCard` — the **lazy, per-tile** half of a corkboard card.
//!
//! [`CorkboardCardsModel`](crate::models::CorkboardCardsModel) bakes the cheap
//! fields (title, label, type, child count) into every row. A leaf's **word count**
//! is the one expensive field resolved here, one handle per **realized** GridView
//! tile, so a 500-card board only ever pays for the ~viewport it shows. The
//! synopsis itself is *not* loaded here: every card now edits it in a live editor
//! over the shared [`OpenDoc`](crate::models::OpenDoc) the view-model holds open, so
//! there is exactly one document per item (card ⇄ expand modal ⇄ editor tab).
//!
//! Read-only, like [`SingleBinderItem`](crate::singles::SingleBinderItem). Two
//! `mod imp` variants share one surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::entities::ContentRole;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

    use skribisto_model::counting::{self, CountMethod, CountingMethodSetting};
    use skribisto_model::language;

    struct Inner {
        ctx: Rc<AppContext>,
        method: Signal<CountingMethodSetting>,
        item_id: Cell<u64>,
        /// The scene-prose content id, watched for a live word-count refresh.
        scene_id: Cell<Option<u64>>,
        /// `None` when the card carries no scene prose (a folder / note).
        word_count: Signal<Option<usize>>,
    }

    #[derive(Clone)]
    pub struct SingleCorkboardCard {
        inner: Rc<Inner>,
    }

    impl SingleCorkboardCard {
        pub fn new(ctx: Rc<AppContext>, method: Signal<CountingMethodSetting>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    ctx,
                    method,
                    item_id: Cell::new(0),
                    scene_id: Cell::new(None),
                    word_count: Signal::new(None),
                }),
            }
        }

        /// Point the handle at a card's item and load its word count.
        pub fn set_card(&self, item_id: u64) {
            self.inner.item_id.set(item_id);
            self.reload();
        }

        pub fn word_count(&self) -> Signal<Option<usize>> {
            self.inner.word_count.clone()
        }

        /// Refresh when this card's scene prose is edited elsewhere, or the counting
        /// method changes. Scoped to the tile's build (dropped when it scrolls out).
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
                move |event: &Event| {
                    if let Some(id) = s.inner.scene_id.get()
                        && event.ids.contains(&id)
                    {
                        s.reload();
                    }
                },
            );
            let s = self.clone();
            ctx.effect(&self.inner.method, move |_| s.reload());
        }

        fn reload(&self) {
            let item_id = self.inner.item_id.get();
            if item_id == 0 {
                return;
            }
            let Some(dto) = binder_item_commands::get_binder_item(&self.inner.ctx, &item_id)
                .ok()
                .flatten()
            else {
                self.clear();
                return;
            };
            // One batch read of the item's content rows — no editor is built.
            let contents: Vec<_> =
                content_commands::get_content_multi(&self.inner.ctx, &dto.contents)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .collect();
            let scene = contents.iter().find(|c| c.role == ContentRole::SceneText);

            self.inner.scene_id.set(scene.map(|c| c.id));

            let words = scene.map(|c| {
                let method = counting::resolve_method(
                    self.inner.method.get(),
                    CountMethod::UnicodeWords,
                    language::primary(&dto.dict_language),
                );
                // Content-addressed cache: re-realizing a tile is a cache hit.
                counting::cached_count(&c.data, method).words
            });
            self.inner.word_count.set(words);
        }

        fn clear(&self) {
            self.inner.scene_id.set(None);
            self.inner.word_count.set(None);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use skribisto_model::counting::CountingMethodSetting;

    #[derive(Clone)]
    pub struct SingleCorkboardCard {
        word_count: Signal<Option<usize>>,
    }

    impl SingleCorkboardCard {
        pub fn new(_ctx: Rc<AppContext>, _method: Signal<CountingMethodSetting>) -> Self {
            Self {
                word_count: Signal::new(None),
            }
        }

        pub fn set_card(&self, item_id: u64) {
            // Deterministic fabricated counts so the mock board reads naturally. The
            // synopsis text itself is fabricated by the mock `OpenDocsStore` when the
            // card opens the shared doc.
            let words = match item_id {
                201 => Some(1204),
                202 => Some(640),
                203 => Some(1510),
                302 => Some(980),
                _ => None,
            };
            self.word_count.set(words);
        }

        pub fn word_count(&self) -> Signal<Option<usize>> {
            self.word_count.clone()
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}
    }
}

pub use imp::SingleCorkboardCard;
