// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The spell-checking engine — an internal service, no reactive state of its own (the same
//! shelf as `open_registry` / `ipc`).
//!
//! It loads a [`spellbook::Dictionary`] per language **once**, shared by every open document
//! that needs it (reversing the old app's one-instance-per-editor-tab waste), and produces a
//! [`SpellChecker`] — a pure misspell predicate. A per-document, **caret-aware** [`SpellSession`]
//! (a host-driven *range session*, mirroring the find feature's `FindSession`) queries that
//! predicate to push the squiggle ranges, omitting the word under the caret. The re-attach
//! *wiring* (dictionary install/remove, mute, language change, `close_work`) lives in
//! `models::open_docs` and `app.rs`; the caret wiring lives in `tabs::shared::editor`.
//!
//! Three view-models sit on top of the engine: [`DictionariesViewModel`] (download/remove/
//! licence-acceptance for the machine-wide dictionary catalogue), [`AddDictionaryViewModel`]
//! (the "Add dictionary" form over it), and [`UserDictionaryViewModel`] (the per-project
//! personal word list). [`add_dictionary_panel`], [`language_pill_field`] and
//! [`toggle_button`] are their views.
//!
//! ## The Firefox model
//!
//! A document's `dict_language` is a list; a word is a mistake only when **every** active
//! (non-muted, installed) dictionary rejects it. The personal word set is checked first, so
//! correctly-spelled prose costs one lookup and only a genuine miss fans out across languages.
//!
//! ## Muting is session state
//!
//! The green check on a language pill toggles a *session* mute — held here, cleared on
//! [`clear`](SpellcheckService::clear) (i.e. `close_work`), never persisted and never in
//! `dict_language`. A muted language is still declared by the text; it is simply left out of
//! the active set when a highlighter is built.
//!
//! ## Encoding
//!
//! [`spellbook::Dictionary::new`] takes `&str`, and spellbook does not honour the `.aff`
//! `SET` encoding directive. Our own downloads are UTF-8, but a *system* dictionary may be
//! ISO-8859-x — so the loader reads raw bytes, detects the declared encoding, and transcodes
//! to UTF-8 before handing the strings to spellbook. A dictionary that still fails to parse is
//! simply absent (cached as `None`), never a crash.

pub(crate) mod add_dictionary_panel;
mod add_dictionary_vm;
mod dictionaries_vm;
pub(crate) mod dictionary_registry;
pub(crate) mod language_pill_field;
pub(crate) mod toggle_button;
mod user_dictionary_vm;

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use teksilo::core::WidgetId;
use teksilo::prelude::Signal;
use teksilo::text_document::{
    Color, DocumentEvent, HighlightFormat, RangeHighlight, SessionId, Subscription, TextDocument,
    UnderlineStyle,
};

use skribisto_model::language;

// The engine is a crate of its own — Hunspell in, a predicate and a ranked
// suggestion list out, with no store, no widget and no application paths. What
// stays in this module is everything that knows *where* a dictionary lives,
// *when* to re-check and *how* to paint a squiggle, all three of which are
// properties of this editor rather than of spell-checking.
pub use spellcheck_engine::{SpellChecker, validate_dictionary_files, word_positions};

pub use add_dictionary_vm::AddDictionaryViewModel;
pub use dictionaries_vm::{DictionariesViewModel, InstallDictError};
pub use user_dictionary_vm::UserDictionaryViewModel;

/// `<data_dir>/dictionaries` — where our downloads live and the loader looks first. The single
/// definition, shared by discovery ([`crate::models`]) and download ([`crate::spellcheck`]).
pub(crate) fn downloaded_dictionaries_dir() -> Option<PathBuf> {
    crate::identity::app_paths().map(|p| p.data_dir().join("dictionaries"))
}

/// The read-only system dictionary directories to probe, in priority order, per OS. Windows
/// has none (apps ship or download their own). Under Flatpak these simply don't exist inside
/// the sandbox, so this quietly finds nothing — by design.
pub(crate) fn system_dictionary_dirs() -> Vec<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        vec![
            PathBuf::from("/usr/share/hunspell"),
            PathBuf::from("/usr/local/share/hunspell"),
            PathBuf::from("/usr/share/myspell/dicts"),
        ]
    }
    #[cfg(target_os = "macos")]
    {
        let mut v = vec![PathBuf::from("/Library/Spelling")];
        if let Some(home) = std::env::var_os("HOME") {
            v.push(PathBuf::from(home).join("Library/Spelling"));
        }
        v
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Vec::new()
    }
}

