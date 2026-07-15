//! The spell-checking engine — an internal service, no reactive state of its own (the same
//! shelf as `open_registry` / `ipc`).
//!
//! It loads a [`spellbook::Dictionary`] per language **once**, shared by every open document
//! that needs it (reversing the old app's one-instance-per-editor-tab waste), and produces a
//! [`SyntaxHighlighter`] that squiggles misspellings. The *wiring* — adding/removing the
//! highlight session on a document, and the five call sites that re-attach — lives in
//! `models::open_docs` and `app.rs` (Step 6); this module is pure engine.
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

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use bastyde::prelude::Signal;
use bastyde::text_document::{Color, HighlightContext, HighlightFormat, SyntaxHighlighter, UnderlineStyle};

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
fn word_positions(text: &str) -> Vec<(usize, usize, &str)> {
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

/// A misspelling highlighter over one document's **active** dictionaries. Immutable and
/// `Send + Sync` (the trait's bound), so it is cheaply `Arc`-shared into the document even
/// though it is only ever built and read on the UI thread.
struct SpellHighlighter {
    /// Active dictionaries, primary first; empty is never installed (a highlighter with no
    /// dictionary is never attached).
    dicts: Vec<Arc<spellbook::Dictionary>>,
    /// The Work's personal words — checked first, so removing one is just removing the entity.
    personal: HashSet<String>,
    /// Squiggle colour, resolved from a theme role at attach time (never a hex literal here).
    color: Color,
}

impl SpellHighlighter {
    fn misspelled(&self, word: &str) -> bool {
        // Numbers, punctuation runs, and the like are not spell-checkable.
        if !word.chars().any(|c| c.is_alphabetic()) {
            return false;
        }
        if self.personal.contains(word) {
            return false;
        }
        // A mistake only when EVERY active dictionary rejects it — `any` short-circuits on the
        // primary for correctly-spelled prose.
        !self.dicts.iter().any(|d| d.check(word))
    }
}

impl SyntaxHighlighter for SpellHighlighter {
    fn highlight_block(&self, text: &str, ctx: &mut HighlightContext) {
        for (char_off, len, word) in word_positions(text) {
            if self.misspelled(word) {
                ctx.set_format(
                    char_off,
                    len,
                    HighlightFormat {
                        underline_style: Some(UnderlineStyle::SpellCheckUnderline),
                        underline_color: Some(self.color),
                        ..Default::default()
                    },
                );
            }
        }
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

    /// Build a highlighter for a document's tag list, or `None` when nothing is active/installed
    /// (the caller then removes any existing session — the degrade path). `color` is the
    /// squiggle colour, resolved from a theme role by the caller.
    pub fn build_highlighter(
        &self,
        tags: &str,
        color: Color,
    ) -> Option<Arc<dyn SyntaxHighlighter>> {
        let dicts = self.active_dicts(tags);
        if dicts.is_empty() {
            return None;
        }
        Some(Arc::new(SpellHighlighter {
            dicts,
            personal: self.inner.personal.borrow().clone(),
            color,
        }))
    }
}

impl Default for SpellcheckService {
    fn default() -> Self {
        Self::new()
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
        let hl = SpellHighlighter {
            dicts: vec![Arc::new(dict)],
            personal: HashSet::new(),
            color: Color::rgb(220, 50, 50),
        };
        assert!(hl.misspelled("helo"), "a misspelling is flagged");
        assert!(!hl.misspelled("hello"), "a good word is not");
        assert!(!hl.misspelled("world"), "another good word is not");
        assert!(!hl.misspelled("123"), "a number is never a misspelling");
    }

    /// A personal word overrides the dictionary — checked first, so no dictionary mutation.
    #[test]
    fn personal_words_win() {
        let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        let mut personal = HashSet::new();
        personal.insert("Skribisto".to_string());
        let hl = SpellHighlighter {
            dicts: vec![Arc::new(dict)],
            personal,
            color: Color::rgb(220, 50, 50),
        };
        assert!(!hl.misspelled("Skribisto"), "a personal word is accepted");
        assert!(hl.misspelled("Skrib"), "but not an unrelated unknown word");
    }

    /// The multi-dictionary union: a word only one language knows is still accepted.
    #[test]
    fn a_word_any_active_dictionary_knows_is_accepted() {
        let en = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
        let fr = spellbook::Dictionary::new("SET UTF-8\n", "1\nbonjour\n").unwrap();
        let hl = SpellHighlighter {
            dicts: vec![Arc::new(en), Arc::new(fr)],
            personal: HashSet::new(),
            color: Color::rgb(220, 50, 50),
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
}
