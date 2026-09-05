// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `DictionariesViewModel` — download, remove, and licence-acceptance for spell-check
//! dictionaries.
//!
//! **App-local, not a Qleany feature.** A downloaded `.dic` is a machine-wide resource that
//! outlives any `Work`, is not undoable, and never touches the entity store — so this is a
//! `teksilo_async` background task (the `DestinationsEditor` shape), not a backend
//! `LongOperation`. Single-instance live state: it owns the in-flight download set and holds
//! the `TaskHandle`s (so a download survives the Settings window closing — dropping the handle
//! would cancel it), created once in `App::build` and shared by `.clone()`.
//!
//! ## Guarantees
//!
//! - **Serialised.** One download at a time (a queue), so three Download clicks don't open
//!   three concurrent transfers — the same reason `save_queue` serialises saves.
//! - **Licence-gated as an invariant.** [`download`](DictionariesViewModel::download) refuses
//!   unless [`has_accepted`](DictionariesViewModel::has_accepted) — a hard check, not a UI
//!   convention, so a scripted or toast-driven caller can't bypass the licence modal.
//! - **Atomic.** Files are written to a per-process-unique `…part` path and `rename`d into
//!   place, so a crash or cancel never leaves a *torn file at the final path* for the
//!   installed-scan to mistake for a complete dictionary. (`rename` within a directory is
//!   atomic; the worst a race between two instances can do is redundant work, both writing a
//!   *complete* file — no corruption, so no cross-process lock is needed for correctness.)
//! - **Progress is indeterminate.** App-local work can't post cross-thread progress the way a
//!   backend `LongOperation` does (`Signal`s are UI-thread only), and dictionaries are small,
//!   so the toast is an honest spinner rather than a faked percentage.

use std::cell::RefCell;
use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use teksilo::prelude::*;
use teksilo::widgets::Toast;

use crate::models::{DictionarySettingsService, InstalledDictionariesModel, license_hash};
use crate::spellcheck::dictionary_registry::{self, DictionaryEntry, Source};

/// One toast surface for the whole download activity, updated in place by id.
const DICT_TOAST_ID: &str = "dict.download";

/// How long to wait for the connection itself. Generous enough for a slow link and a
/// distant mirror, short enough that a dropped packet does not park the worker.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);

/// How long to wait for the first byte of the response once connected. The body itself
/// is deliberately unbounded in time; see [`http_get`].
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

/// A short User-Agent so a raw-file host (GitHub, grammalecte) sees a named client.
fn user_agent() -> String {
    format!(
        "Skribisto/{} (dictionary downloader)",
        env!("CARGO_PKG_VERSION")
    )
}

use crate::spellcheck::downloaded_dictionaries_dir as dictionaries_dir;

/// Why an "Add dictionary" install was refused. Deliberately carries **no** user-facing prose —
/// the caller ([`crate::spellcheck::AddDictionaryViewModel`]) maps each variant to a localized
/// message, so a toast is never half-translated. `Unusable`/`Io` carry a technical detail (a
/// spellbook parse error, a filesystem error) that has no useful translation.
#[derive(Debug)]
pub enum InstallDictError {
    /// The name field was blank.
    NameRequired,
    /// The code is empty or has characters unsafe in a filename.
    InvalidCode,
    /// The code names (or, on a case-insensitive filesystem, case-folds onto) a catalogue code.
    Reserved,
    /// A dictionary with this code (or its case variant) is already installed.
    AlreadyInstalled,
    /// The `.aff`/`.dic` don't read or don't parse as a Hunspell dictionary.
    Unusable(String),
    /// A filesystem failure (no data dir, copy, or config write).
    Io(String),
}

#[derive(Clone)]
pub struct DictionariesViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    settings: DictionarySettingsService,
    installed: InstalledDictionariesModel,
    /// Registry ids currently downloading or queued (drives the row spinner).
    downloading: Signal<BTreeSet<String>>,
    /// The id downloading *right now* (serialisation), and the ids waiting behind it.
    active: RefCell<Option<String>>,
    queue: RefCell<VecDeque<String>>,
    /// Bumped whenever the on-disk set changes (install / remove) — `App` observes this to
    /// re-attach the spell-checker to open documents (Step 6).
    changed: Signal<u64>,
    /// The ids the post-open scan found missing — the Settings ▸ Get-more tab highlights these.
    highlight: Signal<Vec<String>>,
    /// Held so their `Drop`-cancel doesn't fire while a download is in flight.
    tasks: RefCell<Vec<TaskHandle>>,
}