/// The `.aff`/`.dic` file pair for a registry id: our download first, then a system copy under
/// any of the entry's `system_basenames`.
fn locate(id: &str) -> Option<(PathBuf, PathBuf)> {
    if let Some(dir) = downloaded_dictionaries_dir() {
        let aff = dir.join(format!("{id}.aff"));
        let dic = dir.join(format!("{id}.dic"));
        if aff.is_file() && dic.is_file() {
            return Some((aff, dic));
        }
    }
    let basenames = dictionary_registry::by_id(id)
        .map(|e| e.system_basenames.clone())
        .unwrap_or_default();
    for dir in system_dictionary_dirs() {
        for base in &basenames {
            let aff = dir.join(format!("{base}.aff"));
            let dic = dir.join(format!("{base}.dic"));
            if aff.is_file() && dic.is_file() {
                return Some((aff, dic));
            }
        }
    }
    None
}

/// Locate, read (transcoding as needed), and parse a dictionary. `None` if the files are
/// absent or spellbook can't parse them (an unusable dictionary is absent, not a crash).
fn load(id: &str) -> Option<Arc<spellbook::Dictionary>> {
    let (aff_path, dic_path) = locate(id)?;
    spellcheck_engine::load_pair(&aff_path, &dic_path)
}

/// The wavy spell-check underline format in `color` (never a hex literal — the caller resolves
/// `color` from a theme role). Paint-only, so it stays out of the accessibility tree and takes the
/// cheap recolour path.
fn spell_format(color: Color) -> HighlightFormat {
    HighlightFormat {
        underline_style: Some(UnderlineStyle::SpellCheckUnderline),
        underline_color: Some(color),
        ..Default::default()
    }
}

/// The app-wide spell-check engine: a per-language dictionary cache (genuinely Tier 1 —
/// loading a dictionary is expensive and language-keyed, not Work-keyed, so every open Work
/// shares one), the master switch, and the mute-version counter. Cloneable (shares one `Rc`
/// state) — the **same** instance is handed to every window/`WorkSession` (see
/// `shell::windows::ProjectWindowFactory`'s `spellcheck` field), unlike the per-session
/// view-models that get a fresh instance per open Work.
///
/// **The session mute set and the personal words are Tier 2 (per open Work)**, keyed by
/// `work_id` in [`Inner::muted`]/[`Inner::personal`] — *not* one flat set. Before the
/// multi-Work migration there was always at most one open Work, so a flat set and "per Work"
/// were the same thing; with two Works genuinely open at once, closing one must drop only
/// *its own* mutes/personal words, never a still-open sibling's (see [`Self::clear`]).
#[derive(Clone)]
pub struct SpellcheckService {
    inner: Rc<Inner>,
}

struct Inner {
    /// id → loaded dictionary, or `None` for "tried and absent/unusable" (so a miss isn't
    /// re-attempted every keystroke). Tier 1 — shared by every open Work.
    cache: RefCell<HashMap<String, Option<Arc<spellbook::Dictionary>>>>,
    /// Session-muted language keys (resolved registry ids), keyed by `work_id`. Tier 2 — a
    /// Work's own entry is dropped on `close_work` (see [`SpellcheckService::clear`]), never
    /// every Work's.
    muted: RefCell<HashMap<u64, HashSet<String>>>,
    /// Bumped on every mute change **and every master-switch flip** so a language-pill field
    /// rebuilds its check marks — the pills bind this and nothing else, so a switch that did
    /// not bump it would leave them looking live while nothing was being checked. One counter
    /// for the whole process: a pill field rebuilding on an unrelated Work's mute change is
    /// wasted work, not a correctness bug (it re-reads its own Work's `is_muted` and gets the
    /// same answer) — see the migration report's toast-policy note for the same class of
    /// accepted, non-corrupting cross-Work chatter.
    mute_version: Signal<u64>,
    /// The master switch (`SPELLCHECK_ENABLED_KEY`, default on). Mirrored here by `App` from
    /// the settings store so [`build_checker`](SpellcheckService::build_checker) — the single
    /// gate every document passes through — can answer "off" before touching a dictionary.
    /// Tier 1 — an app-wide preference, not a Work property.
    enabled: Cell<bool>,
    /// Each open Work's personal words (`DictWord`), keyed by `work_id`. Tier 2 — see `muted`.
    personal: RefCell<HashMap<u64, HashSet<String>>>,
}

