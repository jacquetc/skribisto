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

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bastyde::core::WidgetId;
use bastyde::prelude::Signal;
use bastyde::text_document::{
    Color, DocumentEvent, HighlightFormat, RangeHighlight, SessionId, Subscription, TextDocument,
    UnderlineStyle,
};

use crate::dictionary_registry;
use skribisto_model::language;

/// `<data_dir>/dictionaries` — where our downloads live and the loader looks first. The single
/// definition, shared by discovery ([`crate::models`]) and download ([`crate::view_models`]).
pub(crate) fn downloaded_dictionaries_dir() -> Option<PathBuf> {
    bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .map(|p| p.data_dir().join("dictionaries"))
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

/// The encoding an `.aff` declares via its `SET <name>` directive (UTF-8 if none/unknown).
fn detect_encoding(aff_bytes: &[u8]) -> &'static encoding_rs::Encoding {
    // `SET` and the label are ASCII, so a lossy decode of the head is safe to scan.
    let head = String::from_utf8_lossy(&aff_bytes[..aff_bytes.len().min(1024)]);
    for line in head.lines() {
        if let Some(rest) = line.strip_prefix("SET ") {
            if let Some(enc) = encoding_rs::Encoding::for_label(rest.trim().as_bytes()) {
                return enc;
            }
        }
    }
    encoding_rs::UTF_8
}

/// Read an `.aff`/`.dic` pair to UTF-8 `String`s, transcoding from the `.aff`'s declared
/// encoding (which governs both files).
fn read_pair(aff_path: &Path, dic_path: &Path) -> Option<(String, String)> {
    let aff_bytes = std::fs::read(aff_path).ok()?;
    let enc = detect_encoding(&aff_bytes);
    let (aff, _, _) = enc.decode(&aff_bytes);
    let dic_bytes = std::fs::read(dic_path).ok()?;
    let (dic, _, _) = enc.decode(&dic_bytes);
    Some((aff.into_owned(), dic.into_owned()))
}

/// Locate, read (transcoding as needed), and parse a dictionary. `None` if the files are
/// absent or spellbook can't parse them (an unusable dictionary is absent, not a crash).
fn load(id: &str) -> Option<Arc<spellbook::Dictionary>> {
    let (aff_path, dic_path) = locate(id)?;
    let (aff, dic) = read_pair(&aff_path, &dic_path)?;
    spellbook::Dictionary::new(&aff, &dic).ok().map(Arc::new)
}

/// Check that an `.aff`/`.dic` pair can actually be used, running the **exact** steps [`load`]
/// does — read + transcode from the declared encoding, then parse with spellbook. Used by the
/// "Add dictionary" flow to refuse a bad pair up front (which would otherwise install silently
/// and simply never flag anything). A pair that validates here is one the engine can load.
pub(crate) fn validate_dictionary_files(aff_path: &Path, dic_path: &Path) -> Result<(), String> {
    let (aff, dic) = read_pair(aff_path, dic_path)
        .ok_or_else(|| "could not read the .aff / .dic files".to_string())?;
    spellbook::Dictionary::new(&aff, &dic)
        .map(|_| ())
        .map_err(|e| format!("not a valid Hunspell dictionary ({e:?})"))
}

/// Word tokens of a block, as `(char_offset, char_length, word)` — the coordinates
/// [`HighlightContext::set_format`] expects (character positions, not bytes). UAX#29 word
/// segmentation keeps contractions and elisions together (`don't`, `l'auteur`), for both the
/// straight `'` and the curly `’`.
///
/// `pub(crate)` so the editor's "Add to dictionary" menu resolves selection/caret words with the
/// **same** tokenizer the squiggles use — what is addable and what is flagged can never disagree.
pub(crate) fn word_positions(text: &str) -> Vec<(usize, usize, &str)> {
    use unicode_segmentation::UnicodeSegmentation;
    let mut out = Vec::new();
    // Running byte→char cursor so the whole pass is O(n), not O(n) per word.
    let mut last_byte = 0usize;
    let mut char_pos = 0usize;
    for (byte_off, word) in text.unicode_word_indices() {
        char_pos += text[last_byte..byte_off].chars().count();
        let len = word.chars().count();
        out.push((char_pos, len, word));
        char_pos += len;
        last_byte = byte_off + word.len();
    }
    out
}

