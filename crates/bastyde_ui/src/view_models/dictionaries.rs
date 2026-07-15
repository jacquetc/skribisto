//! `DictionariesViewModel` — download, remove, and licence-acceptance for spell-check
//! dictionaries.
//!
//! **App-local, not a Qleany feature.** A downloaded `.dic` is a machine-wide resource that
//! outlives any `Work`, is not undoable, and never touches the entity store — so this is a
//! `bastyde_async` background task (the `DestinationsEditor` shape), not a backend
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

use bastyde::prelude::*;
use bastyde::widgets::Toast;

use crate::dictionary_registry::{self, DictionaryEntry, Source};
use crate::models::{DictionarySettingsService, InstalledDictionariesModel, license_hash};

/// One toast surface for the whole download activity, updated in place by id.
const DICT_TOAST_ID: &str = "dict.download";

/// A short User-Agent so a raw-file host (GitHub, grammalecte) sees a named client.
fn user_agent() -> String {
    format!("Skribisto/{} (dictionary downloader)", env!("CARGO_PKG_VERSION"))
}

use crate::spellcheck::downloaded_dictionaries_dir as dictionaries_dir;

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
    pub fn settings_reloadable(&self) -> Rc<dyn bastyde::settings::Reloadable> {
        self.inner.settings.as_reloadable()
    }

    pub fn is_installed(&self, id: &str) -> bool {
        self.inner.installed.is_installed(id)
    }
    pub fn is_downloading(&self, id: &str) -> bool {
        self.inner.downloading.get().contains(id)
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
            ctx.show_toast(Toast::warning(tr!(dict_accept_first(name = display_of(id)))));
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
                .id(DICT_TOAST_ID),
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
                        .auto_dismiss_after(Duration::from_secs(4)),
                );
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(dict_download_failed(name = display_of(id), error = e)))
                        .id(DICT_TOAST_ID),
                );
            }
        }
        self.pump(ctx);
    }

    // ── remove ──

    /// Delete a downloaded dictionary's files and refresh. Bumps `changed` so the spell-checker
    /// re-attaches (degrading any open document that used it — never rewriting `dict_language`).
    pub fn remove(&self, id: &str, ctx: &mut EventContext) {
        if let Some(dir) = dictionaries_dir() {
            let _ = std::fs::remove_file(dir.join(format!("{id}.aff")));
            let _ = std::fs::remove_file(dir.join(format!("{id}.dic")));
        }
        self.inner.installed.refresh();
        self.bump_changed();
        ctx.show_toast(
            Toast::info(tr!(dict_removed(name = display_of(id))))
                .auto_dismiss_after(Duration::from_secs(3)),
        );
    }

    /// Re-scan disk without any change of our own — for the window-focus-regain path, where a
    /// *peer* process may have installed or removed a dictionary.
    pub fn rescan(&self) {
        self.inner.installed.refresh();
        self.bump_changed();
    }

    fn bump_changed(&self) {
        self.inner.changed.set(self.inner.changed.get().wrapping_add(1));
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
pub fn missing_from(
    tags: &BTreeSet<String>,
    is_installed: impl Fn(&str) -> bool,
) -> Vec<String> {
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
fn http_get(url: &str) -> Result<Vec<u8>, String> {
    ureq::get(url)
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