impl SpellcheckService {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Inner {
                cache: RefCell::new(HashMap::new()),
                muted: RefCell::new(HashMap::new()),
                mute_version: Signal::new(0),
                enabled: Cell::new(true),
                personal: RefCell::new(HashMap::new()),
            }),
        }
    }

    /// A version counter bumped whenever the mute set changes — bind a pill field to it to
    /// rebuild its green checks live (from either the Inspector or the Settings pane).
    pub fn mute_version(&self) -> Signal<u64> {
        self.inner.mute_version.clone()
    }

    /// Whether spell-checking is on at all (the master switch).
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled.get()
    }

    /// Drive the master switch. Returns whether it actually changed, so the caller only
    /// re-attaches on a real flip.
    ///
    /// A real flip also bumps [`mute_version`](Self::mute_version): the language pills bind
    /// that signal alone, so without this they would keep showing live green checks while the
    /// switch was off — the same "the UI says one thing, the engine does another" bug the
    /// switch exists to end.
    pub fn set_enabled(&self, on: bool) -> bool {
        let changed = self.inner.enabled.replace(on) != on;
        if changed {
            let v = self.inner.mute_version.get();
            self.inner.mute_version.set(v.wrapping_add(1));
        }
        changed
    }

    /// The cache key for a tag: its resolved registry id, or the tag itself if unrecognised —
    /// so `fr` and `fr-FR` mute/resolve as one language.
    fn key_of(tag: &str) -> String {
        dictionary_registry::resolve_token(tag)
            .map(str::to_string)
            .unwrap_or_else(|| tag.to_string())
    }

    fn dict(&self, id: &str) -> Option<Arc<spellbook::Dictionary>> {
        if let Some(hit) = self.inner.cache.borrow().get(id) {
            return hit.clone();
        }
        let loaded = load(id);
        self.inner
            .cache
            .borrow_mut()
            .insert(id.to_string(), loaded.clone());
        loaded
    }

    /// Whether spell-checking for `tag`'s language is muted this session, for `work_id`'s
    /// Work. `work_id: None` (no project open) is never muted — there is nothing to mute yet.
    pub fn is_muted(&self, tag: &str, work_id: Option<u64>) -> bool {
        let Some(work_id) = work_id else { return false };
        self.inner
            .muted
            .borrow()
            .get(&work_id)
            .is_some_and(|set| set.contains(&Self::key_of(tag)))
    }

    /// Toggle `work_id`'s Work's session mute for `tag`'s language. Returns whether the set
    /// changed (so the caller only re-attaches on a real change). A no-op (never "changed")
    /// with no `work_id` — muting means nothing without an open Work to scope it to.
    pub fn set_muted(&self, tag: &str, muted: bool, work_id: Option<u64>) -> bool {
        let Some(work_id) = work_id else { return false };
        let key = Self::key_of(tag);
        let changed = {
            let mut map = self.inner.muted.borrow_mut();
            let set = map.entry(work_id).or_default();
            if muted {
                set.insert(key)
            } else {
                set.remove(&key)
            }
        };
        if changed {
            let v = self.inner.mute_version.get();
            self.inner.mute_version.set(v.wrapping_add(1));
        }
        changed
    }

    /// Replace `work_id`'s Work's personal words (from its `DictWord` set).
    pub fn set_personal(&self, work_id: u64, words: HashSet<String>) {
        self.inner.personal.borrow_mut().insert(work_id, words);
    }

    /// Drop the loaded-dictionary cache only (keeping every Work's session mutes + personal
    /// words), so the next attach re-reads disk. Called when a dictionary is installed or
    /// removed: without this, `dict()`'s per-id cache would keep serving a stale entry — a
    /// cached miss would hide a fresh install, and a cached `Arc` would keep a just-removed
    /// dictionary alive (so squiggles would neither appear nor degrade until the project is
    /// reopened). Tier 1 — shared by every open Work, so this is never per-`work_id`.
    pub fn invalidate_dictionaries(&self) {
        self.inner.cache.borrow_mut().clear();
    }

    /// Drop `work_id`'s Work's own project-scoped state — its session mutes and personal
    /// words. Called on `close_work`; a fresh project reloads lazily and starts unmuted.
    ///
    /// Removes exactly this Work's own map entry, never every open Work's: the dictionary
    /// *cache* is deliberately untouched (Tier 1, shared — see [`invalidate_dictionaries`](Self::invalidate_dictionaries)),
    /// and a still-open sibling Work's mutes/personal words under a different `work_id` are
    /// a different map entry, never reached by this call.
    pub fn clear(&self, work_id: u64) {
        let had_mutes = self
            .inner
            .muted
            .borrow()
            .get(&work_id)
            .is_some_and(|set| !set.is_empty());
        self.inner.muted.borrow_mut().remove(&work_id);
        self.inner.personal.borrow_mut().remove(&work_id);
        if had_mutes {
            let v = self.inner.mute_version.get();
            self.inner.mute_version.set(v.wrapping_add(1));
        }
    }

    /// The active (non-muted, installed) dictionaries for a tag list, primary first, deduped,
    /// under `work_id`'s Work's own mute set.
    fn active_dicts(
        &self,
        tags: &[String],
        work_id: Option<u64>,
    ) -> Vec<Arc<spellbook::Dictionary>> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        let muted = self.inner.muted.borrow();
        let my_mutes = work_id.and_then(|id| muted.get(&id));
        for tag in language::all(tags) {
            let key = Self::key_of(tag);
            if my_mutes.is_some_and(|set| set.contains(&key)) {
                continue;
            }
            if !seen.insert(key.clone()) {
                continue;
            }
            if let Some(d) = self.dict(&key) {
                out.push(d);
            }
        }
        out
    }

    /// Build a [`SpellChecker`] for a document's tag list under `work_id`'s Work, or `None`
    /// when nothing is active/installed (the caller then clears its session — the degrade
    /// path). `work_id` scopes both the mute set (via [`active_dicts`](Self::active_dicts))
    /// and the personal words baked into the returned checker — never a different open
    /// Work's, and never every open Work's merged together.
    pub fn build_checker(&self, tags: &[String], work_id: Option<u64>) -> Option<SpellChecker> {
        // The master switch, checked first: every document's checker is built here, so one
        // early return turns spell-check off everywhere — squiggles, `is_misspelled`, and the
        // context menu's corrections alike — without loading a dictionary or walking a tag.
        // Off reuses the existing degrade path (`None` → the session clears its ranges and
        // keeps its cheap empty layer attached), so nothing new has to be torn down.
        if !self.inner.enabled.get() {
            return None;
        }
        let dicts = self.active_dicts(tags, work_id);
        if dicts.is_empty() {
            return None;
        }
        let personal = work_id
            .and_then(|id| self.inner.personal.borrow().get(&id).cloned())
            .unwrap_or_default();
        Some(SpellChecker::new(dicts, personal))
    }
}