impl DictionariesViewModel {
    pub fn new(settings: DictionarySettingsService, installed: InstalledDictionariesModel) -> Self {
        Self {
            inner: Rc::new(Inner {
                settings,
                installed,
                downloading: Signal::new(BTreeSet::new()),
                active: RefCell::new(None),
                queue: RefCell::new(VecDeque::new()),
                changed: Signal::new(0),
                highlight: Signal::new(Vec::new()),
                tasks: RefCell::new(Vec::new()),
            }),
        }
    }

    // ── read handles for the UI ──

    pub fn installed_model(&self) -> InstalledDictionariesModel {
        self.inner.installed.clone()
    }
    pub fn downloading_signal(&self) -> Signal<BTreeSet<String>> {
        self.inner.downloading.clone()
    }
    pub fn changed_signal(&self) -> Signal<u64> {
        self.inner.changed.clone()
    }
    pub fn highlight_signal(&self) -> Signal<Vec<String>> {
        self.inner.highlight.clone()
    }
    pub fn set_highlight(&self, ids: Vec<String>) {
        self.inner.highlight.set(ids);
    }

    /// The accepted-licence store's `Reloadable` hook, for the app's shared `SettingsRegistry`
    /// (register it and keep the handle alive so a peer process's acceptance reloads in place).
    pub fn settings_reloadable(&self) -> Rc<dyn teksilo::settings::Reloadable> {
        self.inner.settings.as_reloadable()
    }

    pub fn is_installed(&self, id: &str) -> bool {
        self.inner.installed.is_installed(id)
    }
    pub fn is_downloading(&self, id: &str) -> bool {
        self.inner.downloading.get().contains(id)
    }

    /// The user-added dictionaries recorded in the config (name + code) — so the language picker
    /// can offer them and show their names, alongside the catalogue.
    pub fn user_dictionaries(&self) -> Vec<crate::models::UserDictionary> {
        self.inner.settings.user_dictionaries()
    }

    // ── licence acceptance (the download gate) ──

    /// The BLAKE3 of the currently-bundled licence text for `id`'s dictionary, or `None` if
    /// the id or its licence asset is unknown.
    fn current_license_hash(id: &str) -> Option<String> {
        let entry = dictionary_registry::by_id(id)?;
        let text = dictionary_registry::license_text(&entry.license_asset)?;
        Some(license_hash(text))
    }

    /// Whether the user has accepted the *current* licence text for `id`.
    pub fn has_accepted(&self, id: &str) -> bool {
        match Self::current_license_hash(id) {
            Some(hash) => self.inner.settings.has_accepted(id, &hash),
            None => false,
        }
    }

    /// Record acceptance of `id`'s licence (the exact bundled text) at `now` (RFC3339).
    pub fn accept_license(&self, id: &str, now: &str) {
        if let Some(hash) = Self::current_license_hash(id) {
            let _ = self.inner.settings.accept(id, &hash, now);
        }
    }

    // ── download ──

    /// Queue a download of `id`. Refuses if not licence-accepted (the invariant), or if it is
    /// already installed / already in flight. Serialised: at most one transfer runs at a time.
    pub fn download(&self, id: &str, ctx: &mut EventContext) {
        if self.is_installed(id) || self.is_downloading(id) {
            return;
        }
        if !self.has_accepted(id) {
            // A caller that reached here without the licence modal (a bug, or a future scripted
            // path) is refused, not silently served.
            //
            // Broadcast: a dictionary is a machine-wide resource that outlives any
            // one Work (see the module doc), so every open window hears about it,
            // not just whichever window's Settings panel is open.
            ctx.show_toast(
                Toast::warning(tr!(dict_accept_first(name = display_of(id)))).broadcast(),
            );
            return;
        }
        self.inner.queue.borrow_mut().push_back(id.to_string());
        self.set_downloading(id, true);
        self.pump(ctx);
    }

