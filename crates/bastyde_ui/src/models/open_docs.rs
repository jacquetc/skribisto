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
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use bastyde::prelude::{BuildContext, Signal};
use bastyde::text_document::Color;

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, content_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItem, BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
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
    /// Whether the synopsis pane is currently shown (the global setting, mirrored here by `App`).
    /// A freshly-opened doc's synopsis spell session inherits this, so a re-attach never
    /// re-tokenises a hidden synopsis. Default `true`.
    synopsis_visible: Cell<bool>,
    /// The open project, for resolving each item's effective language (its own tag, else
    /// the Work's). Set by `App` on `LoadWork`/`NewWork`.
    work_id: Cell<Option<u64>>,
    work_lang: RefCell<String>,
    /// The memoised [`language_map`](OpenDocsStore::language_map), with the binder
    /// [fingerprint](LangFingerprint) it was built from.
    ///
    /// The uncached call fetched and cloned every `BinderItem` in the project to build
    /// the whole map at once. `open()` attaches spell-check to each
    /// freshly-built doc, so a container stream opening one document per row paid that
    /// whole-project walk once per row — O(rows × items). Cached, the walk happens once
    /// per structural change instead.
    lang_cache: RefCell<Option<(LangFingerprint, HashMap<u64, String>)>>,
}

/// What the cached language map is keyed on: the open work, its default language, and
/// every binder item id.
///
/// Creates and removals change the id sequence, so both drop the cache. Order is folded in
/// too, which is now *conservative* rather than load-bearing: an item's language no longer
/// depends on its neighbours (`skribisto_model::language::tags_in_binder` resolves an item's
/// own tag, else the Work's), so a pure reorder cannot change any answer. It stays in the
/// hash because a reorder is rare and cheap to over-invalidate, and because dropping it
/// would buy a subtle dependency on the resolver never regaining a positional rule.
///
/// Deliberately *not* covered: an item's own `dict_language` changing in place, which leaves
/// the id sequence identical. Detecting that here would mean re-reading every item — exactly
/// the cost this exists to avoid — so it is caught from the other side instead, by
/// [`OpenDocsStore::wire`]'s subscription to the `BinderItem::Updated` event. Shape is
/// fingerprinted; in-place fields are pushed. Between them the cache has no blind spot —
/// including "Apply to children", which writes `dict_language` on every descendant through
/// that same event.
type LangFingerprint = u64;

