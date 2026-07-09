//! `SingleScene` — one scene's editable **main text** (`SceneText`) plus its
//! reactive title / label, for the Chapter folder tab's Full Chapter view.
//!
//! Combines a read-only [`SingleBinderItem`](crate::singles::SingleBinderItem)
//! probe (title / label, refreshed by the owning `ChapterViewModel`) with a live
//! `TextDocument` + a [`SingleContent`](crate::singles::SingleContent) for the
//! `SceneText` row (create-or-update persistence). Mirrors the `ProseField`
//! pattern in `tabs.rs` but as a Layer-A single so the Full Chapter rows carry no
//! tab chrome. Two `mod imp` variants share one public surface (see
//! [`crate::singles`]).

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::rc::Rc;

    use bastyde::prelude::*;
    use bastyde::text_document::TextDocument;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;
    use frontend::direct_access::ContentDto;

    use crate::singles::{SingleBinderItem, SingleContent};

    struct Inner {
        item_id: u64,
        probe: SingleBinderItem,
        doc: TextDocument,
        content: SingleContent,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleScene {
        inner: Rc<Inner>,
    }

    impl SingleScene {
        pub fn new(ctx: Rc<AppContext>, item_id: u64) -> Self {
            let probe = SingleBinderItem::new(ctx.clone());
            probe.set_id(Some(item_id));

            let existing = load_scene_text(&ctx, item_id);
            let doc = TextDocument::new();
            let _ = doc
                .set_djot(existing.as_ref().map(|c| c.data.as_str()).unwrap_or(""))
                .and_then(|op| op.wait());
            doc.set_modified(false);
            let content =
                SingleContent::for_field(ctx.clone(), item_id, ContentRole::SceneText, existing.as_ref());

            Self {
                inner: Rc::new(Inner {
                    item_id,
                    probe,
                    doc,
                    content,
                    ctx,
                }),
            }
        }

        pub fn id(&self) -> u64 {
            self.inner.item_id
        }

        /// The scene's name, reactive (refreshed via [`refresh_meta`](Self::refresh_meta)).
        pub fn title(&self) -> Signal<String> {
            self.inner.probe.title()
        }

        /// The scene's free-text label, reactive.
        pub fn label(&self) -> Signal<String> {
            self.inner
                .probe
                .dto_signal()
                .map(|d| d.as_ref().map(|x| x.label.clone()).unwrap_or_default())
        }

        /// The live editable `SceneText` document the editor binds to.
        pub fn main_doc(&self) -> TextDocument {
            self.inner.doc.clone()
        }

        /// Persist the scene's text if edited (create-or-update the `SceneText` row).
        pub fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
            if !self.inner.doc.is_modified() {
                return Ok(());
            }
            self.inner.content.set_data(self.inner.doc.to_djot()?);
            self.inner.content.save(stack)?;
            self.inner.doc.set_modified(false);
            Ok(())
        }

        /// Reload title / label from the store (after this scene is renamed/relabelled).
        pub fn refresh_meta(&self) {
            self.inner.probe.set_id(Some(self.inner.item_id));
        }

        /// Reload the `SceneText` document from the store (after a merge/split
        /// rewrote this scene's stored content).
        pub fn reload_doc(&self) {
            let existing = load_scene_text(&self.inner.ctx, self.inner.item_id);
            let _ = self
                .inner
                .doc
                .set_djot(existing.as_ref().map(|c| c.data.as_str()).unwrap_or(""))
                .and_then(|op| op.wait());
            self.inner.doc.set_modified(false);
        }
    }

    /// The item's `SceneText` content row, if any.
    fn load_scene_text(ctx: &AppContext, item_id: u64) -> Option<ContentDto> {
        let ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(ctx, &ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::SceneText)
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::prelude::*;
    use bastyde::text_document::TextDocument;

    use frontend::AppContext;

    #[derive(Clone)]
    pub struct SingleScene {
        item_id: u64,
        title: Signal<String>,
        label: Signal<String>,
        doc: TextDocument,
    }

    impl SingleScene {
        pub fn new(_ctx: Rc<AppContext>, item_id: u64) -> Self {
            let n = item_id.saturating_sub(200);
            let doc = TextDocument::new();
            let body = format!(
                "This is the fabricated body of scene {n}. The morning light crept over the \
                 ridgeline and the camp began to stir; she had not slept, and the cold had \
                 settled deep into her hands.\n\nA second paragraph follows, so the Full Chapter \
                 view shows real flowing prose per scene."
            );
            let _ = doc.set_djot(&body).and_then(|op| op.wait());
            doc.set_modified(false);
            Self {
                item_id,
                title: Signal::new(format!("Scene {n}")),
                label: Signal::new(if n == 1 { "opening beat".to_string() } else { String::new() }),
                doc,
            }
        }

        pub fn id(&self) -> u64 {
            self.item_id
        }
        pub fn title(&self) -> Signal<String> {
            self.title.clone()
        }
        pub fn label(&self) -> Signal<String> {
            self.label.clone()
        }
        pub fn main_doc(&self) -> TextDocument {
            self.doc.clone()
        }
        pub fn flush(&self, _stack: Option<u64>) -> anyhow::Result<()> {
            Ok(())
        }
        pub fn refresh_meta(&self) {}
        pub fn reload_doc(&self) {}
    }
}

pub use imp::SingleScene;