    /// Start the next queued download if none is in flight.
    fn pump(&self, ctx: &mut EventContext) {
        if self.inner.active.borrow().is_some() {
            return;
        }
        let Some(id) = self.inner.queue.borrow_mut().pop_front() else {
            return;
        };
        *self.inner.active.borrow_mut() = Some(id.clone());
        self.start_download(id, ctx);
    }

    fn start_download(&self, id: String, ctx: &mut EventContext) {
        let Some(entry) = dictionary_registry::by_id(&id).cloned() else {
            return self.finish_one(&id, Err("unknown dictionary".into()), ctx);
        };
        let Some(dest) = dictionaries_dir() else {
            return self.finish_one(&id, Err("no data directory available".into()), ctx);
        };
        ctx.show_toast(
            Toast::loading(tr!(dict_download_title(name = entry.display_name.clone())))
                .id(DICT_TOAST_ID)
                .broadcast(),
        );

        let me = self.clone();
        let id_for_done = id.clone();
        let handle = ctx.spawn_local_with(
            async move {
                spawn_blocking(move || do_download(&entry, &dest))
                    .await
                    .unwrap_or_else(|_| Err("the download worker stopped unexpectedly".to_string()))
            },
            move |result, ctx2| me.finish_one(&id_for_done, result, ctx2),
        );
        self.inner.tasks.borrow_mut().push(handle);
    }

    fn finish_one(&self, id: &str, result: Result<(), String>, ctx: &mut EventContext) {
        *self.inner.active.borrow_mut() = None;
        self.set_downloading(id, false);
        match result {
            Ok(()) => {
                self.inner.installed.refresh();
                self.bump_changed();
                ctx.show_toast(
                    Toast::success(tr!(dict_download_done(name = display_of(id))))
                        .id(DICT_TOAST_ID)
                        .auto_dismiss_after(Duration::from_secs(4))
                        .broadcast(),
                );
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(dict_download_failed(name = display_of(id), error = e)))
                        .id(DICT_TOAST_ID)
                        .broadcast(),
                );
            }
        }
        self.pump(ctx);
    }

    // ── remove ──

    /// Delete a downloaded dictionary's files and refresh. Bumps `changed` so the spell-checker
    /// re-attaches (degrading any open document that used it — never rewriting `dict_language`).
    pub fn remove(&self, id: &str, ctx: &mut EventContext) {
        let name = self.display_of_installed(id);
        if let Some(dir) = dictionaries_dir() {
            let _ = std::fs::remove_file(dir.join(format!("{id}.aff")));
            let _ = std::fs::remove_file(dir.join(format!("{id}.dic")));
        }
        // Drop any user-added record for this code — a no-op for a downloaded registry dictionary.
        let _ = self.inner.settings.remove_user_dictionary(id);
        self.inner.installed.refresh();
        self.bump_changed();
        ctx.show_toast(
            Toast::info(tr!(dict_removed(name = name)))
                .auto_dismiss_after(Duration::from_secs(3))
                .broadcast(),
        );
    }

    /// Install a dictionary the user picked from local `.aff`/`.dic` files. Validates that the
    /// pair actually parses (the same read+transcode+spellbook path the loader uses), copies both
    /// into the download dir as `{code}.aff`/`.dic` — so the loader and the install scan find it
    /// exactly like a downloaded one, with no engine change — and records its display name. The
    /// files are local and small, so this is synchronous (unlike a network download). Returns a
    /// human-readable error the caller surfaces; on success it refreshes the installed list and
    /// bumps `changed` so open documents re-attach and pick the new dictionary up live.
    pub fn install_user_dictionary(
        &self,
        name: &str,
        code: &str,
        aff_src: &std::path::Path,
        dic_src: &std::path::Path,
    ) -> Result<(), InstallDictError> {
        use InstallDictError as E;
        let name = name.trim();
        let code = code.trim();
        if name.is_empty() {
            return Err(E::NameRequired);
        }
        if !is_valid_code(code) {
            return Err(E::InvalidCode);
        }
        // A custom code must not shadow a catalogue dictionary (those are added through "Get
        // more"). Case-insensitive, because the download dir is case-insensitive on Windows/macOS
        // — `EN-US.aff` and `en-US.aff` are one file there, so `EN-US` would clobber `en-US`.
        if dictionary_registry::collides_with_catalogue(code) {
            return Err(E::Reserved);
        }
        crate::spellcheck::validate_dictionary_files(aff_src, dic_src).map_err(E::Unusable)?;
        let dest = dictionaries_dir().ok_or_else(|| E::Io("no data directory available".into()))?;
        std::fs::create_dir_all(&dest).map_err(|e| E::Io(format!("create dir: {e}")))?;
        // Refuse if either destination already exists. `exists()` honours the filesystem's own
        // case sensitivity, so this catches a re-add AND a case-variant collision on Win/macOS —
        // otherwise the copy below would silently overwrite an installed dictionary.
        let aff_dest = dest.join(format!("{code}.aff"));
        let dic_dest = dest.join(format!("{code}.dic"));
        if aff_dest.exists() || dic_dest.exists() {
            return Err(E::AlreadyInstalled);
        }
        copy_pair(code, aff_src, dic_src, &dest).map_err(E::Io)?;
        // Record the name; if that write fails, roll the copied files back so we never orphan
        // files with no name record (which would then read as "⟨code⟩ (system)" and block re-add).
        if let Err(e) = self.inner.settings.add_user_dictionary(code, name) {
            let _ = std::fs::remove_file(&aff_dest);
            let _ = std::fs::remove_file(&dic_dest);
            return Err(E::Io(format!("could not save the dictionary record: {e}")));
        }
        self.inner.installed.refresh();
        self.bump_changed();
        Ok(())
    }

    /// Whether an installed dictionary carries `code`, case-insensitively — for the Add form to
    /// flag a re-add before the install refuses it.
    pub fn is_installed_ci(&self, code: &str) -> bool {
        self.inner.installed.is_installed_ci(code)
    }

    /// The installed list's display name for `id` (so a removed *user* dictionary's toast shows
    /// the name the user gave it, not just its code); falls back to the registry, then the id.
    fn display_of_installed(&self, id: &str) -> String {
        self.inner
            .installed
            .display_name(id)
            .unwrap_or_else(|| display_of(id))
    }

    /// Re-scan disk without any change of our own — for the window-focus-regain path, where a
    /// *peer* process may have installed or removed a dictionary.
    pub fn rescan(&self) {
        self.inner.installed.refresh();
        self.bump_changed();
    }

    fn bump_changed(&self) {
        self.inner
            .changed
            .set(self.inner.changed.get().wrapping_add(1));
    }

    fn set_downloading(&self, id: &str, on: bool) {
        let mut set = self.inner.downloading.get();
        let changed = if on {
            set.insert(id.to_string())
        } else {
            set.remove(id)
        };
        if changed {
            self.inner.downloading.set(set);
        }
    }
}

