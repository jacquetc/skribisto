//! `OpenDocsStore` + `OpenDoc` — the app's shared holder of open editing state.
//!
//! An [`OpenDoc`] is one binder item's **live, shareable** editing state: its main
//! text + synopsis documents, its title fields, the Full Chapter view-model, and
//! the dirty flag — everything a `RichTextEditor` binds to. Because a
//! [`TextDocument`](bastyde::text_document::TextDocument) is a cheap `Arc` handle,
//! two editors bound to the same `OpenDoc` share one live document.
//!
//! [`OpenDocsStore`] keeps one `Rc<OpenDoc>` per open item id, reference-counted:
//! [`open`](OpenDocsStore::open) builds-or-reuses (and refs) it, [`release`](OpenDocsStore::release)
//! unrefs and — on the last reference — flushes + evicts. Registered as
//! `app_state`, so the shared documents are reachable **outside** the editor tabs
//! (a preview pane, the Full Chapter view, any future consumer), not only from the
//! `TabWidget`s. This file is written once (no `#[cfg]` seam): the real/mock
//! difference lives below it in `SingleContent` / `SingleBinderItem`, exactly as
//! for the other Layer-A models.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::Signal;

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::singles::SingleBinderItem;
use crate::tabs::{ProseField, ProseKind, TitleField, prose_field, prose_kind_for, title_field};

/// One open item's live editing state, shared by every view showing that item.
pub struct OpenDoc {
    pub item_id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub kind: Option<ProseKind>,
    pub title: Option<TitleField>,
    pub subtitle: Option<TitleField>,
    pub main: Option<ProseField>,
    pub synopsis: Option<ProseField>,
    /// `true` once any editor bound to this doc edited a field since the last
    /// save. Cleared by [`flush`](Self::flush).
    pub dirty: Signal<bool>,
    /// The store's aggregate "an edit happened" counter — bumped by every edit,
    /// observed by the debounced autosave timer.
    edited: Signal<u64>,
}

impl OpenDoc {
    /// Build an item's editing state from its already-fetched `Content` rows,
    /// loading each allowed role into the right field. `edited` is the store's
    /// shared edit counter.
    ///
    /// An `OpenDoc` is a **leaf**: it owns documents, nothing else. The container
    /// tabs' `StreamViewModel` deliberately does *not* live here — it holds the
    /// `OpenDocsStore` (to open its rows), and the store owns this `OpenDoc`, so
    /// hanging it here would close an `Rc` cycle. Worse, `OpenDocsStore::clear()`
    /// drops its `OpenDoc`s *while* holding the map's `RefCell` borrow, so a `Drop`
    /// that released row refs would re-enter `borrow_mut()` and panic. It lives on
    /// `ContentTab` instead, which nothing in the store points back at.
    pub fn build(
        ctx: &Rc<AppContext>,
        item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
        contents: &[ContentDto],
        edited: Signal<u64>,
    ) -> Self {
        let mut doc = OpenDoc {
            item_id,
            role: role.clone(),
            sub_role: sub_role.clone(),
            kind: prose_kind_for(role, sub_role),
            title: None,
            subtitle: None,
            main: None,
            synopsis: None,
            dirty: Signal::new(false),
            edited,
        };
        for cr in skribisto_model::allowed_content(role, sub_role) {
            let existing = contents.iter().find(|c| &c.role == cr);
            match cr {
                ContentRole::SynopsisText => {
                    doc.synopsis = Some(prose_field(ctx, item_id, cr.clone(), existing))
                }
                ContentRole::SceneText | ContentRole::NoteText => {
                    doc.main = Some(prose_field(ctx, item_id, cr.clone(), existing))
                }
                ContentRole::BookSubtitle => {
                    doc.subtitle = Some(title_field(ctx, item_id, cr.clone(), existing))
                }
                ContentRole::BookTitle | ContentRole::ChapterTitle | ContentRole::PartTitle => {
                    doc.title = Some(title_field(ctx, item_id, cr.clone(), existing))
                }
            }
        }
        doc
    }