impl Default for SpellcheckService {
    fn default() -> Self {
        Self::new()
    }
}

/// A per-document, **caret-aware** spell-check highlighter — one host-driven *range session* on a
/// `TextDocument`. It recomputes the misspelled ranges on every edit and every caret-word change,
/// **omitting the word the caret currently sits in**, and pushes them with `set_session_ranges`.
/// That is the Word/Google-Docs behaviour: don't flag the word you're mid-typing; reveal it once
/// the caret leaves (a space / newline / punctuation, or a click / arrow away).
///
/// It mirrors `teksilo::widgets::rich_text::FindSession`'s lifecycle — a held `Subscription`, a
/// `dirty` flag drained by a per-frame `tick`, and a `Drop` that retires the layer. The caret is
/// read **live** at recompute time (via the closure a focused view supplies): the editor batches a
/// printable keystroke a frame behind the caret signal, so a value pushed earlier would be stale
/// exactly when a just-typed space should reveal the word.
/// Reads the focused editor's caret offset on demand.
type CaretProbe = Rc<dyn Fn() -> usize>;

pub struct SpellSession {
    doc: TextDocument,
    session: SessionId,
    /// Set by the `on_change` subscription (a `Send + Sync` closure) on an offset-moving edit;
    /// hence `Arc<AtomicBool>`, exactly like `FindSession.dirty`.
    content_dirty: Arc<AtomicBool>,
    /// Set by the caret / focus effects (UI thread only). Kept apart from `content_dirty` so a
    /// caret-only move re-derives the exemption from the cached misspelling set — O(misspellings) —
    /// rather than re-tokenising the whole document.
    caret_dirty: Cell<bool>,
    /// Whether this session's document is currently *shown* anywhere (default `true`). The
    /// synopsis session is set inactive while the synopsis pane is hidden (a global setting), so a
    /// re-attach — dictionary install, mute, language change, project load — does not pay a full
    /// O(document) re-tokenise of a 20k-word synopsis nobody can see. Turning it back on schedules
    /// one catch-up rebuild. The main-text session is always active.
    active: Cell<bool>,
    /// Held so the subscription lives as long as the session (dropping it unsubscribes).
    _sub: Subscription,
    checker: RefCell<Option<SpellChecker>>,
    color: Cell<Color>,
    /// The one focused view of this document, if any: `(widget id, a LIVE caret reader)`. Only its
    /// caret exempts a word; `None` = no view focused = nothing exempt. The reader is a closure so
    /// a headless test can inject a caret without a mounted editor (production reads
    /// `EditorHandle::cursor_position`).
    focused: RefCell<Option<(WidgetId, CaretProbe)>>,
    /// Every misspelling in the document, exemption **not** applied — the cache a caret move
    /// re-filters instead of re-scanning.
    all_ranges: RefCell<Vec<RangeHighlight>>,
    /// The last set pushed, so an unchanged recompute skips the repaint (`RangeHighlight: Eq`).
    last_ranges: RefCell<Vec<RangeHighlight>>,
    /// Identity `(start, length)` of the range the caret exempted at the last recompute, or
    /// `None` if it exempted nothing. The caret ticks every time it *moves*, but it stays inside
    /// the same word across a whole burst of keystrokes — and while it does, the exempted range
    /// is unchanged, so the filtered set is byte-identical to what was already pushed. Caching
    /// this identity lets a same-word caret move skip the O(all_ranges) filter + clone entirely
    /// (see [`apply_exemption`](Self::apply_exemption)); on a densely-flagged document that clone
    /// is the per-keystroke cost, run whether or not anything changed.
    last_exempt: Cell<Option<(usize, usize)>>,
    /// Test-only: how many times the slow path (the filter + clone) actually ran, so a test can
    /// prove a same-word caret move takes the fast path.
    #[cfg(test)]
    exemption_recomputes: Cell<usize>,
}