/// The registry display name for an id, or the id itself if unknown — for a toast/label.
fn display_of(id: &str) -> String {
    dictionary_registry::by_id(id)
        .map(|e| e.display_name.clone())
        .unwrap_or_else(|| id.to_string())
}

/// Which registry ids a project's used languages need but the machine lacks.
///
/// Pure over an `is_installed` predicate so it is unit-testable without a filesystem. A tag is
/// "missing" only when it resolves to a **known, downloadable** registry id that isn't
/// installed — an unrecognised tag names no dictionary to offer.
pub fn missing_from(tags: &BTreeSet<String>, is_installed: impl Fn(&str) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for tag in tags {
        if let Some(id) = dictionary_registry::resolve_token(tag)
            && seen.insert(id)
            && !is_installed(id)
        {
            out.push(id.to_string());
        }
    }
    out
}

impl DictionariesViewModel {
    /// The downloadable dictionaries a project's used languages are missing (see
    /// [`missing_from`]).
    pub fn missing_for(&self, tags: &BTreeSet<String>) -> Vec<String> {
        missing_from(tags, |id| self.is_installed(id))
    }
}

/// Fetch and install one dictionary's `.aff`/`.dic` (blocking; runs on a worker thread).
fn do_download(entry: &DictionaryEntry, dest_dir: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dest_dir).map_err(|e| format!("create dir: {e}"))?;
    let (aff_bytes, dic_bytes) = match &entry.source {
        Source::DirectFiles { aff_url, dic_url } => (http_get(aff_url)?, http_get(dic_url)?),
        Source::ZipMember {
            zip_url,
            aff_member,
            dic_member,
        } => {
            let zip_bytes = cached_zip(zip_url, dest_dir)?;
            (
                zip_member(&zip_bytes, aff_member)?,
                zip_member(&zip_bytes, dic_member)?,
            )
        }
    };
    // Both files must be complete before either lands, so a half-install can't be seen.
    write_atomic(&dest_dir.join(format!("{}.aff", entry.id)), &aff_bytes)?;
    write_atomic(&dest_dir.join(format!("{}.dic", entry.id)), &dic_bytes)?;
    Ok(())
}