    /// The `on_change` hook for this doc's editors: mark it dirty and bump the
    /// store's aggregate edit counter (drives autosave).
    pub fn mark_dirty_fn(&self) -> impl Fn() + 'static {
        let dirty = self.dirty.clone();
        let edited = self.edited.clone();
        move || {
            dirty.set(true);
            edited.set(edited.get().wrapping_add(1));
        }
    }

    /// Persist every changed field back to its `Content` row (creating the row on
    /// first save) via each field's `SingleContent`. Idempotent (clean fields are
    /// a no-op). Routes through the undo `stack`.
    pub fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        if let Some(f) = &self.title {
            f.flush(stack)?;
        }
        if let Some(f) = &self.subtitle {
            f.flush(stack)?;
        }
        if let Some(f) = &self.main {
            f.flush(stack)?;
        }
        if let Some(f) = &self.synopsis {
            f.flush(stack)?;
        }
        self.dirty.set(false);
        Ok(())
    }

    /// Discard the live edits and re-read every present field from its persisted
    /// `Content` row — for a doc another use case rewrote out from under us (a merge
    /// absorbing a neighbour, a split cutting the source in two). Both writing roles
    /// are reloaded, not just the prose: a merge concatenates the synopses too.
    ///
    /// The caller flushes first, so nothing unsaved is lost; it must also pump a
    /// frame afterwards, since `set_djot` only queues a document event.
    pub fn reload(&self) {
        if let Some(f) = &self.title {
            f.reload();
        }
        if let Some(f) = &self.subtitle {
            f.reload();
        }
        if let Some(f) = &self.main {
            f.reload();
        }
        if let Some(f) = &self.synopsis {
            f.reload();
        }
        self.dirty.set(false);
    }
}

struct Entry {
    doc: Rc<OpenDoc>,
    refs: usize,
}

struct Inner {
    open: RefCell<HashMap<u64, Entry>>,
    app_ctx: Rc<AppContext>,
    /// Reactive read handle re-pointed at an item to fetch its `(role, sub_role)`.
    item_probe: SingleBinderItem,
    /// Aggregate "an edit happened" counter shared by every open doc.
    edited: Signal<u64>,
}

/// The app-wide store of open documents (cheap `Rc` handle, shared by clone).
#[derive(Clone)]
pub struct OpenDocsStore {
    inner: Rc<Inner>,
}

impl OpenDocsStore {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            inner: Rc::new(Inner {
                open: RefCell::new(HashMap::new()),
                item_probe: SingleBinderItem::new(app_ctx.clone()),
                app_ctx,
                edited: Signal::new(0),
            }),
        }
    }

    /// The aggregate edit signal — bind the debounced autosave to it.
    pub fn edited_any(&self) -> Signal<u64> {
        self.inner.edited.clone()
    }

    /// Open item `item_id`, building its [`OpenDoc`] the first time and reusing it
    /// (bumping the refcount) thereafter. `None` if the item can't be read.
    pub fn open(&self, item_id: u64) -> Option<Rc<OpenDoc>> {
        if let Some(entry) = self.inner.open.borrow_mut().get_mut(&item_id) {
            entry.refs += 1;
            return Some(entry.doc.clone());
        }
        // Resolve `(role, sub_role)` via the reactive single (Layer A), then load
        // the allowed content rows.
        self.inner.item_probe.set_id(Some(item_id));
        let item = self.inner.item_probe.dto()?;
        let contents = self.load_contents(item_id, &item.role, &item.sub_role);
        let doc = Rc::new(OpenDoc::build(
            &self.inner.app_ctx,
            item_id,
            &item.role,
            &item.sub_role,
            &contents,
            self.inner.edited.clone(),
        ));
        self.inner.open.borrow_mut().insert(
            item_id,
            Entry {
                doc: doc.clone(),
                refs: 1,
            },
        );
        Some(doc)
    }

    /// Release one reference to `item_id`. On the **last** reference, flush the
    /// doc (persisting any unsaved edits) and evict it.
    pub fn release(&self, item_id: u64, stack: Option<u64>) {
        let evicted = {
            let mut map = self.inner.open.borrow_mut();
            let Some(entry) = map.get_mut(&item_id) else {
                return;
            };
            entry.refs = entry.refs.saturating_sub(1);
            if entry.refs == 0 {
                map.remove(&item_id).map(|e| e.doc)
            } else {
                None
            }
        };
        if let Some(doc) = evicted {
            let _ = doc.flush(stack);
        }
    }

    /// Flush every open doc once (changed fields only).
    pub fn flush_all(&self, stack: Option<u64>) {
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            let _ = doc.flush(stack);
        }
    }

    /// Drop **all** open docs without flushing — for a project switch, where the
    /// outgoing work is already saved (or discarded) by the close/load flow.
    pub fn clear(&self) {
        self.inner.open.borrow_mut().clear();
    }

    /// Read an item's content rows, keeping only the roles the constraint matrix
    /// allows for its `(role, sub_role)`. `Content.data` is Djot.
    fn load_contents(
        &self,
        item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Vec<ContentDto> {
        let ctx = &*self.inner.app_ctx;
        let allowed = skribisto_model::allowed_content(role, sub_role);
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(|c| allowed.contains(&c.role))
            .collect()
    }
}

