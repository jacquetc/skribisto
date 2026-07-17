// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleCorkboardCard` — the **lazy, per-tile** half of a corkboard card.
//!
//! [`CorkboardCardsModel`](crate::models::CorkboardCardsModel) bakes the cheap
//! fields (title, label, type, child count) into every row. The two *expensive*
//! things — the synopsis **rich document** and a leaf's **word count** — are
//! resolved here, one handle per **realized** GridView tile, so a 500-card board
//! only ever pays for the ~viewport it shows. The synopsis is shown in full
//! (rich, read-only, scrollable) — a `TextDocument` loaded from the child's
//! `SynopsisText` — and refreshes when that content is edited elsewhere.
//!
//! Read-only, like [`SingleBinderItem`](crate::singles::SingleBinderItem). Two
//! `mod imp` variants share one surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;
    use bastyde::text_document::TextDocument;

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
        /// The synopsis, rendered read-only in the card. A stable handle updated
        /// in place on load / edit, so the viewer bound to it just repaints.
        synopsis_doc: TextDocument,
        /// Content ids watched for a live refresh.
        synopsis_id: Cell<Option<u64>>,
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
                    synopsis_doc: TextDocument::new(),
                    synopsis_id: Cell::new(None),
                    scene_id: Cell::new(None),
                    word_count: Signal::new(None),
                }),
            }
        }

        /// Point the handle at a card's item and load its synopsis + word count.
        pub fn set_card(&self, item_id: u64) {
            self.inner.item_id.set(item_id);
            self.reload();
        }

        /// The synopsis document — feed it to a read-only `RichTextEditor`.
        pub fn synopsis_doc(&self) -> TextDocument {
            self.inner.synopsis_doc.clone()
        }

        pub fn word_count(&self) -> Signal<Option<usize>> {
            self.inner.word_count.clone()
        }

        /// Refresh when this card's content is edited elsewhere, or the counting
        /// method changes. Scoped to the tile's build (dropped when it scrolls out).
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
                move |event: &Event| {
                    let watched = [s.inner.synopsis_id.get(), s.inner.scene_id.get()];
                    let hit = watched
                        .into_iter()
                        .flatten()
                        .any(|id| event.ids.contains(&id));
                    if hit {
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
            let synopsis = contents
                .iter()
                .find(|c| c.role == ContentRole::SynopsisText);
            let scene = contents.iter().find(|c| c.role == ContentRole::SceneText);

            self.inner.synopsis_id.set(synopsis.map(|c| c.id));
            self.inner.scene_id.set(scene.map(|c| c.id));

            // Load the synopsis into the read-only doc. `set_djot_sync` (not the
            // async setter) because this is a load, not a user edit.
            let djot = synopsis.map(|c| c.data.clone()).unwrap_or_default();
            let _ = self.inner.synopsis_doc.set_djot_sync(&djot);

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
            self.inner.synopsis_id.set(None);
            self.inner.scene_id.set(None);
            let _ = self.inner.synopsis_doc.set_djot_sync("");
            self.inner.word_count.set(None);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::prelude::*;
    use bastyde::text_document::TextDocument;

    use frontend::AppContext;
    use skribisto_model::counting::CountingMethodSetting;

    #[derive(Clone)]
    pub struct SingleCorkboardCard {
        synopsis_doc: TextDocument,
        word_count: Signal<Option<usize>>,
    }

    impl SingleCorkboardCard {
        pub fn new(_ctx: Rc<AppContext>, _method: Signal<CountingMethodSetting>) -> Self {
            Self {
                synopsis_doc: TextDocument::new(),
                word_count: Signal::new(None),
            }
        }

        pub fn set_card(&self, item_id: u64) {
            // Deterministic fabricated synopsis + count so the mock board reads naturally.
            let (text, words) = match item_id {
                201 => (
                    "Mara takes the dinghy out as the squall closes in.\n\nThe lamp dies; she climbs the tower in the dark and finds the torn page.",
                    Some(1204),
                ),
                202 => (
                    "Dawn over the rocks.\n\nThe keeper's secret is out, and Mara must decide whether the light is worth keeping lit.",
                    Some(640),
                ),
                203 => (
                    "The lamp-room, the wind, and a sound from below the waterline that the keeper refuses to name.",
                    Some(1510),
                ),
                302 => (
                    "Arrival on the mainland ferry.\n\nFirst sight of the island and the tower that will define the year ahead.",
                    Some(980),
                ),
                104 => (
                    "A folder chapter — its child scenes compile into this section. The keeper's history, told in fragments.",
                    None,
                ),
                _ => ("", None),
            };
            let _ = self.synopsis_doc.set_djot_sync(text);
            self.word_count.set(words);
        }

        pub fn synopsis_doc(&self) -> TextDocument {
            self.synopsis_doc.clone()
        }

        pub fn word_count(&self) -> Signal<Option<usize>> {
            self.word_count.clone()
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}
    }
}

pub use imp::SingleCorkboardCard;