/// A blocking HTTP GET to `Vec<u8>`, with the 10 MB default read cap raised (some `.dic`s are
/// larger) and a descriptive User-Agent.
///
/// ## Why the two timeouts, and why not a third
///
/// `ureq` 3 sets **no** timeout of any kind by default: connect, resolve, send and
/// receive are all `None` (`ureq::config`). A firewall that drops packets rather than
/// refusing them therefore parks this worker thread forever, and because downloads are
/// serialised behind one queue, that one stall takes the whole dictionary feature with
/// it for the rest of the session. Neither the toast nor the queue has a way out: the
/// call never returns to fire either.
///
/// [`Config::timeout_connect`] bounds reaching the host and
/// [`Config::timeout_recv_response`] bounds the wait for the first byte of the
/// response. There is deliberately **no** `timeout_global`: it would cap the whole
/// transfer including the body, and a body here is up to 64 MB over whatever line the
/// writer has. A cap generous enough for a slow line is no cap at all, and one tight
/// enough to be useful would cancel honest downloads.
///
/// [`Config::timeout_connect`]: https://docs.rs/ureq/3/ureq/config/struct.ConfigBuilder.html
/// [`Config::timeout_recv_response`]: https://docs.rs/ureq/3/ureq/config/struct.ConfigBuilder.html
fn http_get(url: &str) -> Result<Vec<u8>, String> {
    ureq::get(url)
        .config()
        .timeout_connect(Some(CONNECT_TIMEOUT))
        .timeout_recv_response(Some(RESPONSE_TIMEOUT))
        .build()
        .header("User-Agent", &user_agent())
        .call()
        .map_err(|e| format!("request failed: {e}"))?
        .body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_to_vec()
        .map_err(|e| format!("read failed: {e}"))
}

/// The grammalecte zip, fetched once and cached under `dest_dir/.zipcache/` so installing all
/// three French variants downloads it a single time.
fn cached_zip(url: &str, dest_dir: &std::path::Path) -> Result<Vec<u8>, String> {
    let cache_dir = dest_dir.join(".zipcache");
    std::fs::create_dir_all(&cache_dir).map_err(|e| format!("cache dir: {e}"))?;
    let cache_path = cache_dir.join(format!("{}.zip", blake3::hash(url.as_bytes()).to_hex()));
    if let Ok(bytes) = std::fs::read(&cache_path) {
        return Ok(bytes);
    }
    let bytes = http_get(url)?;
    write_atomic(&cache_path, &bytes)?;
    Ok(bytes)
}

/// Read one member of a zip archive to bytes.
fn zip_member(zip_bytes: &[u8], member: &str) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes))
        .map_err(|e| format!("open zip: {e}"))?;
    let mut file = archive
        .by_name(member)
        .map_err(|e| format!("zip member {member:?}: {e}"))?;
    let mut buf = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut buf)
        .map_err(|e| format!("read zip member {member:?}: {e}"))?;
    Ok(buf)
}

