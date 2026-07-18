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

pub(crate) mod toggle_button;
pub(crate) mod dictionary_registry;
pub(crate) mod add_dictionary_panel;
pub(crate) mod language_pill_field;

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

/// How many corrections the context menu offers at most. The suggestions sit flat at the top of
/// the menu, so this is a menu-length budget as much as a relevance one — past a handful, a list
/// of guesses is harder to scan than retyping the word.
const MAX_SUGGESTIONS: usize = 6;

/// The largest edit distance at which a *personal* word is offered as a correction. Two edits is
/// the usual typo radius (Hunspell's own replacement table works in the same neighbourhood);
/// wider than that and a short project term starts "correcting" to every other project term.
const MAX_PERSONAL_DISTANCE: usize = 2;

/// How many of [`MAX_SUGGESTIONS`] are held back for *distant* personal matches when any exist.
///
/// Without a reservation the project's own term is unreachable exactly when it matters. A
/// two-edit typo of a coined word (`Skiibsto` for `Skribisto`) is a word no installed dictionary
/// knows, which is precisely when Hunspell's ngram search is at its most talkative — it happily
/// returns six unrelated English guesses. Appending the personal matches after those and then
/// truncating would drop the only correction the writer wanted.
const PERSONAL_SUGGESTION_FLOOR: usize = 2;

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

    /// Ranked corrections for a misspelled `word`, drawn from **both** the installed dictionaries
    /// and the Work's own personal words. At most [`MAX_SUGGESTIONS`], deduped exact-case.
    ///
    /// ## Why the personal set is searched separately
    ///
    /// [`spellbook::Dictionary::suggest`] is closed over the *compiled* dictionary: it cannot see
    /// [`personal`](Self::personal) at all, so a typo of a project's own coined term would never
    /// be corrected to it — the one case a writer most needs. Those near-matches are therefore
    /// found here, by bounded edit distance over the personal set.
    ///
    /// ## The ordering
    ///
    /// A personal word within **one** edit goes first: for an invented word the installed
    /// dictionary has nothing real to offer, and its ngram guesses are noise next to the term the
    /// writer actually meant. The dictionary's own ranked suggestions follow (they are the right
    /// answer for a typo of an ordinary word), and the looser personal matches come last — but
    /// with [`PERSONAL_SUGGESTION_FLOOR`] slots reserved for them, so a talkative dictionary can
    /// never crowd the project's own term off the end of the list.
    pub(crate) fn suggest(&self, word: &str) -> Vec<String> {
        if !word.chars().any(|c| c.is_alphabetic()) {
            return Vec::new();
        }
        // Each dictionary's suggestions, concatenated **lazily**: `map` + `flatten` pull one
        // dictionary at a time, so a second language's ngram search never runs once
        // `merge_suggestions` has stopped taking.
        let dict = self.dicts.iter().flat_map(|d| {
            let mut buf = Vec::new();
            d.suggest(word, &mut buf); // clears `buf` itself before filling it
            buf
        });
        merge_suggestions(word, self.personal_suggestions(word), dict)
    }

    /// Personal words within [`MAX_PERSONAL_DISTANCE`] edits of `word`, as `(distance, word)`,
    /// nearest first.
    ///
    /// Distance is measured on the **lower-cased** forms, so a personal word differing only in
    /// casing comes back at distance 0. That is not a curiosity but the common case: the personal
    /// set is matched exact-case, so typing `skribisto` when the project stores `Skribisto` *is* a
    /// misspelling — and the correction to offer is the stored casing.
    ///
    /// Ties break alphabetically: `personal` is a `HashSet`, whose iteration order varies run to
    /// run, and a context menu whose items reshuffle between right-clicks is unusable.
    ///
    /// The typed word itself is *not* filtered here — [`push_unique`] is the single gate that
    /// drops it, so every source is held to the same rule.
    fn personal_suggestions(&self, word: &str) -> Vec<(usize, String)> {
        // Collected once: the needle is invariant across the scan, and this runs over every
        // personal word (an imported list may hold thousands).
        let needle: Vec<char> = word.to_lowercase().chars().collect();
        let mut scored: Vec<(usize, String)> = self
            .personal
            .iter()
            .filter_map(|w| {
                let lower = w.to_lowercase();
                // Rule the candidate out on length before building its char vector: a gap wider
                // than the cap cannot be closed by any number of edits, and counting allocates
                // nothing. Counted on the *lower-cased* form, since lowercasing can change a
                // word's length (`İ` becomes two chars) and the distance is measured there.
                if lower.chars().count().abs_diff(needle.len()) > MAX_PERSONAL_DISTANCE {
                    return None;
                }
                let candidate: Vec<char> = lower.chars().collect();
                bounded_levenshtein(&needle, &candidate, MAX_PERSONAL_DISTANCE)
                    .map(|d| (d, w.clone()))
            })
            .collect();
        scored.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        scored
    }
}