/// A misspelling predicate over one document's **active** dictionaries — the pure engine a
/// per-document [`SpellSession`] queries per word. Immutable: a snapshot of the active
/// dictionaries (primary first) + the Work's personal words. Built by
/// [`SpellcheckService::build_checker`]. `Clone` is cheap — `Arc` dictionaries — so one build can
/// feed both the main and synopsis sessions.
#[derive(Clone)]
pub struct SpellChecker {
    /// Active dictionaries, primary first; empty is never built (a checker with no dictionary is
    /// never produced — the caller clears the session instead).
    dicts: Vec<Arc<spellbook::Dictionary>>,
    /// The Work's personal words — the **last** checker, consulted only after every installed
    /// dictionary has rejected the word: the project's own fallback for words no dictionary knows.
    /// Removing one is just removing the entity.
    personal: HashSet<String>,
}

impl SpellChecker {
    fn misspelled(&self, word: &str) -> bool {
        // Numbers, punctuation runs, and the like are not spell-checkable.
        if !word.chars().any(|c| c.is_alphabetic()) {
            return false;
        }
        // The true dictionaries first — correctly-spelled prose is accepted by the primary and
        // never reaches the personal set (`any` short-circuits). Only a word that *no* installed
        // dictionary knows falls through to the project's own word list, the final checker.
        if self.dicts.iter().any(|d| d.check(word)) {
            return false;
        }
        !self.personal.contains(word)
    }
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

/// The app-wide spell-check engine: a per-language dictionary cache, the session mute set, and
/// the current Work's personal words. Cloneable (shares one `Rc` state).
#[derive(Clone)]
pub struct SpellcheckService {
    inner: Rc<Inner>,
}

struct Inner {
    /// id → loaded dictionary, or `None` for "tried and absent/unusable" (so a miss isn't
    /// re-attempted every keystroke).
    cache: RefCell<HashMap<String, Option<Arc<spellbook::Dictionary>>>>,
    /// Session-muted language keys (resolved registry ids). Cleared on `close_work`.
    muted: RefCell<HashSet<String>>,
    /// Bumped on every mute change so a language-pill field can rebuild its check marks.
    mute_version: Signal<u64>,
    /// The open Work's personal words (`DictWord`).
    personal: RefCell<HashSet<String>>,
}

impl SpellcheckService {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Inner {
                cache: RefCell::new(HashMap::new()),
                muted: RefCell::new(HashSet::new()),
                mute_version: Signal::new(0),
                personal: RefCell::new(HashSet::new()),
            }),
        }
    }

    /// A version counter bumped whenever the mute set changes — bind a pill field to it to
    /// rebuild its green checks live (from either the Inspector or the Settings pane).
    pub fn mute_version(&self) -> Signal<u64> {
        self.inner.mute_version.clone()
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

    /// Whether spell-checking for `tag`'s language is muted this session.
    pub fn is_muted(&self, tag: &str) -> bool {
        self.inner.muted.borrow().contains(&Self::key_of(tag))
    }

    /// Toggle the session mute for `tag`'s language. Returns whether the set changed (so the
    /// caller only re-attaches on a real change).
    pub fn set_muted(&self, tag: &str, muted: bool) -> bool {
        let key = Self::key_of(tag);
        let changed = {
            let mut set = self.inner.muted.borrow_mut();
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

    /// Replace the open Work's personal words (from its `DictWord` set).
    pub fn set_personal(&self, words: HashSet<String>) {
        *self.inner.personal.borrow_mut() = words;
    }

    /// Drop the loaded-dictionary cache only (keeping session mutes + personal words), so the
    /// next attach re-reads disk. Called when a dictionary is installed or removed: without this,
    /// `dict()`'s per-id cache would keep serving a stale entry — a cached miss would hide a fresh
    /// install, and a cached `Arc` would keep a just-removed dictionary alive (so squiggles would
    /// neither appear nor degrade until the project is reopened).
    pub fn invalidate_dictionaries(&self) {
        self.inner.cache.borrow_mut().clear();
    }

    /// Drop everything project-scoped — the dictionary cache, mutes, and personal words. Called
    /// on `close_work`; a fresh project reloads lazily and starts unmuted.
    pub fn clear(&self) {
        self.inner.cache.borrow_mut().clear();
        let had_mutes = !self.inner.muted.borrow().is_empty();
        self.inner.muted.borrow_mut().clear();
        self.inner.personal.borrow_mut().clear();
        if had_mutes {
            let v = self.inner.mute_version.get();
            self.inner.mute_version.set(v.wrapping_add(1));
        }
    }

    /// The active (non-muted, installed) dictionaries for a tag list, primary first, deduped.
    fn active_dicts(&self, tags: &str) -> Vec<Arc<spellbook::Dictionary>> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for tag in language::all(tags) {
            let key = Self::key_of(tag);
            if self.inner.muted.borrow().contains(&key) {
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

    /// Build a [`SpellChecker`] for a document's tag list, or `None` when nothing is
    /// active/installed (the caller then clears its session — the degrade path).
    pub fn build_checker(&self, tags: &str) -> Option<SpellChecker> {
        let dicts = self.active_dicts(tags);
        if dicts.is_empty() {
            return None;
        }
        Some(SpellChecker {
            dicts,
            personal: self.inner.personal.borrow().clone(),
        })
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
/// It mirrors `bastyde::widgets::rich_text::FindSession`'s lifecycle — a held `Subscription`, a
/// `dirty` flag drained by a per-frame `tick`, and a `Drop` that retires the layer. The caret is
/// read **live** at recompute time (via the closure a focused view supplies): the editor batches a
/// printable keystroke a frame behind the caret signal, so a value pushed earlier would be stale
/// exactly when a just-typed space should reveal the word.
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
    /// Held so the subscription lives as long as the session (dropping it unsubscribes).
    _sub: Subscription,
    checker: RefCell<Option<SpellChecker>>,
    color: Cell<Color>,
    /// The one focused view of this document, if any: `(widget id, a LIVE caret reader)`. Only its
    /// caret exempts a word; `None` = no view focused = nothing exempt. The reader is a closure so
    /// a headless test can inject a caret without a mounted editor (production reads
    /// `EditorHandle::cursor_position`).
    focused: RefCell<Option<(WidgetId, Rc<dyn Fn() -> usize>)>>,
    /// Every misspelling in the document, exemption **not** applied — the cache a caret move
    /// re-filters instead of re-scanning.
    all_ranges: RefCell<Vec<RangeHighlight>>,
    /// The last set pushed, so an unchanged recompute skips the repaint (`RangeHighlight: Eq`).
    last_ranges: RefCell<Vec<RangeHighlight>>,
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
            _sub: sub,
            checker: RefCell::new(None),
            color: Cell::new(Color::rgb(220, 50, 50)),
            focused: RefCell::new(None),
            all_ranges: RefCell::new(Vec::new()),
            last_ranges: RefCell::new(Vec::new()),
        })
    }

    /// Set the active checker + squiggle colour and recompute now. Called from app-level effects
    /// (dictionary install/remove, mute, language change) — a safe context, never inside a doc
    /// event, so recomputing directly cannot re-enter the `on_change` dispatch. `None` clears the
    /// squiggles (the degrade path) while leaving the cheap empty session attached.
    pub fn set_checker(&self, checker: Option<SpellChecker>, color: Color) {
        *self.checker.borrow_mut() = checker;
        self.color.set(color);
        self.rebuild_all_ranges();
        self.apply_exemption();
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
        let content = self.content_dirty.swap(false, Ordering::Relaxed);
        let caret = self.caret_dirty.replace(false);
        if !content && !caret {
            return;
        }
        if content {
            self.rebuild_all_ranges();
        }
        self.apply_exemption();
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

    /// Filter the cached misspellings by the live caret — drop the one word it sits in (inclusive
    /// of both ends, so typing at a word's end keeps it exempt) — and push, but only if the result
    /// changed since the last push (so typing *within* the exempt word repaints nothing).
    fn apply_exemption(&self) {
        let caret = self.focused.borrow().as_ref().map(|(_, f)| f());
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
mod tests {
    use super::*;

    /// Char offsets are correct through accented text (a byte offset would be wrong here).
    #[test]
    fn word_positions_uses_char_offsets() {
        // "éàî mot" — the first word is 3 chars (6 bytes); "mot" starts at char 4.
        let got = word_positions("éàî mot");
        assert_eq!(got[0], (0, 3, "éàî"));
        assert_eq!(got[1], (4, 3, "mot"));
    }

    /// Contractions and elisions stay one token, for both apostrophes.
    #[test]
    fn word_positions_keeps_apostrophes() {
        let straight: Vec<&str> = word_positions("don't").iter().map(|(_, _, w)| *w).collect();
        assert_eq!(straight, ["don't"]);
        let curly: Vec<&str> = word_positions("l\u{2019}auteur").iter().map(|(_, _, w)| *w).collect();
        assert_eq!(curly, ["l\u{2019}auteur"]);
    }

    /// A tiny real dictionary flags the misspelling and leaves the good word and the number.
    #[test]
    fn highlighter_flags_only_the_misspelling() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n")
            .expect("tiny dictionary parses");
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal: HashSet::new(),
        };
        assert!(hl.misspelled("helo"), "a misspelling is flagged");
        assert!(!hl.misspelled("hello"), "a good word is not");
        assert!(!hl.misspelled("world"), "another good word is not");
        assert!(!hl.misspelled("123"), "a number is never a misspelling");
    }

    /// A personal word overrides the dictionary — checked first, so no dictionary mutation.
    #[test]
    fn personal_words_are_the_final_fallback() {
        // The project's own word list is the last checker: a word no installed dictionary knows is
        // rescued by the personal set; an unknown word absent from both stays flagged.
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        let mut personal = HashSet::new();
        personal.insert("Skribisto".to_string());
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal,
        };
        assert!(!hl.misspelled("Skribisto"), "a personal word is accepted");
        assert!(hl.misspelled("Skrib"), "but not an unrelated unknown word");
    }

    /// The multi-dictionary union: a word only one language knows is still accepted.
    #[test]
    fn a_word_any_active_dictionary_knows_is_accepted() {
        let en = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        let fr = spellbook::Dictionary::new("SET UTF-8\n", "1\nbonjour\n").unwrap();
        let hl = SpellChecker {
            dicts: vec![Arc::new(en), Arc::new(fr)],
            personal: HashSet::new(),
        };
        assert!(!hl.misspelled("hello"), "English word accepted");
        assert!(!hl.misspelled("bonjour"), "French word accepted");
        assert!(hl.misspelled("guten"), "a word neither knows is flagged");
    }

    /// `validate_dictionary_files` accepts a real pair and rejects garbage / missing files.
    #[test]
    fn validate_accepts_good_rejects_bad() {
        let dir = std::env::temp_dir().join(format!("skrib-valdict-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let aff = dir.join("ok.aff");
        let dic = dir.join("ok.dic");
        std::fs::write(&aff, "SET UTF-8\n").unwrap();
        std::fs::write(&dic, "1\nhello\n").unwrap();
        assert!(validate_dictionary_files(&aff, &dic).is_ok(), "a real pair validates");

        // A missing file is a read error, not a panic.
        assert!(validate_dictionary_files(&dir.join("nope.aff"), &dic).is_err());

        // A .dic whose count line is nonsense fails to parse (spellbook rejects it).
        let bad = dir.join("bad.dic");
        std::fs::write(&bad, "not-a-count\n\0\0garbage").unwrap();
        assert!(validate_dictionary_files(&aff, &bad).is_err(), "garbage is rejected");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The ISO-8859-1 transcode path: a `SET ISO8859-1` `.aff` decodes its bytes correctly.
    #[test]
    fn detect_and_decode_latin1() {
        let enc = detect_encoding(b"SET ISO8859-1\nTRY esiat\n");
        assert_eq!(enc.name(), "windows-1252"); // encoding_rs maps ISO-8859-1 to its superset
        // 0xE9 is 'é' in Latin-1.
        let (decoded, _, _) = enc.decode(b"caf\xe9");
        assert_eq!(decoded, "café");
    }

    // ── SpellSession (the caret-aware range highlighter) ──

    fn tiny_doc(text: &str) -> TextDocument {
        let d = TextDocument::new();
        d.set_plain_text(text).unwrap();
        d
    }

    /// A checker knowing `hello`/`world` — so `helo`/`wrld` are misspelled.
    fn en_checker() -> SpellChecker {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
        SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal: HashSet::new(),
        }
    }

    /// Focus a single view whose caret is read from `cell` (the closure production supplies is
    /// `move || handle.cursor_position()`; a test injects a plain cell instead).
    fn focus_at(session: &SpellSession, cell: &Rc<Cell<usize>>) {
        let c = cell.clone();
        session.on_focus(WidgetId::default(), Rc::new(move || c.get()));
    }

    fn starts(session: &SpellSession) -> Vec<usize> {
        session.last_ranges.borrow().iter().map(|r| r.start).collect()
    }

    #[test]
    fn session_exempts_the_caret_word_and_flags_the_rest() {
        let doc = tiny_doc("helo wrld"); // both misspelled
        let session = SpellSession::new(&doc);
        let caret = Rc::new(Cell::new(2usize)); // inside "helo" [0,4]
        focus_at(&session, &caret);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert_eq!(starts(&session), vec![5], "caret word exempt; only wrld (char 5) flagged");
    }

    #[test]
    fn no_focused_view_flags_every_misspelling() {
        let doc = tiny_doc("helo wrld");
        let session = SpellSession::new(&doc);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert_eq!(starts(&session), vec![0, 5], "no exemption without a focused caret");
    }

    #[test]
    fn caret_at_the_word_end_keeps_it_exempt() {
        let doc = tiny_doc("helo wrld");
        let session = SpellSession::new(&doc);
        let caret = Rc::new(Cell::new(4usize)); // the END of "helo" [0,4] — still typing it
        focus_at(&session, &caret);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert_eq!(starts(&session), vec![5], "inclusive end keeps the just-typed word exempt");
    }

    #[test]
    fn caret_exempts_at_most_one_word_at_a_zero_gap_boundary() {
        // Two TOUCHING misspelled ranges — [0,1) and [1,2) — as adjacent CJK/Hiragana characters
        // produce (UAX#29 splits them, and no CJK dictionary means both are "misspelled"). A caret
        // exactly on the shared boundary (char 1) is inclusive-in both; only the first must drop.
        let doc = tiny_doc("ab");
        let session = SpellSession::new(&doc);
        let fmt = || spell_format(Color::rgb(220, 50, 50));
        *session.all_ranges.borrow_mut() = vec![
            RangeHighlight { start: 0, length: 1, format: fmt() },
            RangeHighlight { start: 1, length: 1, format: fmt() },
        ];
        let caret = Rc::new(Cell::new(1usize));
        focus_at(&session, &caret);
        session.apply_exemption();
        assert_eq!(starts(&session), vec![1], "only the first touching word is exempt, not both");
    }

    #[test]
    fn moving_the_caret_reveals_the_word_left_and_hides_the_word_entered() {
        let doc = tiny_doc("helo wrld");
        let session = SpellSession::new(&doc);
        let caret = Rc::new(Cell::new(2usize)); // in "helo"
        focus_at(&session, &caret);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert_eq!(starts(&session), vec![5], "helo exempt, wrld flagged");

        caret.set(6); // move into "wrld" [5,9]
        session.on_caret(WidgetId::default());
        session.tick();
        assert_eq!(starts(&session), vec![0], "now helo is flagged and wrld exempt");
    }

    #[test]
    fn a_content_edit_re_derives_on_the_next_tick() {
        let doc = tiny_doc("hello"); // correct → nothing flagged
        let session = SpellSession::new(&doc);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert!(session.last_ranges.borrow().is_empty(), "correct prose has no squiggle");

        doc.set_plain_text("helo").unwrap(); // now misspelled — fires an offset-moving event
        session.tick();
        assert_eq!(starts(&session), vec![0], "the edit is picked up on the tick");
    }

    #[test]
    fn char_offsets_are_document_absolute_through_accents() {
        // "café" is correct; "wrld" is the misspelling. A byte offset would place it at 6 (é is two
        // bytes); the char offset is 5.
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\ncafé\nworld\n").unwrap();
        let checker = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal: HashSet::new(),
        };
        let doc = tiny_doc("café wrld");
        let session = SpellSession::new(&doc);
        session.set_checker(Some(checker), Color::rgb(220, 50, 50));
        assert_eq!(starts(&session), vec![5], "char offset, not byte offset");
    }

    #[test]
    fn a_second_paragraph_gets_absolute_offsets() {
        let doc = tiny_doc("hello\nwrld"); // block 2 ("wrld") starts one past block 1 ("hello")
        let session = SpellSession::new(&doc);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert_eq!(starts(&session), vec![6], "wrld sits at char 6 (5 + the 1-char block gap)");
    }

    #[test]
    fn no_checker_clears_the_squiggles() {
        let doc = tiny_doc("helo wrld");
        let session = SpellSession::new(&doc);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert!(!session.last_ranges.borrow().is_empty());
        session.set_checker(None, Color::rgb(220, 50, 50)); // degrade
        assert!(session.last_ranges.borrow().is_empty(), "no checker → no ranges, session kept");
    }

    #[test]
    fn an_idle_tick_is_a_no_op() {
        let doc = tiny_doc("helo wrld");
        let session = SpellSession::new(&doc);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        let before = session.last_ranges.borrow().clone();
        session.tick(); // nothing dirty
        assert_eq!(*session.last_ranges.borrow(), before, "an idle tick changes nothing");
    }
}