impl SpellSession {
    /// Create the range session on `doc` and subscribe to its edits. No checker yet — the ranges
    /// stay empty until [`set_checker`](Self::set_checker).
    pub fn new(doc: &TextDocument) -> Rc<Self> {
        let session = doc.add_range_session();
        let content_dirty = Arc::new(AtomicBool::new(false));
        let sub = {
            let content_dirty = content_dirty.clone();
            doc.on_change(move |event| {
                // Only offset-moving events need a re-scan — and never `HighlightPaintChanged`,
                // which our own `set_session_ranges` emits (reacting to it would self-loop). Same
                // filter `FindSession` uses.
                if matches!(
                    event,
                    DocumentEvent::ContentsChanged { .. }
                        | DocumentEvent::DocumentReset
                        | DocumentEvent::BlockCountChanged(_)
                        | DocumentEvent::FlowElementsInserted { .. }
                        | DocumentEvent::FlowElementsRemoved { .. }
                ) {
                    content_dirty.store(true, Ordering::Relaxed);
                }
            })
        };
        Rc::new(Self {
            doc: doc.clone(),
            session,
            content_dirty,
            caret_dirty: Cell::new(false),
            active: Cell::new(true),
            _sub: sub,
            checker: RefCell::new(None),
            color: Cell::new(Color::rgb(220, 50, 50)),
            focused: RefCell::new(None),
            all_ranges: RefCell::new(Vec::new()),
            last_ranges: RefCell::new(Vec::new()),
            last_exempt: Cell::new(None),
            #[cfg(test)]
            exemption_recomputes: Cell::new(0),
        })
    }