#[cfg(test)]
impl OpenDocsStore {
    /// Seed one entry directly (bypassing the backend probe) at one reference, so
    /// the refcount/eviction lifecycle is testable without a loaded project.
    fn insert_for_test(&self, doc: Rc<OpenDoc>) {
        let id = doc.item_id;
        self.inner
            .open
            .borrow_mut()
            .insert(id, Entry { doc, refs: 1 });
    }

    /// The current reference count for `item_id`, or `None` if not open.
    fn refs_for_test(&self, item_id: u64) -> Option<usize> {
        self.inner.open.borrow().get(&item_id).map(|e| e.refs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `open()` reuses an already-open doc (bumping refs), `release()` decrements
    /// and keeps it while other references remain, and evicts only at zero — the
    /// lifecycle both split panes rely on for the same item.
    #[test]
    fn refcount_reuses_releases_and_evicts() {
        let ctx = Rc::new(AppContext::new());
        let store = OpenDocsStore::new(ctx.clone());
        // Seed one open doc (as `open()` would for the first consumer).
        let doc = Rc::new(OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            store.edited_any(),
        ));
        store.insert_for_test(doc);
        assert_eq!(store.refs_for_test(1), Some(1));

        // A second consumer (e.g. the other pane) reuses the entry — refs 1 -> 2.
        let reused = store.open(1).expect("an already-open doc is reused");
        assert_eq!(store.refs_for_test(1), Some(2));
        assert_eq!(reused.item_id, 1);

        // Releasing once keeps the entry (still referenced by the first consumer).
        store.release(1, None);
        assert_eq!(store.refs_for_test(1), Some(1));

        // Releasing the last reference evicts it.
        store.release(1, None);
        assert_eq!(store.refs_for_test(1), None, "evicted at zero references");

        // Releasing an unknown / already-evicted id is a no-op.
        store.release(1, None);
        assert_eq!(store.refs_for_test(1), None);
    }

    /// The store's document is a live, shareable resource **independent of any
    /// `TabWidget`**: two handles to one `OpenDoc`'s main text are the *same* live
    /// document (an edit in one is seen by the other). This is the exact mechanism
    /// the split view's two panes rely on to show one item side-by-side.
    #[test]
    fn open_doc_shares_one_live_document() {
        let ctx = Rc::new(AppContext::new());
        let doc = OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
        );
        let main = doc
            .main
            .as_ref()
            .expect("a Scene owns a main text document");
        // No `TabWidget` anywhere: two bare handles to the shared document.
        let a = main.doc.clone();
        let b = main.doc.clone();
        let _ = a.set_djot("shared edit").and_then(|op| op.wait());
        assert!(
            b.to_djot().unwrap().contains("shared edit"),
            "an edit through one handle must be visible through the other"
        );
    }

    /// `mark_dirty_fn` flips the doc's dirty flag and bumps the store's aggregate
    /// edit counter (what the autosave timer observes).
    #[test]
    fn mark_dirty_sets_dirty_and_bumps_edited() {
        let ctx = Rc::new(AppContext::new());
        let edited = Signal::new(0u64);
        let doc = OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            edited.clone(),
        );
        assert!(!doc.dirty.get());
        let before = edited.get();
        doc.mark_dirty_fn()();
        assert!(doc.dirty.get());
        assert_eq!(edited.get(), before + 1);
    }
}