/// Whether `code` is usable as an on-disk basename (`{code}.aff`): non-empty, at least one
/// alphanumeric, and only characters safe in a filename and a BCP-47-ish tag. This keeps a custom
/// code from smuggling a path separator into the copy destination.
fn is_valid_code(code: &str) -> bool {
    !code.is_empty()
        && code.chars().any(|c| c.is_ascii_alphanumeric())
        && code
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Copy a local `.aff`/`.dic` pair into `dest_dir` as `{code}.aff`/`.dic`, together: the bytes
/// are read first (so a missing source fails before anything is written), and if the second write
/// fails the first is rolled back — no half-install for the scan to mistake for a whole one. The
/// bytes are copied **raw**, preserving the `.aff`'s `SET` encoding directive so the loader
/// transcodes it correctly later.
fn copy_pair(
    code: &str,
    aff_src: &std::path::Path,
    dic_src: &std::path::Path,
    dest_dir: &std::path::Path,
) -> Result<(), String> {
    let aff_bytes = std::fs::read(aff_src).map_err(|e| format!("read .aff: {e}"))?;
    let dic_bytes = std::fs::read(dic_src).map_err(|e| format!("read .dic: {e}"))?;
    let aff_dest = dest_dir.join(format!("{code}.aff"));
    let dic_dest = dest_dir.join(format!("{code}.dic"));
    write_atomic(&aff_dest, &aff_bytes)?;
    if let Err(e) = write_atomic(&dic_dest, &dic_bytes) {
        let _ = std::fs::remove_file(&aff_dest); // roll back the half-install
        return Err(e);
    }
    Ok(())
}

/// Write `bytes` to `final_path` atomically: a per-process-unique temp sibling, then `rename`.
fn write_atomic(final_path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let tmp = final_path.with_extension(format!("{}.{n}.part", std::process::id()));
    std::fs::write(&tmp, bytes).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, final_path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("install {}: {e}", final_path.display())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `missing_from` offers only downloadable, not-installed, deduped tags — and ignores
    /// unrecognised ones.
    #[test]
    fn missing_offers_only_downloadable_uninstalled() {
        let mut tags = BTreeSet::new();
        tags.insert("en-US".to_string()); // downloadable, pretend installed → not offered
        tags.insert("fr-FR".to_string()); // downloadable, not installed → offered
        tags.insert("fr".to_string()); // resolves to fr-FR (basename) → deduped, already offered
        tags.insert("kl-KL".to_string()); // unrecognised → ignored

        let installed = |id: &str| id == "en-US";
        let missing = missing_from(&tags, installed);

        assert_eq!(missing, vec!["fr-FR".to_string()]);
    }

    /// The custom-code guard: accepts tag-shaped codes, rejects empty, separator-bearing, and
    /// all-punctuation ones (which would smuggle a path into the copy destination).
    #[test]
    fn is_valid_code_guards_the_basename() {
        assert!(is_valid_code("fr-FR-x-custom"));
        assert!(is_valid_code("cy_GB"));
        assert!(is_valid_code("la"));
        assert!(!is_valid_code(""), "empty");
        assert!(!is_valid_code("../etc"), "path separator");
        assert!(!is_valid_code("a/b"), "slash");
        assert!(!is_valid_code("fr FR"), "space");
        assert!(!is_valid_code("--."), "no alphanumeric");
    }

    /// `copy_pair` lands both files under the code, and a missing source fails before writing
    /// anything (no half-install).
    #[test]
    fn copy_pair_lands_both_or_nothing() {
        let dir = std::env::temp_dir().join(format!("skrib-cp-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let aff_src = dir.join("src.aff");
        let dic_src = dir.join("src.dic");
        std::fs::write(&aff_src, b"SET UTF-8\n").unwrap();
        std::fs::write(&dic_src, b"1\nhello\n").unwrap();
        let dest = dir.join("dest");
        std::fs::create_dir_all(&dest).unwrap();

        copy_pair("fr-x", &aff_src, &dic_src, &dest).unwrap();
        assert_eq!(
            std::fs::read(dest.join("fr-x.aff")).unwrap(),
            b"SET UTF-8\n"
        );
        assert_eq!(std::fs::read(dest.join("fr-x.dic")).unwrap(), b"1\nhello\n");

        // A missing .dic source: the read fails before any write, so nothing lands.
        let bad = copy_pair("gg-x", &aff_src, &dir.join("nope.dic"), &dest);
        assert!(bad.is_err());
        assert!(
            !dest.join("gg-x.aff").exists(),
            "nothing written on a source read error"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A round-trip through the atomic writer leaves the exact bytes at the final path and no
    /// leftover `.part`.
    #[test]
    fn write_atomic_lands_bytes_and_cleans_up() {
        let dir = std::env::temp_dir().join(format!("skrib-wa-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let target = dir.join("x.dic");
        write_atomic(&target, b"hello").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"hello");
        let leftovers: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .filter(|e| e.path().to_string_lossy().contains(".part"))
            .collect();
        assert!(leftovers.is_empty(), "a .part file was left behind");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