    /// Set the active checker + squiggle colour and recompute now. Called from app-level effects
    /// (dictionary install/remove, mute, language change) — a safe context, never inside a doc
    /// event, so recomputing directly cannot re-enter the `on_change` dispatch. `None` clears the
    /// squiggles (the degrade path) while leaving the cheap empty session attached.
    pub fn set_checker(&self, checker: Option<SpellChecker>, color: Color) {
        *self.checker.borrow_mut() = checker;
        self.color.set(color);
        // An invisible pane (a hidden synopsis) stores the new checker but defers the actual
        // O(document) re-tokenise until it is shown — flagging the rebuild as owed. A visible
        // pane rebuilds now; `forced` because `all_ranges` was just rebuilt, so the cached
        // exemption identity is stale and the fast path must not trust it.
        if self.active.get() {
            self.rebuild_all_ranges();
            self.apply_exemption(true);
        } else {
            self.content_dirty.store(true, Ordering::Relaxed);
        }
    }

    /// Show or hide this session (the synopsis session follows the global synopsis-pane setting;
    /// the main session stays active). Returning to active schedules one catch-up rebuild for any
    /// edit or re-attach that landed while it was hidden — drained on the next `tick`.
    pub fn set_active(&self, active: bool) {
        let was = self.active.replace(active);
        if active && !was {
            self.content_dirty.store(true, Ordering::Relaxed);
        }
    }

    /// Whether `word` is currently flagged as a misspelling by this document's
    /// active checker — the same predicate that draws the squiggle. `false` when
    /// no checker is active (no installed dictionary → nothing is "wrong"), which
    /// is why the editor's "Add to dictionary" item only offers flagged words. A
    /// word already in the personal dictionary is not flagged, so it is never
    /// re-offered.
    pub(crate) fn is_misspelled(&self, word: &str) -> bool {
        self.checker
            .borrow()
            .as_ref()
            .map(|c| c.misspelled(word))
            .unwrap_or(false)
    }

    /// Ranked corrections for `word` from this document's active checker — the installed
    /// dictionaries *and* the Work's personal words (see [`SpellChecker::suggest`]). Empty when no
    /// checker is active, and empty is a legitimate answer for a word nothing can correct: the
    /// menu says so rather than hiding the fact it looked.
    pub(crate) fn suggest(&self, word: &str) -> Vec<String> {
        self.checker
            .borrow()
            .as_ref()
            .map(|c| c.suggest(word))
            .unwrap_or_default()
    }

    /// A view gained focus and becomes the caret source. `caret` reads the **live** offset.
    pub fn on_focus(&self, view: WidgetId, caret: Rc<dyn Fn() -> usize>) {
        *self.focused.borrow_mut() = Some((view, caret));
        self.caret_dirty.set(true);
    }

    /// A view lost focus or was torn down: if it was the caret source, nothing is exempt now.
    pub fn on_blur(&self, view: WidgetId) {
        let was_source = self
            .focused
            .borrow()
            .as_ref()
            .is_some_and(|(v, _)| *v == view);
        if was_source {
            *self.focused.borrow_mut() = None;
            self.caret_dirty.set(true);
        }
    }

    /// The focused view's caret moved — re-derive the exemption next tick. (Ignored for a view
    /// that isn't the current caret source, so a background split pane can't steal the exemption.)
    pub fn on_caret(&self, view: WidgetId) {
        let is_source = self
            .focused
            .borrow()
            .as_ref()
            .is_some_and(|(v, _)| *v == view);
        if is_source {
            self.caret_dirty.set(true);
        }
    }

    /// Per frame: recompute only if something changed. A content edit re-tokenises the whole
    /// document; a caret-only move just re-filters the cache. Coalesces a burst of events into one
    /// recompute + at most one push.
    pub fn tick(&self) {
        // A hidden pane does no work and — crucially — leaves its dirty flags *set*, so the
        // catch-up rebuild it owes survives until it is shown again.
        if !self.active.get() {
            return;
        }
        let content = self.content_dirty.swap(false, Ordering::Relaxed);
        let caret = self.caret_dirty.replace(false);
        if !content && !caret {
            return;
        }
        if content {
            self.rebuild_all_ranges();
        }
        // A content edit rebuilt `all_ranges`, so force a full recompute; a caret-only move
        // leaves `all_ranges` intact and may take the fast path.
        self.apply_exemption(content);
    }