/// Fingerprint the inputs the language map is derived from that are cheap to read:
/// the work, its default language, and the ordered item ids of every binder.
fn fingerprint_of(work_id: Option<u64>, work_lang: &str, shape: &[Vec<u64>]) -> LangFingerprint {
    let mut hasher = DefaultHasher::new();
    work_id.hash(&mut hasher);
    work_lang.hash(&mut hasher);
    shape.hash(&mut hasher);
    hasher.finish()
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
                synopsis_visible: Cell::new(true),
                work_id: Cell::new(None),
                work_lang: RefCell::new(String::new()),
                lang_cache: RefCell::new(None),
            }),
        }
    }

    /// Install the spell-check engine (once, from `App`). Until this is set, spell attaches are
    /// no-ops and the app behaves exactly as before spell-check existed.
    pub fn set_spellcheck(&self, spell: SpellcheckService) {
        *self.inner.spell.borrow_mut() = Some(spell);
    }

    /// Subscribe the cached language map to the edits its
    /// [fingerprint](LangFingerprint) cannot see.
    ///
    /// The fingerprint covers the binder's *shape*, so a create, a removal or a move
    /// invalidates the map on its own. An item's own `dict_language` or `sub_role`
    /// changing in place leaves that shape byte-for-byte identical, and re-reading every
    /// item to notice is the very cost the cache exists to avoid — so the entity event
    /// pushes it instead. `BinderItem::Updated` covers both: the Inspector's language
    /// pill writes `dict_language` through it, and a Promote rewrites `sub_role` through
    /// it.
    ///
    /// Call from `build`, on **every** build. `BuildContext::subscribe_event` scopes a
    /// subscription to the widget's current build and drops it on the next, so a
    /// "subscribe once" guard would make the store go deaf the first time `App` rebuilt;
    /// re-subscribing cannot duplicate, because the old callback is already gone.
    ///
    /// The closure may hold the store by value — unlike the stream view-model's, this
    /// subscription is owned by the widget tree, not by the thing it captures, so there
    /// is no `Rc` cycle to close.
    pub fn wire(&self, ctx: &mut BuildContext) {
        let me = self.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
            move |_event: &Event| me.invalidate_language_cache(),
        );
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

    /// Track whether the synopsis pane is shown (the global setting), and push it to every open
    /// doc's synopsis spell session. Hidden → that session goes inactive and stops paying for
    /// re-attaches it can't display; shown → it schedules one catch-up rebuild. `App` calls this
    /// on the setting's change and once to seed it.
    pub fn set_synopsis_visible(&self, visible: bool) {
        self.inner.synopsis_visible.set(visible);
        for entry in self.inner.open.borrow().values() {
            if let Some(s) = entry.doc.spell_synopsis() {
                s.set_active(visible);
            }
        }
    }

    /// Point the store at the open project's default language, for the effective-language
    /// resolution. Called on `LoadWork`/`NewWork`; does not itself re-attach (the caller pairs
    /// it with [`attach_all`](Self::attach_all) once personal words are also loaded).
    pub fn set_project_language(&self, work_id: Option<u64>, work_lang: String) {
        self.inner.work_id.set(work_id);
        *self.inner.work_lang.borrow_mut() = work_lang;
        // A different project (or default language) resolves every item differently.
        self.invalidate_language_cache();
    }

    /// Re-attach the spell-checker to **every** open document — the single path for install,
    /// remove, mute, language-change, focus-regain, and theme change. Recomputes each item's
    /// effective language through the same resolver search uses.
    pub fn attach_all(&self) {
        let Some(spell) = self.inner.spell.borrow().clone() else {
            return;
        };
        let color = self.inner.squiggle.get();
        // Belt and braces. [`wire`](Self::wire) already drops the map when an item's
        // language changes, but every language edit also funnels through here, and
        // `attach_all` is rare enough that re-resolving costs nothing. It also keeps a
        // store that was never wired (a headless test) correct.
        self.invalidate_language_cache();
        let work_lang = self.inner.work_lang.borrow().clone();
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        // Resolve every doc's tags under one borrow of the map, then attach outside it.
        let attachments: Vec<(Rc<OpenDoc>, String)> = self.with_language_map(|map| {
            docs.into_iter()
                .map(|doc| {
                    let tags = map
                        .get(&doc.item_id)
                        .cloned()
                        .unwrap_or_else(|| work_lang.clone());
                    (doc, tags)
                })
                .collect()
        });
        for (doc, tags) in attachments {
            doc.attach_spell(&spell, &tags, color);
        }
    }

    /// Attach the spell-checker to one freshly-built doc (on open / rebuild).
    fn attach_one(&self, doc: &Rc<OpenDoc>) {
        let Some(spell) = self.inner.spell.borrow().clone() else {
            return;
        };
        // Inherit the current synopsis visibility *before* attaching, so a doc opened while the
        // synopsis pane is hidden never eagerly tokenises its (possibly huge) synopsis — its
        // `set_checker` sees the inactive flag and defers.
        if let Some(s) = doc.spell_synopsis() {
            s.set_active(self.inner.synopsis_visible.get());
        }
        let tags = self.language_for(doc.item_id);
        doc.attach_spell(&spell, &tags, self.inner.squiggle.get());
    }

    /// Every distinct language tag the open project actually uses — the union across every
    /// item's effective list plus the Work's default. Drives the post-open missing-dictionary
    /// scan (Step 10).
    pub fn project_languages(&self) -> std::collections::BTreeSet<String> {
        let mut out: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        self.with_language_map(|map| {
            for list in map.values() {
                out.extend(skribisto_model::language::all(list).map(str::to_string));
            }
        });
        out.extend(
            skribisto_model::language::all(&self.inner.work_lang.borrow()).map(str::to_string),
        );
        out
    }

    /// The effective language list of one item (its own tag, else the Work's), for the
    /// Inspector's placeholder. Falls back to the Work's default.
    pub fn effective_language(&self, item_id: u64) -> String {
        self.language_for(item_id)
    }

    /// One item's effective language list, read through the cached map.
    fn language_for(&self, item_id: u64) -> String {
        self.with_language_map(|map| map.get(&item_id).cloned())
            .unwrap_or_else(|| self.inner.work_lang.borrow().clone())
    }

    /// Drop the cached language map, forcing the next read to re-resolve from the items.
    ///
    /// For the edits the fingerprint cannot see — an item's own `dict_language` or
    /// `sub_role` changing in place. See [`LangFingerprint`].
    fn invalidate_language_cache(&self) {
        *self.inner.lang_cache.borrow_mut() = None;
    }

    /// Run `f` against the project's language map, rebuilding it first if the binder's
    /// shape changed since it was cached.
    ///
    /// Passes the map by reference rather than returning it: a clone would be one
    /// `String` allocation per item on every call, which is most of what the cache is
    /// here to avoid.
    fn with_language_map<R>(&self, f: impl FnOnce(&HashMap<u64, String>) -> R) -> R {
        let shape = self.binder_shape();
        let fingerprint = fingerprint_of(
            self.inner.work_id.get(),
            &self.inner.work_lang.borrow(),
            &shape,
        );
        let fresh = matches!(&*self.inner.lang_cache.borrow(), Some((fp, _)) if *fp == fingerprint);
        if !fresh {
            // Build before taking the cache's mutable borrow — the build reads the
            // backend, never the cache.
            let map = self.build_language_map(&shape);
            *self.inner.lang_cache.borrow_mut() = Some((fingerprint, map));
        }
        let cache = self.inner.lang_cache.borrow();
        match &*cache {
            Some((_, map)) => f(map),
            // Unreachable: just filled above. Total rather than `expect`.
            None => f(&HashMap::new()),
        }
    }

    /// The open project's binder item ids, per binder, in document order.
    ///
    /// The cheap half of language resolution: ids only — no `BinderItem` is fetched or
    /// cloned — which is what makes it affordable to check on every read.
    fn binder_shape(&self) -> Vec<Vec<u64>> {
        let Some(work_id) = self.inner.work_id.get() else {
            return Vec::new();
        };
        let ctx = &*self.inner.app_ctx;
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default()
            .into_iter()
            .map(|binder_id| {
                binder_commands::get_binder_relationship(
                    ctx,
                    &binder_id,
                    &BinderRelationshipField::BinderItems,
                )
                .unwrap_or_default()
            })
            .collect()
    }

    /// The effective language tag list of every item in the open project, resolved through the
    /// **shared** `skribisto_model::language` rule (an item's own tag, else the Work's) so
    /// spell-check and search never disagree about what language a scene is in.
    ///
    /// The expensive half: one `BinderItem` fetched and cloned per item. Called only when
    /// [`binder_shape`](Self::binder_shape) says the project changed.
    fn build_language_map(&self, shape: &[Vec<u64>]) -> HashMap<u64, String> {
        let mut map = HashMap::new();
        if self.inner.work_id.get().is_none() {
            return map;
        }
        let work_lang = self.inner.work_lang.borrow().clone();
        let ctx = &*self.inner.app_ctx;
        for item_ids in shape {
            // Only the fields the resolver reads. Order is irrelevant to the resolver now
            // (each item answers for itself), but relationship order is what we get anyway.
            let items: Vec<BinderItem> = binder_item_commands::get_binder_item_multi(ctx, item_ids)
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
        // The incoming project re-resolves from scratch; don't hold the old map's
        // strings until then.
        self.invalidate_language_cache();
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

    /// The language cache is only as good as what its fingerprint distinguishes. It is
    /// the *shape* half of the contract on [`LangFingerprint`]: every structural change
    /// that can move an item under a different Book — and so change the language it
    /// inherits — must change the fingerprint.
    #[test]
    fn fingerprint_distinguishes_every_structural_change() {
        let base = fingerprint_of(Some(1), "en-US", &[vec![10, 11, 12]]);

        assert_eq!(
            base,
            fingerprint_of(Some(1), "en-US", &[vec![10, 11, 12]]),
            "same shape must reuse the cache"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(2), "en-US", &[vec![10, 11, 12]]),
            "a different work resolves every item differently"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), "fr-FR", &[vec![10, 11, 12]]),
            "the work's default language is the fallback for every item"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), "en-US", &[vec![10, 11, 12, 13]]),
            "a created item"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), "en-US", &[vec![10, 12]]),
            "a removed item"
        );
        // The load-bearing one: a move keeps the same ids, so only *order* betrays it —
        // and order is exactly what decides which Book an item inherits from.
        assert_ne!(
            base,
            fingerprint_of(Some(1), "en-US", &[vec![12, 11, 10]]),
            "a reorder can change which Book an item sits under"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), "en-US", &[vec![10, 11], vec![12]]),
            "the same ids split across two binders is a different shape"
        );
        assert_ne!(
            base,
            fingerprint_of(None, "en-US", &[vec![10, 11, 12]]),
            "no open project"
        );
    }

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
