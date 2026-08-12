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
use std::path::{Path, PathBuf};
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

/// The encoding an `.aff` declares via its `SET <name>` directive (UTF-8 if none/unknown).
fn detect_encoding(aff_bytes: &[u8]) -> &'static encoding_rs::Encoding {
    // `SET` and the label are ASCII, so a lossy decode of the head is safe to scan.
    let head = String::from_utf8_lossy(&aff_bytes[..aff_bytes.len().min(1024)]);
    for line in head.lines() {
        if let Some(rest) = line.strip_prefix("SET ")
            && let Some(enc) = encoding_rs::Encoding::for_label(rest.trim().as_bytes())
        {
            return enc;
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
/// [`HighlightContext::set_format`](teksilo::text_document::HighlightContext::set_format)
/// expects (character positions, not bytes). UAX#29 word
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
        let dict = spellbook::Dictionary::new("SET UTF-8\n", &dic).expect("tiny dictionary parses");
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
    // Every push above is gated on a ceiling, so no trailing truncate is needed — adding one back
    // would silently eat the reserved slots.
    out
}

/// Push `s` unless it is the word the writer typed, or an equal suggestion is already there.
///
/// The single gate every source passes through. Deduping keeps the first (better-ranked)
/// occurrence when two dictionaries offer the same correction; rejecting `typed` means no source
/// can echo the input back as its own correction — a menu item that would edit nothing.
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
        Some(SpellChecker { dicts, personal })
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
