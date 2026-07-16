// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `OpenDocsStore` + `OpenDoc` — the app's shared holder of open editing state.
//!
//! An [`OpenDoc`] is one binder item's **live, shareable** editing state: its main
//! text + synopsis documents, its title fields and the dirty flag — everything a
//! `RichTextEditor` binds to. It is a **leaf**: it owns documents and nothing else (see
//! [`OpenDoc::build`] for why the container tabs' stream view-model deliberately does
//! *not* live here). Because a
//! [`TextDocument`](bastyde::text_document::TextDocument) is a cheap `Arc` handle,
//! two editors bound to the same `OpenDoc` share one live document.
//!
//! [`OpenDocsStore`] keeps one `Rc<OpenDoc>` per open item id, reference-counted:
//! [`open`](OpenDocsStore::open) builds-or-reuses (and refs) it, [`release`](OpenDocsStore::release)
//! unrefs and — on the last reference — flushes + evicts. Registered as
//! `app_state`, so the shared documents are reachable **outside** the editor tabs (a
//! preview pane, a manuscript stream's rows, any future consumer), not only from the
//! `TabWidget`s — which is what makes one item resolve to one live document everywhere.
//! This file is written once (no `#[cfg]` seam): the real/mock
//! difference lives below it in `SingleContent` / `SingleBinderItem`, exactly as
//! for the other Layer-A models.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::Signal;
use bastyde::text_document::Color;

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, content_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItem, BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::singles::SingleBinderItem;
use crate::spellcheck::{SpellSession, SpellcheckService};
use crate::tabs::{
    ProseField, ProseKind, TitleField, TitlePart, prose_field, prose_kind_for, title_field,
};

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
    /// The caret-aware spell-check range session on the main / synopsis document, if that field
    /// exists. Created once in [`build`](Self::build); `attach_spell` sets its checker (dictionary
    /// install/remove, mute, language change) and the editor feeds it the focused view's caret.
    /// `Rc` so the editor build can hold a clone to drive it.
    spell_main: Option<Rc<SpellSession>>,
    spell_synopsis: Option<Rc<SpellSession>>,
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
            spell_main: None,
            spell_synopsis: None,
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
                // The two *names*. They are edited as the item's title/subtitle (what the
                // outline tree and the tab show) and mirrored into these content rows on
                // save — see `TitleField`.
                ContentRole::BookSubtitle => {
                    doc.subtitle = Some(title_field(ctx, item_id, TitlePart::SubTitle))
                }
                ContentRole::BookTitle | ContentRole::ChapterTitle | ContentRole::PartTitle => {
                    doc.title = Some(title_field(ctx, item_id, TitlePart::Title))
                }
            }
        }
        // Give each present prose document its caret-aware spell-check range session. Empty until
        // `attach_spell` sets a checker; it lives as long as the `OpenDoc` (its `Drop` retires the
        // highlight layer).
        doc.spell_main = doc.main.as_ref().map(|f| SpellSession::new(&f.doc));
        doc.spell_synopsis = doc.synopsis.as_ref().map(|f| SpellSession::new(&f.doc));
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

    /// Point this doc's spell sessions at the effective language list `tags`, in the squiggle
    /// `color`. Builds one [`SpellChecker`](crate::spellcheck) and hands it to each present prose
    /// session, which recomputes immediately; `None` (nothing installed/active) clears the
    /// squiggles — the degrade path. Only the two prose fields carry a session; the title/subtitle
    /// are plain `Signal<String>`.
    pub fn attach_spell(&self, spell: &SpellcheckService, tags: &str, color: Color) {
        let checker = spell.build_checker(tags);
        if let Some(s) = &self.spell_main {
            s.set_checker(checker.clone(), color);
        }
        if let Some(s) = &self.spell_synopsis {
            s.set_checker(checker, color);
        }
    }

    /// The caret-aware spell session on the main prose document, if any — the editor build feeds it
    /// the focused view's caret. `None` for a doc with no main prose (a pure container).
    pub fn spell_main(&self) -> Option<Rc<SpellSession>> {
        self.spell_main.clone()
    }

    /// The caret-aware spell session on the synopsis document, if any.
    pub fn spell_synopsis(&self) -> Option<Rc<SpellSession>> {
        self.spell_synopsis.clone()
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
    /// The spell-check engine, set once by `App`. `None` until then (headless tests, or before
    /// the first project loads) — every attach is then a no-op, so opening still works.
    spell: RefCell<Option<SpellcheckService>>,
    /// The squiggle colour, resolved from a theme role by `App` (updated on theme change).
    squiggle: Cell<Color>,
    /// The open project, for resolving each item's effective language (item → nearest Book →
    /// Work). Set by `App` on `LoadWork`/`NewWork`.
    work_id: Cell<Option<u64>>,
    work_lang: RefCell<String>,
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
                spell: RefCell::new(None),
                // A sensible default until `App` resolves the theme's error role.
                squiggle: Cell::new(Color::rgb(202, 66, 60)),
                work_id: Cell::new(None),
                work_lang: RefCell::new(String::new()),
            }),
        }
    }

    /// Install the spell-check engine (once, from `App`). Until this is set, spell attaches are
    /// no-ops and the app behaves exactly as before spell-check existed.
    pub fn set_spellcheck(&self, spell: SpellcheckService) {
        *self.inner.spell.borrow_mut() = Some(spell);
    }

    /// Set the squiggle colour (a theme error role, resolved by `App`). Re-attaches only when
    /// the colour actually changed (a theme switch) — `App` calls this on every rebuild, so an
    /// unconditional re-attach would be needless churn.
    pub fn set_squiggle_color(&self, color: Color) {
        if self.inner.squiggle.get() != color {
            self.inner.squiggle.set(color);
            self.attach_all();
        }
    }

    /// Point the store at the open project's default language, for the effective-language
    /// resolution. Called on `LoadWork`/`NewWork`; does not itself re-attach (the caller pairs
    /// it with [`attach_all`](Self::attach_all) once personal words are also loaded).
    pub fn set_project_language(&self, work_id: Option<u64>, work_lang: String) {
        self.inner.work_id.set(work_id);
        *self.inner.work_lang.borrow_mut() = work_lang;
    }

    /// Re-attach the spell-checker to **every** open document — the single path for install,
    /// remove, mute, language-change, focus-regain, and theme change. Recomputes each item's
    /// effective language through the same resolver search uses.
    pub fn attach_all(&self) {
        let Some(spell) = self.inner.spell.borrow().clone() else {
            return;
        };
        let color = self.inner.squiggle.get();
        let map = self.language_map();
        let work_lang = self.inner.work_lang.borrow().clone();
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            let tags = map
                .get(&doc.item_id)
                .cloned()
                .unwrap_or_else(|| work_lang.clone());
            doc.attach_spell(&spell, &tags, color);
        }
    }

    /// Attach the spell-checker to one freshly-built doc (on open / rebuild).
    fn attach_one(&self, doc: &Rc<OpenDoc>) {
        let Some(spell) = self.inner.spell.borrow().clone() else {
            return;
        };
        let map = self.language_map();
        let tags = map
            .get(&doc.item_id)
            .cloned()
            .unwrap_or_else(|| self.inner.work_lang.borrow().clone());
        doc.attach_spell(&spell, &tags, self.inner.squiggle.get());
    }

    /// Every distinct language tag the open project actually uses — the union across every
    /// item's effective list plus the Work's default. Drives the post-open missing-dictionary
    /// scan (Step 10).
    pub fn project_languages(&self) -> std::collections::BTreeSet<String> {
        let mut out: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for list in self.language_map().values() {
            out.extend(skribisto_model::language::all(list).map(str::to_string));
        }
        out.extend(
            skribisto_model::language::all(&self.inner.work_lang.borrow()).map(str::to_string),
        );
        out
    }

    /// The effective language list of one item (item → nearest Book → Work), for the
    /// Inspector's inherited-language placeholder. Falls back to the Work's default.
    pub fn effective_language(&self, item_id: u64) -> String {
        self.language_map()
            .get(&item_id)
            .cloned()
            .unwrap_or_else(|| self.inner.work_lang.borrow().clone())
    }

    /// The effective language tag list of every item in the open project, resolved through the
    /// **shared** `skribisto_model::language` chain (item → nearest Book → Work) so spell-check
    /// and search never disagree about what language a scene is in.
    fn language_map(&self) -> HashMap<u64, String> {
        let mut map = HashMap::new();
        let Some(work_id) = self.inner.work_id.get() else {
            return map;
        };
        let work_lang = self.inner.work_lang.borrow().clone();
        let ctx = &*self.inner.app_ctx;
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            // Only the three fields the resolver reads; document order is preserved by
            // `get_binder_item_multi` (relationship order), which the "nearest Book" scan needs.
            let items: Vec<BinderItem> = binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|dto| BinderItem {
                    id: dto.id,
                    sub_role: dto.sub_role.clone(),
                    dict_language: dto.dict_language.clone(),
                    ..Default::default()
                })
                .collect();
            skribisto_model::language::tags_in_binder(&work_lang, &items, &mut map);
        }
        map
    }

    /// The aggregate edit signal — bind the debounced autosave to it.
    pub fn edited_any(&self) -> Signal<u64> {
        self.inner.edited.clone()
    }

    /// Look up an **already-open** doc without changing its reference count — for a
    /// read-only consumer (the status-bar word count) that must not perturb the
    /// open/release lifecycle the editor panes own. `None` if the item isn't open.
    pub fn peek(&self, item_id: u64) -> Option<Rc<OpenDoc>> {
        self.inner.open.borrow().get(&item_id).map(|e| e.doc.clone())
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
        self.attach_one(&doc);
        Some(doc)
    }

    /// Rebuild `item_id`'s document **in place**, keeping its reference count.
    ///
    /// An `OpenDoc`'s fields are decided once, at construction, from the constraint
    /// matrix for the item's `(role, sub_role)`. A Promote rewrites that type, so every
    /// cached doc for it is now wrong — it may hold a prose field the new type has no
    /// room for, or be missing one it now has. Releasing is not enough: while any other
    /// holder still references the entry, the stale doc stays in the map and the next
    /// `open()` hands it straight back.
    ///
    /// Returns the fresh doc, or `None` if the item isn't open (or can't be read).
    pub fn rebuild(&self, item_id: u64, stack: Option<u64>) -> Option<Rc<OpenDoc>> {
        let old = self
            .inner
            .open
            .borrow()
            .get(&item_id)
            .map(|e| e.doc.clone())?;
        let _ = old.flush(stack);

        self.inner.item_probe.set_id(Some(item_id));
        let item = self.inner.item_probe.dto()?;
        let contents = self.load_contents(item_id, &item.role, &item.sub_role);
        let fresh = Rc::new(OpenDoc::build(
            &self.inner.app_ctx,
            item_id,
            &item.role,
            &item.sub_role,
            &contents,
            self.inner.edited.clone(),
        ));
        if let Some(entry) = self.inner.open.borrow_mut().get_mut(&item_id) {
            entry.doc = fresh.clone();
        }
        self.attach_one(&fresh);
        Some(fresh)
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

    /// Reload only the docs among `item_ids` that are **currently open**, discarding
    /// their live edits and re-reading each present field from disk.
    ///
    /// For a use case that rewrote a `Content` row out from under a live editor — a
    /// project-wide replace being the case here: `replace_in_project` edits the
    /// persisted prose directly, so any open tab of a touched item now shows the
    /// old text until it is re-read. Unlike [`open`](Self::open)+`reload`+`release`,
    /// this never *builds* a doc that wasn't already open (that would parse the
    /// whole item only to evict it a line later), and unlike [`rebuild`](Self::rebuild)
    /// it keeps the same `Rc<OpenDoc>` (the `(role, sub_role)` didn't change, only
    /// the text) so live tab views stay bound. The caller must pump a frame after
    /// (`set_djot` only queues a document event).
    pub fn reload_open(&self, item_ids: &[u64]) {
        let map = self.inner.open.borrow();
        for id in item_ids {
            if let Some(entry) = map.get(id) {
                entry.doc.reload();
            }
        }
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