    /// Re-tokenise the whole document into `all_ranges` (every misspelling, no exemption). Empty
    /// when no checker is active. Char offsets are document-absolute (`block.position()` + the
    /// block-local char offset), the space `set_session_ranges` expects.
    ///
    /// This runs O(document) on each content edit (coalesced to once per frame by `tick`). That is
    /// fine for Skribisto's documents — one scene or one chapter, a few thousand words, ≈1-2 ms —
    /// which is why a caret-only move re-filters the cache instead (`apply_exemption`) rather than
    /// re-running this. If a single document ever grew unbounded, the follow-up is per-block range
    /// caching keyed off `ContentsChanged { position }` (re-tokenise only the edited block).
    fn rebuild_all_ranges(&self) {
        let mut ranges = Vec::new();
        if let Some(checker) = self.checker.borrow().as_ref() {
            let color = self.color.get();
            for block in self.doc.blocks() {
                let base = block.position();
                let text = block.text();
                for (char_off, len, word) in word_positions(&text) {
                    if checker.misspelled(word) {
                        ranges.push(RangeHighlight {
                            start: base + char_off,
                            length: len,
                            format: spell_format(color),
                        });
                    }
                }
            }
        }
        *self.all_ranges.borrow_mut() = ranges;
    }

    /// The identity `(start, length)` of the **first** cached range the caret falls in (inclusive
    /// of both ends), or `None`. A cheap comparison-only scan with an early exit — no clone — so
    /// it is safe to run on every caret move purely to decide whether the pushed set can change.
    fn caret_exempt_identity(&self, caret: Option<usize>) -> Option<(usize, usize)> {
        let c = caret?;
        self.all_ranges
            .borrow()
            .iter()
            .find(|r| c >= r.start && c <= r.start + r.length)
            .map(|r| (r.start, r.length))
    }

    /// Filter the cached misspellings by the live caret — drop the one word it sits in (inclusive
    /// of both ends, so typing at a word's end keeps it exempt) — and push, but only if the result
    /// changed since the last push (so typing *within* the exempt word repaints nothing).
    ///
    /// `forced` must be set whenever `all_ranges` was just rebuilt (a content edit / a new
    /// checker): the fast path below trusts the *cached* exemption identity, which only means
    /// anything while `all_ranges` is unchanged.
    ///
    /// The fast path is the point of this. The caret ticks on every move, but stays inside one
    /// word across a burst of typing; while it does, the exempted range is unchanged and the
    /// filtered set is byte-for-byte what was already pushed. Recognising that from the cached
    /// identity skips the O(all_ranges) filter + clone — the dominant per-keystroke cost on a
    /// densely-flagged document.
    fn apply_exemption(&self, forced: bool) {
        let caret = self.focused.borrow().as_ref().map(|(_, f)| f());
        let exempt = self.caret_exempt_identity(caret);
        if !forced && self.last_exempt.get() == exempt {
            return; // same word exempt, `all_ranges` unchanged → nothing to rebuild or push
        }
        self.last_exempt.set(exempt);
        #[cfg(test)]
        self.exemption_recomputes
            .set(self.exemption_recomputes.get() + 1);
        // Exempt **at most one** word — the first the caret falls in. This matters only where two
        // misspelled words touch with no separator (adjacent CJK / Hiragana characters, which
        // UAX#29 splits into one-char tokens): a caret on the shared boundary is inclusive-in both,
        // and without this cap both would drop. The contract is "the word the caret sits in",
        // singular. For separator-delimited scripts at most one ever matches, so this is a no-op.
        let mut exempted = false;
        let next: Vec<RangeHighlight> = self
            .all_ranges
            .borrow()
            .iter()
            .filter(|r| match caret {
                Some(c) if !exempted && c >= r.start && c <= r.start + r.length => {
                    exempted = true;
                    false
                }
                _ => true,
            })
            .cloned()
            .collect();
        if *self.last_ranges.borrow() != next {
            self.doc.set_session_ranges(self.session, next.clone());
            *self.last_ranges.borrow_mut() = next;
        }
    }
}

impl Drop for SpellSession {
    fn drop(&mut self) {
        // The `Subscription`'s own drop stops callback delivery but does NOT retire the highlight
        // layer — remove it explicitly, exactly as `FindSession` does.
        self.doc.remove_session(self.session);
    }
}

#[cfg(test)]
mod tests;