#[cfg(test)]
impl SpellChecker {
    /// A checker over one tiny in-memory dictionary plus a personal set.
    ///
    /// The seam the menu-resolution tests need: the production path
    /// ([`SpellcheckService::build_checker`]) reads installed `.aff`/`.dic` pairs off disk, which
    /// a unit test has no business depending on.
    pub(crate) fn for_tests(dic_words: &[&str], personal: &[&str]) -> Self {
        let dic = format!("{}\n{}\n", dic_words.len(), dic_words.join("\n"));
        let dict =
            spellbook::Dictionary::new("SET UTF-8\n", &dic).expect("tiny dictionary parses");
        Self {
            dicts: vec![Arc::new(dict)],
            personal: personal.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Rank and budget the suggestion sources into the final list.
///
/// Split out from [`SpellChecker::suggest`] as a pure function over its three inputs because the
/// budget cannot be tested through a real dictionary: a synthetic test dictionary has no `TRY`
/// table and so cannot be provoked into the ngram chattiness this exists to defend against, while
/// a real one would drag installed `.aff`/`.dic` files into a unit test.
///
/// `personal` is `(distance, word)` nearest-first; `dict` is pulled **lazily** and only as far as
/// the budget allows, so an unconsumed dictionary's suggester never runs.
fn merge_suggestions(
    typed: &str,
    personal: Vec<(usize, String)>,
    mut dict: impl Iterator<Item = String>,
) -> Vec<String> {
    // `near` (<= 1 edit) leads; `far` (2 edits) trails but is guaranteed room.
    let (near, far): (Vec<_>, Vec<_>) = personal.into_iter().partition(|(d, _)| *d <= 1);
    let mut out: Vec<String> = Vec::new();

    // Everything before the reserved slots shares one ceiling — `near` included. `near` is not
    // exempt just because it ranks first: `personal_suggestions` caps distance, not *count*, and
    // a glossary of similar short terms (a Kai / Kal / Kar naming family) can yield more one-edit
    // matches than the whole menu holds, which would push `far` past the end.
    let ceiling = MAX_SUGGESTIONS.saturating_sub(far.len().min(PERSONAL_SUGGESTION_FLOOR));
    for (_, w) in &near {
        if out.len() >= ceiling {
            break;
        }
        push_unique(&mut out, typed, w.clone());
    }
    // Pull only while there is room to keep what comes back: `for s in dict` would fetch one more
    // and discard it, and since the sources are concatenated lazily that wasted pull can cross
    // into the next dictionary and run a whole ngram search for an item this drops on the floor.
    while out.len() < ceiling {
        match dict.next() {
            Some(s) => push_unique(&mut out, typed, s),
            None => break,
        }
    }
    for (_, w) in &far {
        if out.len() >= MAX_SUGGESTIONS {
            break;
        }
        push_unique(&mut out, typed, w.clone());
    }
    // A `far` word that merely repeated something already listed leaves its reserved slot empty —
    // give it back to the dictionary rather than hand back a short menu while suggestions remain.
    while out.len() < MAX_SUGGESTIONS {
        match dict.next() {
            Some(s) => push_unique(&mut out, typed, s),
            None => break,
        }
    }
    // Every push above is gated on a ceiling, so no trailing truncate is needed — and none should
    // be added back: a truncate here is what silently ate the reserved slots before.
    out
}

/// Push `s` unless it is the word the writer typed, or an equal suggestion is already there.
///
/// The single gate every source passes through. Deduping keeps the first (better-ranked)
/// occurrence when two dictionaries offer the same correction; rejecting `typed` means no source
/// can echo the input back as its own correction — a menu item that would edit nothing. Only the
/// personal set used to be held to that rule, which left the dictionaries free to suggest the
/// input via a case or split-word variant.
fn push_unique(out: &mut Vec<String>, typed: &str, s: String) {
    if s != typed && !out.iter().any(|e| e == &s) {
        out.push(s);
    }
}

/// Levenshtein distance between `a` and `b`, or `None` once it is known to exceed `max`.
///
/// The cap is what keeps this cheap enough to run over the whole personal set on a right-click: a
/// survivor bails as soon as every path through a row is already too far, so a candidate costs a
/// few rows rather than a full matrix.
///
/// Takes **already-lower-cased `char` slices** rather than `&str`. Both are the caller's to
/// prepare: the needle is invariant across a scan and would otherwise be re-collected for every
/// candidate, and the cheap length gate belongs *before* a candidate's vector is built, not after
/// — measuring in `char`s, so accented terms count in letters rather than UTF-8 bytes.
fn bounded_levenshtein(a: &[char], b: &[char], max: usize) -> Option<usize> {
    if a.len().abs_diff(b.len()) > max {
        return None;
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for i in 1..=a.len() {
        cur[0] = i;
        let mut row_min = cur[0];
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
            row_min = row_min.min(cur[j]);
        }
        if row_min > max {
            return None;
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    Some(prev[b.len()]).filter(|d| *d <= max)
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
    /// Bumped on every mute change **and every master-switch flip** so a language-pill field
    /// rebuilds its check marks — the pills bind this and nothing else, so a switch that did
    /// not bump it would leave them looking live while nothing was being checked.
    mute_version: Signal<u64>,
    /// The master switch (`SPELLCHECK_ENABLED_KEY`, default on). Mirrored here by `App` from
    /// the settings store so [`build_checker`](SpellcheckService::build_checker) — the single
    /// gate every document passes through — can answer "off" before touching a dictionary.
    enabled: Cell<bool>,
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
                enabled: Cell::new(true),
                personal: RefCell::new(HashSet::new()),
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
        // The master switch, checked first: every document's checker is built here, so one
        // early return turns spell-check off everywhere — squiggles, `is_misspelled`, and the
        // context menu's corrections alike — without loading a dictionary or walking a tag.
        // Off reuses the existing degrade path (`None` → the session clears its ranges and
        // keeps its cheap empty layer attached), so nothing new has to be torn down.
        if !self.inner.enabled.get() {
            return None;
        }
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
    focused: RefCell<Option<(WidgetId, Rc<dyn Fn() -> usize>)>>,
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
    /// identity skips the O(all_ranges) filter + clone — on a Lorem-Ipsum-dense document, the
    /// dominant per-keystroke cost, previously paid every tick whether or not anything changed.
    fn apply_exemption(&self, forced: bool) {
        let caret = self.focused.borrow().as_ref().map(|(_, f)| f());
        let exempt = self.caret_exempt_identity(caret);
        if !forced && self.last_exempt.get() == exempt {
            return; // same word exempt, `all_ranges` unchanged → nothing to rebuild or push
        }
        self.last_exempt.set(exempt);
        #[cfg(test)]
        self.exemption_recomputes.set(self.exemption_recomputes.get() + 1);
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

    // ── the master switch (Settings ▸ Spelling / the title-bar toggle / F7) ──

    /// A service whose `en-US` dictionary is already in the cache, so these tests never touch
    /// the disk and never depend on what this machine happens to have installed.
    fn service_with_tiny_dict() -> SpellcheckService {
        let svc = SpellcheckService::new();
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
        svc.inner
            .cache
            .borrow_mut()
            .insert("en-US".to_string(), Some(Arc::new(dict)));
        svc
    }

    /// Spell-check is on out of the box — the switch is an escape hatch, not an opt-in.
    #[test]
    fn spellcheck_is_enabled_by_default() {
        assert!(SpellcheckService::new().is_enabled());
    }

    #[test]
    fn set_enabled_reports_change_only_on_a_real_flip() {
        let svc = SpellcheckService::new();
        assert!(!svc.set_enabled(true), "already on — not a change");
        assert!(svc.set_enabled(false), "on -> off is a change");
        assert!(!svc.set_enabled(false), "already off — not a change");
        assert!(svc.set_enabled(true), "off -> on is a change");
    }

    /// The pills bind `mute_version` and nothing else, so a flip must bump it or they keep
    /// showing live green checks while the switch is off.
    #[test]
    fn set_enabled_bumps_mute_version_on_a_real_flip() {
        let svc = SpellcheckService::new();
        let v = svc.mute_version();
        let before = v.get();
        svc.set_enabled(false);
        assert_eq!(v.get(), before + 1, "a real flip rebuilds the pill field");
        svc.set_enabled(false);
        assert_eq!(v.get(), before + 1, "a no-op flip must not churn the UI");
    }

    /// The one gate: off means every document's checker is `None`, which is the same degrade
    /// path a missing dictionary already takes.
    #[test]
    fn build_checker_short_circuits_when_disabled_and_resumes_when_re_enabled() {
        let svc = service_with_tiny_dict();
        assert!(svc.build_checker("en-US").is_some(), "on by default");

        svc.set_enabled(false);
        assert!(svc.build_checker("en-US").is_none(), "off — no checker at all");

        svc.set_enabled(true);
        let checker = svc.build_checker("en-US").expect("back on");
        assert!(checker.misspelled("helo"), "and it checks again");
    }

    /// Off must beat everything downstream — a document that would otherwise be checked (an
    /// installed, unmuted language) still gets nothing.
    #[test]
    fn the_master_switch_overrides_an_otherwise_checkable_document() {
        let svc = service_with_tiny_dict();
        assert!(!svc.is_muted("en-US"), "precondition: nothing muted");
        svc.set_enabled(false);
        assert!(
            svc.build_checker("en-US").is_none(),
            "an installed, unmuted language is still not checked when the switch is off"
        );
    }

    /// `clear()` is `close_work`: it drops **project** state (dictionary cache, session mutes,
    /// personal words). The master switch is an app-wide preference and must survive — a
    /// writer who turned spell-check off does not expect the next project to turn it back on.
    #[test]
    fn close_work_does_not_reset_the_master_switch() {
        let svc = SpellcheckService::new();
        svc.set_enabled(false);
        svc.clear();
        assert!(!svc.is_enabled(), "the switch is app-wide, not project state");
    }

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

    // ── Suggestions (the context-menu corrections) ──

    /// The installed dictionary corrects a typo of an ordinary word.
    #[test]
    fn suggest_offers_dictionary_corrections() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal: HashSet::new(),
        };
        let got = hl.suggest("helo");
        assert!(got.contains(&"hello".to_string()), "expected 'hello' in {got:?}");
        assert!(got.len() <= MAX_SUGGESTIONS, "capped at {MAX_SUGGESTIONS}: {got:?}");
        // Nothing alphabetic is not correctable.
        assert!(hl.suggest("123").is_empty(), "a number has no corrections");
    }

    /// **The personal-dictionary suggestion.** `spellbook` cannot see the personal set, so a typo
    /// of a project's coined term is only ever corrected by our own near-match pass — and it must
    /// outrank the dictionary's guesses for an invented word.
    #[test]
    fn suggest_offers_personal_words_the_dictionary_cannot_know() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
        let mut personal = HashSet::new();
        personal.insert("Skribisto".to_string());
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal,
        };
        // Sanity: the dictionary alone knows nothing of it.
        let mut raw = Vec::new();
        hl.dicts[0].suggest("Skibisto", &mut raw);
        assert!(
            !raw.contains(&"Skribisto".to_string()),
            "precondition: spellbook cannot suggest a personal word ({raw:?})"
        );
        // But we can — and it comes first, being one edit away.
        let got = hl.suggest("Skibisto");
        assert_eq!(
            got.first().map(String::as_str),
            Some("Skribisto"),
            "a one-edit personal word leads the list, got {got:?}"
        );
    }

    /// A personal word differing only in **casing** is a distance-0 match. This is the everyday
    /// case: the personal set is matched exact-case, so `skribisto` is genuinely flagged, and the
    /// correction to offer is the project's own casing.
    #[test]
    fn suggest_corrects_the_casing_of_a_personal_word() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        let mut personal = HashSet::new();
        personal.insert("Skribisto".to_string());
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal,
        };
        assert!(hl.misspelled("skribisto"), "precondition: exact-case matching flags it");
        assert_eq!(
            hl.suggest("skribisto").first().map(String::as_str),
            Some("Skribisto"),
            "the stored casing is offered"
        );
    }

    /// A personal word is never suggested for itself, and a distant one is not suggested at all.
    #[test]
    fn suggest_skips_the_typed_word_and_distant_personal_words() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        let mut personal = HashSet::new();
        personal.insert("Skribisto".to_string());
        personal.insert("Bastyde".to_string());
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal,
        };
        // "Skribisto" itself is not flagged, but even asked directly it must not echo back.
        assert!(
            !hl.suggest("Skribisto").contains(&"Skribisto".to_string()),
            "a word is never its own correction"
        );
        // "Bastyde" is far from "Skibisto" — beyond the typo radius, so it is not offered.
        assert!(
            !hl.suggest("Skibisto").contains(&"Bastyde".to_string()),
            "an unrelated personal word is not a correction"
        );
    }

    /// A dictionary chatty enough to fill every slot on its own — what Hunspell's ngram search
    /// does for a coined word nothing knows. Synthesised, because a test-sized dictionary has no
    /// `TRY` table and cannot be provoked into guessing that freely.
    fn chatty_dict() -> Vec<String> {
        ["aaa", "bbb", "ccc", "ddd", "eee", "fff", "ggg", "hhh"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// **The reserved slots.** Without [`PERSONAL_SUGGESTION_FLOOR`] a two-edit personal match is
    /// appended past the cap and truncated away — so for `Skiibsto` the writer would get six
    /// unrelated English guesses and never the project's own term.
    #[test]
    fn a_distant_personal_word_survives_a_talkative_dictionary() {
        let personal = vec![(2usize, "Helios".to_string())];
        let got = merge_suggestions("helo", personal, chatty_dict().into_iter());
        assert!(
            got.contains(&"Helios".to_string()),
            "the 2-edit personal word keeps its reserved slot, got {got:?}"
        );
        assert_eq!(got.len(), MAX_SUGGESTIONS, "and the list is still full: {got:?}");
        assert_eq!(
            got.last().map(String::as_str),
            Some("Helios"),
            "it trails the dictionary's own ranked guesses"
        );
    }

    /// No personal matches ⇒ nothing is reserved and the dictionary gets every slot.
    #[test]
    fn without_personal_matches_the_dictionary_fills_the_list() {
        let got = merge_suggestions("helo", vec![], chatty_dict().into_iter());
        assert_eq!(
            got.len(),
            MAX_SUGGESTIONS,
            "the floor must not shrink the list when there is nothing to reserve for: {got:?}"
        );
    }

    /// A one-edit personal match leads *and* still leaves the dictionary its slots.
    #[test]
    fn a_near_personal_word_leads_without_reserving() {
        let personal = vec![(1usize, "Helo2".to_string())];
        let got = merge_suggestions("helo", personal, chatty_dict().into_iter());
        assert_eq!(got.first().map(String::as_str), Some("Helo2"), "{got:?}");
        assert_eq!(got.len(), MAX_SUGGESTIONS);
    }

    /// **`near` is not exempt from the ceiling.** A glossary of similar short terms (a
    /// Kai/Kal/Kar naming family) yields more one-edit matches than the menu holds; those used to
    /// be pushed before the ceiling existed, so the *reserved* far slots were appended past the
    /// cap and truncated away — the reservation silently evicted by the very list it leads.
    #[test]
    fn a_crowd_of_near_personal_words_cannot_evict_the_reserved_far_slots() {
        let personal = vec![
            (1usize, "Kai".to_string()),
            (1, "Kal".to_string()),
            (1, "Kar".to_string()),
            (1, "Kaz".to_string()),
            (1, "Kay".to_string()),
            (1, "Kah".to_string()),
            (2, "Kaito".to_string()),
            (2, "Kalim".to_string()),
        ];
        let got = merge_suggestions("Kax", personal, chatty_dict().into_iter());
        assert_eq!(got.len(), MAX_SUGGESTIONS, "{got:?}");
        assert!(
            got.contains(&"Kaito".to_string()) && got.contains(&"Kalim".to_string()),
            "both reserved far slots survive a crowd of near matches, got {got:?}"
        );
        assert_eq!(
            got.iter().filter(|w| w.len() == 3).count(),
            4,
            "near is capped at the ceiling (6 - 2 reserved), got {got:?}"
        );
    }

    /// **A wasted reserved slot is given back.** When a far word merely repeats something already
    /// listed, `push_unique` drops it — the slot must go back to the dictionary rather than hand
    /// back a short menu while suggestions remain unfetched.
    #[test]
    fn a_far_word_colliding_with_a_dictionary_suggestion_backfills() {
        // "aaa" is both the project's own term and the dictionary's first guess.
        let personal = vec![(2usize, "aaa".to_string())];
        let got = merge_suggestions("typed", personal, chatty_dict().into_iter());
        assert_eq!(
            got.len(),
            MAX_SUGGESTIONS,
            "the collided slot is refilled from the dictionary, got {got:?}"
        );
        assert_eq!(got.iter().filter(|w| *w == "aaa").count(), 1, "and not duplicated");
    }

    /// The dictionary is never pulled past the budget — the early-exit that keeps a second
    /// language's ngram search from running once the list is full.
    #[test]
    fn the_dictionary_is_not_pulled_past_the_budget() {
        let pulled = Cell::new(0usize);
        let dict = (0..100).map(|i| {
            pulled.set(pulled.get() + 1);
            format!("w{i}")
        });
        let got = merge_suggestions("helo", vec![(2, "Helios".into())], dict);
        assert_eq!(got.len(), MAX_SUGGESTIONS);
        // 5 dictionary slots (6 minus the one reserved), so the 6th pull never happens.
        assert_eq!(
            pulled.get(),
            MAX_SUGGESTIONS - 1,
            "the suggester must stop at the budget, not run dry"
        );
    }

    /// No source may echo the typed word back as its own correction — a menu item that edits
    /// nothing. The personal set was already filtered; the dictionaries were not.
    #[test]
    fn the_typed_word_is_never_its_own_correction() {
        let dict = vec!["helo".to_string(), "hello".to_string()];
        let got = merge_suggestions("helo", vec![(0, "helo".into())], dict.into_iter());
        assert_eq!(got, ["hello"], "the input is dropped from every source: {got:?}");
    }

    /// Ties are ordered deterministically — `personal` is a `HashSet`, and a menu that reshuffles
    /// between right-clicks is unusable.
    #[test]
    fn personal_suggestions_are_stable_across_runs() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        // Three terms all exactly one edit from "Xan" — the tie the HashSet would shuffle.
        let personal: HashSet<String> = ["Xen", "Xin", "Xon"].iter().map(|s| s.to_string()).collect();
        let hl = SpellChecker {
            dicts: vec![Arc::new(dict)],
            personal,
        };
        let first = hl.suggest("Xan");
        assert_eq!(first, ["Xen", "Xin", "Xon"], "equal-distance ties sort alphabetically");
        for _ in 0..5 {
            assert_eq!(hl.suggest("Xan"), first, "the order must not vary between calls");
        }
    }

    /// The bounded edit distance itself: case is folded by the caller, the cap is honoured, and
    /// accented words measure in characters.
    #[test]
    fn bounded_levenshtein_measures_chars_and_honours_the_cap() {
        let d = |a: &str, b: &str| {
            let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
            bounded_levenshtein(&a, &b, 2)
        };
        assert_eq!(d("abc", "abc"), Some(0));
        assert_eq!(d("abc", "abd"), Some(1)); // substitution
        assert_eq!(d("abc", "ab"), Some(1)); // deletion
        assert_eq!(d("abc", "abcd"), Some(1)); // insertion
        assert_eq!(d("abc", "xyz"), None, "3 edits exceeds the cap");
        // A length gap alone exceeds the cap — rejected without building the matrix.
        assert_eq!(d("a", "abcdef"), None);
        // "café" vs "cafe" is one char edit, not two bytes' worth.
        assert_eq!(d("café", "cafe"), Some(1));
        // Empty on either side is the other's length, still subject to the cap.
        assert_eq!(d("", "ab"), Some(2));
        assert_eq!(d("ab", ""), Some(2));
        assert_eq!(d("", ""), Some(0));
    }

    /// The length gate must not reject a candidate whose *lower-cased* form is within the cap
    /// even though its raw form is not — `İ` lower-cases to two chars, so counting the raw word
    /// would measure the wrong length.
    #[test]
    fn a_candidate_is_gated_on_its_lower_cased_length() {
        // "İ" (U+0130) lower-cases to "i̇" — 1 char becomes 2.
        assert_eq!("İ".chars().count(), 1);
        assert_eq!("İ".to_lowercase().chars().count(), 2);
        // A personal word whose lower-cased form is exactly the typed word must be found.
        let hl = SpellChecker::for_tests(&["hello"], &["İstanbul"]);
        let got = hl.suggest("i\u{307}stanbul"); // the lower-cased spelling, typed by the writer
        assert_eq!(
            got.first().map(String::as_str),
            Some("İstanbul"),
            "the stored casing is offered, got {got:?}"
        );
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
        session.apply_exemption(true); // forced: `all_ranges` was poked in directly
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

    /// The B2-M0 fast path: a caret move that stays inside the same exempted word must not
    /// re-run the O(all_ranges) filter+clone. Typing within a word ticks the caret on every
    /// keystroke, so on a densely-flagged document that clone was the per-keystroke cost.
    #[test]
    fn a_same_word_caret_move_skips_the_filter() {
        let doc = tiny_doc("helo wrld"); // both misspelled
        let session = SpellSession::new(&doc);
        let caret = Rc::new(Cell::new(1usize)); // inside "helo" [0,4]
        focus_at(&session, &caret);
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        let after_first = session.exemption_recomputes.get();
        assert_eq!(starts(&session), vec![5], "helo exempt, wrld flagged");

        // Three caret moves that never leave "helo" [0,4] — the exempted range is unchanged.
        for pos in [0usize, 2, 4] {
            caret.set(pos);
            session.on_caret(WidgetId::default());
            session.tick();
        }
        assert_eq!(
            session.exemption_recomputes.get(),
            after_first,
            "staying inside the exempt word takes the fast path — no re-filter"
        );
        assert_eq!(starts(&session), vec![5], "and the pushed set is unchanged");

        // Crossing into "wrld" [5,9] genuinely changes the exemption, so it must recompute.
        caret.set(6);
        session.on_caret(WidgetId::default());
        session.tick();
        assert_eq!(
            session.exemption_recomputes.get(),
            after_first + 1,
            "leaving the word recomputes exactly once"
        );
        assert_eq!(starts(&session), vec![0], "wrld now exempt, helo flagged");
    }

    /// B2-M0b: an inactive (hidden) session defers the eager rebuild `set_checker` would do,
    /// then catches up on the first tick after it is shown. This is what stops a re-attach
    /// (dictionary install / mute / language change) from re-tokenising a hidden 20k-word
    /// synopsis nobody can see.
    #[test]
    fn a_hidden_session_defers_its_rebuild_until_shown() {
        let doc = tiny_doc("helo wrld"); // both misspelled
        let session = SpellSession::new(&doc);
        session.set_active(false);

        // A checker arrives while hidden — stored, but no ranges computed yet.
        session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
        assert!(
            session.all_ranges.borrow().is_empty(),
            "a hidden pane does not tokenise on set_checker"
        );
        assert!(session.last_ranges.borrow().is_empty(), "and nothing is pushed");

        // A tick while still hidden stays a no-op (and must not consume the owed rebuild).
        session.tick();
        assert!(session.all_ranges.borrow().is_empty(), "still nothing while hidden");

        // Shown again → the next tick performs the deferred rebuild.
        session.set_active(true);
        session.tick();
        assert_eq!(
            starts(&session),
            vec![0, 5],
            "the catch-up rebuild runs once the pane is shown (no focused caret → both flagged)"
        );
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
