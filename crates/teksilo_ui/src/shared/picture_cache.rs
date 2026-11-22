// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Decoded pictures for the Gallery and the picture viewer, made away from the interface
//! thread and kept within a memory budget.
//!
//! A folder of research screenshots is hundreds of pictures of a few megapixels each.
//! Decoding them where they are drawn would freeze the window for the length of a scroll,
//! and keeping every one decoded would hold gigabytes. So a tile *asks* for its picture and
//! is handed a [`Signal`] that turns [`PictureState::Ready`] once a background thread has
//! decoded and shrunk it; meanwhile it draws a placeholder of the same size.
//!
//! ## What bounds the cost
//!
//! - **Threads.** `spawn_blocking` starts one OS thread per call, so at most
//!   [`MAX_WORKERS`] decodes run at once and the rest wait in a queue. The queue is
//!   newest first: the tiles asked for last are the ones the writer has just scrolled to,
//!   and a tile scrolled past long ago should not hold them up. Past [`MAX_QUEUE`] the
//!   oldest requests are dropped, and their tiles ask again if they are ever shown.
//! - **Memory.** Decoded pixels are kept up to [`MEMORY_BUDGET`], the least recently asked
//!   for going first. A picture dropped from memory is decoded again (cheaply, from the
//!   disk cache) the next time a tile shows it.
//! - **Disk.** A thumbnail is written once, as a PNG in the user's cache directory, named by
//!   the picture's content hash and its size. Never in the project: an exploded-folder
//!   project's `assets/` holds what the model lists and nothing else (see
//!   `skrib_format::media`), and a thumbnail is something the OS may reclaim. A hash names
//!   the same bytes forever, so a cached thumbnail can never be stale.
//! - **Hostile files.** A picture is read only after its header passes the ceilings an
//!   import applies ([`document_ingest::pictures::displayable_size`]): a project can come
//!   from anywhere, and a few crafted bytes can claim billions of pixels.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use teksilo::prelude::{AsyncRuntimeHandle, Signal, spawn_blocking};

/// How many pictures decode at once.
pub const MAX_WORKERS: usize = 3;

/// How many requests may wait. Beyond it the oldest are dropped.
pub const MAX_QUEUE: usize = 256;

/// The most decoded pixels kept in memory: 256 MiB, about two hundred and fifty thumbnails
/// at their largest, or a handful of full-size screenshots.
pub const MEMORY_BUDGET: usize = 256 << 20;

/// The longest side of the largest thumbnail a tile may ask for, in physical pixels.
pub const MAX_THUMB_EDGE: u32 = 1024;

/// One picture's pixels, decoded and upright, ready for `ImageWidget::from_raw`.
#[derive(Debug)]
pub struct Decoded {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Decoded {
    fn bytes(&self) -> usize {
        self.pixels.len()
    }
}

/// Where a requested picture is.
#[derive(Debug, Clone)]
pub enum PictureState {
    /// Not asked for, or dropped from memory since: a tile seeing this asks again.
    Idle,
    /// Waiting for, or in, a decode.
    Loading,
    Ready(Rc<Decoded>),
    /// The file is missing, refused or does not decode. Said once; not retried until the
    /// picture is asked for afresh.
    Failed,
}

impl PartialEq for PictureState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Idle, Self::Idle)
            | (Self::Loading, Self::Loading)
            | (Self::Failed, Self::Failed) => true,
            (Self::Ready(a), Self::Ready(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// A picture at one size: its content hash and the longest side asked for, `0` for the
/// picture's own size.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PictureKey {
    pub hash: String,
    pub edge: u32,
}

impl PictureKey {
    /// The key for the picture stored under `src` (`assets/<hash>.<ext>`), at `edge`.
    ///
    /// `None` for a reference outside the project's pictures: a hand-typed path or an
    /// address on the web, which nothing here reads.
    pub fn for_src(src: &str, edge: u32) -> Option<Self> {
        let hash = asset_hash(src)?;
        Some(Self {
            hash: hash.to_string(),
            edge,
        })
    }
}

/// The content hash a project picture's reference carries, `None` for any other reference.
///
/// A reference the app writes is `assets/<64 hex digits>.<ext>`, and only that shape is
/// accepted: the hash becomes a file name in the cache directory, so it must never be able
/// to name anything else.
pub fn asset_hash(src: &str) -> Option<&str> {
    let name = src.strip_prefix("assets/")?;
    let (stem, ext) = name.rsplit_once('.')?;
    let is_ext = !ext.is_empty() && ext.bytes().all(|b| b.is_ascii_alphanumeric());
    (is_content_hash(stem) && is_ext).then_some(stem)
}

/// Whether `hash` has the shape of a content hash: 64 hex digits, a blake3 as hex.
///
/// The one rule for what may name a file in the cache directory. A resource's recorded
/// hash comes from the project's own files, which can come from anywhere: a crafted
/// `"../../somewhere/else"` would otherwise make a thumbnail write outside the cache.
pub fn is_content_hash(hash: &str) -> bool {
    hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The context [`file_key_hash`] derives a key under, so a key made from a file's place
/// is never the content hash of any file.
const FILE_KEY_CONTEXT: &str = "Skribisto 2026 picture cache key of a file by its place";

/// The hash to key the picture at `path` on, `recorded` being the content hash the
/// project holds for it.
///
/// The recorded hash, when it is one ([`is_content_hash`]). A linked file has none, since
/// adding it reads nothing, and neither has a Scrivener bookmark; a crafted project's may
/// not be one. Such a file is keyed by where it is, its length and when it last changed,
/// as the system reports them: an edit to the file makes a new key, so its thumbnail is
/// never stale, and the key costs one look at the file's metadata rather than a read of
/// its bytes. Derived under its own context, so it cannot be the content hash of any file,
/// and still 64 hex digits, so the disk cache keeps it like any other.
///
/// `None` when the file's metadata cannot be read: there is nothing to show then anyway.
pub fn file_key_hash(path: &Path, recorded: &str) -> Option<String> {
    if is_content_hash(recorded) {
        return Some(recorded.to_string());
    }
    let meta = std::fs::metadata(path).ok()?;
    let changed = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let mut hasher = blake3::Hasher::new_derive_key(FILE_KEY_CONTEXT);
    // The path first and the two fixed-width numbers after it: no two files' inputs can
    // run into each other.
    hasher.update(path.as_os_str().as_encoded_bytes());
    hasher.update(&meta.len().to_le_bytes());
    hasher.update(&changed.to_le_bytes());
    Some(hasher.finalize().to_hex().to_string())
}

/// The file in `media_dir` a project picture's reference names, under the same rule as
/// [`asset_hash`].
pub fn asset_path(media_dir: &Path, src: &str) -> Option<PathBuf> {
    asset_hash(src)?;
    let base = Path::new(src).file_name()?;
    Some(media_dir.join(base))
}

/// Read, check and decode the picture at `path`, shrunk so its longest side is at most
/// `edge` (`0`: its own size), using and filling the thumbnail cache in `disk` when given.
///
/// Runs on a worker thread. `None` for anything that does not end in pixels: a missing or
/// unreadable file, a format the editor does not paint, a header past the ceilings.
pub fn load(path: &Path, key: &PictureKey, disk: Option<&Path>) -> Option<Decoded> {
    let cached = disk.and_then(|dir| cached_file(dir, key));
    if let Some(file) = &cached
        && let Some(decoded) = decode_checked(file, key.edge)
    {
        return Some(decoded);
    }
    let decoded = decode_checked(path, 0)?;
    if key.edge == 0 || decoded.width.max(decoded.height) <= key.edge {
        return Some(decoded);
    }
    let icon = teksilo::canvas::RasterIcon::from_raw(decoded.pixels, decoded.width, decoded.height);
    let small = icon.downsample_to_max(key.edge).unwrap_or(icon);
    if let Some(file) = &cached {
        write_thumbnail(file, &small);
    }
    Some(Decoded {
        pixels: small.pixels().to_vec(),
        width: small.width(),
        height: small.height(),
    })
}

/// The file in the cache directory `dir` that holds `key`'s thumbnail. `None` for a
/// picture at its own size, which is never cached, and for a key whose hash could name
/// anything but a file in `dir` ([`is_content_hash`]): such a picture is still decoded,
/// only never cached.
fn cached_file(dir: &Path, key: &PictureKey) -> Option<PathBuf> {
    (key.edge > 0 && is_content_hash(&key.hash))
        .then(|| dir.join(format!("{}-{}.png", key.hash, key.edge)))
}

/// Decode `path` once its header has passed the import ceilings. `max_edge`, when not `0`,
/// also refuses a file larger than that, which only a cached thumbnail may be.
fn decode_checked(path: &Path, max_edge: u32) -> Option<Decoded> {
    let size = std::fs::metadata(path).ok()?.len();
    if size > document_ingest::pictures::MAX_PICTURE_BYTES {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    let (width, height) = document_ingest::pictures::displayable_size(&bytes).ok()?;
    if max_edge > 0 && width.max(height) > max_edge {
        return None;
    }
    let icon = teksilo::canvas::RasterIcon::decode(&bytes).ok()?;
    Some(Decoded {
        pixels: icon.pixels().to_vec(),
        width: icon.width(),
        height: icon.height(),
    })
}

/// Write a thumbnail beside its peers, through a temporary file so a reader never sees half
/// of one. A failure costs only the next decode, so it is not reported.
fn write_thumbnail(file: &Path, icon: &teksilo::canvas::RasterIcon) {
    // Only ever a name `cached_file` made: a bare file name under the cache directory.
    let named_by_a_hash = file
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.rsplit_once('-'))
        .is_some_and(|(hash, edge)| is_content_hash(hash) && edge.parse::<u32>().is_ok());
    if !named_by_a_hash {
        return;
    }
    let Some(dir) = file.parent() else { return };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let Some(png) = crate::shared::images::encode_png(icon) else {
        return;
    };
    let partial = file.with_extension(format!("{}.partial", std::process::id()));
    if std::fs::write(&partial, png).is_ok() && std::fs::rename(&partial, file).is_err() {
        let _ = std::fs::remove_file(&partial);
    }
}

/// The directory thumbnails are cached in, `None` when the platform offers no cache
/// directory (thumbnails are then decoded each time, and nothing else changes).
pub fn default_disk_dir() -> Option<PathBuf> {
    crate::identity::app_paths().map(|paths| paths.cache_dir().join("thumbnails"))
}

struct Job {
    key: PictureKey,
    path: PathBuf,
}

struct Inner {
    budget: usize,
    disk: Option<PathBuf>,
    /// Most recently asked for first.
    ready: RefCell<VecDeque<(PictureKey, Rc<Decoded>)>>,
    used: Cell<usize>,
    slots: RefCell<HashMap<PictureKey, Signal<PictureState>>>,
    queue: RefCell<VecDeque<Job>>,
    in_flight: Cell<usize>,
}

/// The process's decoded pictures. Cheap to clone; every clone is the same cache.
#[derive(Clone)]
pub struct PictureCache {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for PictureCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictureCache")
            .field("used", &self.inner.used.get())
            .field("queued", &self.inner.queue.borrow().len())
            .finish()
    }
}

thread_local! {
    static SHARED: PictureCache = PictureCache::new(MEMORY_BUDGET, default_disk_dir());
}

impl PictureCache {
    pub fn new(budget: usize, disk: Option<PathBuf>) -> Self {
        Self {
            inner: Rc::new(Inner {
                budget,
                disk,
                ready: RefCell::new(VecDeque::new()),
                used: Cell::new(0),
                slots: RefCell::new(HashMap::new()),
                queue: RefCell::new(VecDeque::new()),
                in_flight: Cell::new(0),
            }),
        }
    }

    /// The one cache every view shares. Pictures are addressed by content, so a thumbnail
    /// one project's Gallery decoded is the same picture in another's.
    pub fn shared() -> Self {
        SHARED.with(Clone::clone)
    }

    /// Ask for the picture at `path` under `key`, decoding it on `rt` (synchronously when
    /// there is no runtime, as in a headless test). The returned signal says where it is,
    /// and changes when it arrives.
    pub fn request(
        &self,
        rt: Option<&AsyncRuntimeHandle>,
        path: PathBuf,
        key: PictureKey,
    ) -> Signal<PictureState> {
        let slot = self
            .inner
            .slots
            .borrow_mut()
            .entry(key.clone())
            .or_insert_with(|| Signal::new(PictureState::Idle))
            .clone();
        if let Some(decoded) = self.touch(&key) {
            slot.set_if_changed(PictureState::Ready(decoded));
            return slot;
        }
        match slot.get() {
            PictureState::Loading | PictureState::Failed => return slot,
            PictureState::Idle | PictureState::Ready(_) => {}
        }
        slot.set(PictureState::Loading);
        {
            let mut queue = self.inner.queue.borrow_mut();
            queue.push_front(Job { key, path });
            while queue.len() > MAX_QUEUE {
                if let Some(dropped) = queue.pop_back()
                    && let Some(old) = self.inner.slots.borrow_mut().remove(&dropped.key)
                {
                    old.set(PictureState::Idle);
                }
            }
        }
        self.pump(rt);
        slot
    }

    /// Bytes of decoded pixels held now.
    pub fn used(&self) -> usize {
        self.inner.used.get()
    }

    /// The decoded picture under `key`, moved to the front, if memory still holds it.
    fn touch(&self, key: &PictureKey) -> Option<Rc<Decoded>> {
        let mut ready = self.inner.ready.borrow_mut();
        let at = ready.iter().position(|(k, _)| k == key)?;
        let entry = ready.remove(at)?;
        let decoded = entry.1.clone();
        ready.push_front(entry);
        Some(decoded)
    }

    fn pump(&self, rt: Option<&AsyncRuntimeHandle>) {
        while self.inner.in_flight.get() < MAX_WORKERS {
            let Some(job) = self.inner.queue.borrow_mut().pop_front() else {
                return;
            };
            let disk = self.inner.disk.clone();
            match rt {
                Some(rt) => {
                    self.inner.in_flight.set(self.inner.in_flight.get() + 1);
                    let me = self.clone();
                    let rt_again = rt.clone();
                    rt.spawn_local(async move {
                        let Job { key, path } = job;
                        let worker_key = key.clone();
                        let decoded =
                            spawn_blocking(move || load(&path, &worker_key, disk.as_deref()))
                                .await
                                .ok()
                                .flatten();
                        me.inner
                            .in_flight
                            .set(me.inner.in_flight.get().saturating_sub(1));
                        me.finish(key, decoded);
                        me.pump(Some(&rt_again));
                    })
                    .detach();
                }
                None => {
                    let decoded = load(&job.path, &job.key, disk.as_deref());
                    self.finish(job.key, decoded);
                }
            }
        }
    }

    fn finish(&self, key: PictureKey, decoded: Option<Decoded>) {
        let slot = self.inner.slots.borrow().get(&key).cloned();
        let Some(decoded) = decoded else {
            if let Some(slot) = slot {
                slot.set(PictureState::Failed);
            }
            return;
        };
        let decoded = Rc::new(decoded);
        self.inner.used.set(self.inner.used.get() + decoded.bytes());
        self.inner
            .ready
            .borrow_mut()
            .push_front((key.clone(), decoded.clone()));
        self.evict();
        // Set last: a tile rebuilding on this change asks again, and must find it held.
        if let Some(slot) = slot {
            slot.set(PictureState::Ready(decoded));
        }
    }

    /// Drop the least recently asked for until the budget holds, always keeping the newest.
    fn evict(&self) {
        loop {
            let over = self.inner.used.get() > self.inner.budget;
            let Some((key, decoded)) = ({
                let mut ready = self.inner.ready.borrow_mut();
                if over && ready.len() > 1 {
                    ready.pop_back()
                } else {
                    None
                }
            }) else {
                return;
            };
            self.inner
                .used
                .set(self.inner.used.get().saturating_sub(decoded.bytes()));
            if let Some(slot) = self.inner.slots.borrow_mut().remove(&key) {
                slot.set(PictureState::Idle);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn png_file(dir: &Path, name: &str, width: u32, height: u32) -> PathBuf {
        let icon = teksilo::canvas::RasterIcon::from_raw(
            vec![200; (width * height * 4) as usize],
            width,
            height,
        );
        let bytes = crate::shared::images::encode_png(&icon).unwrap_or_default();
        let path = dir.join(name);
        std::fs::write(&path, bytes).unwrap_or_default();
        path
    }

    fn key(edge: u32) -> PictureKey {
        PictureKey {
            hash: HASH.to_string(),
            edge,
        }
    }

    #[test]
    fn only_a_project_picture_reference_has_a_hash() {
        assert_eq!(asset_hash(&format!("assets/{HASH}.png")), Some(HASH));
        assert_eq!(asset_hash(&format!("{HASH}.png")), None);
        assert_eq!(asset_hash("assets/../../etc/passwd.png"), None);
        assert_eq!(asset_hash(&format!("assets/{HASH}")), None);
        assert_eq!(asset_hash("https://example.org/picture.png"), None);
        assert_eq!(asset_hash(&format!("assets/{HASH}.p/g")), None);
        let dir = Path::new("/media");
        assert_eq!(
            asset_path(dir, &format!("assets/{HASH}.jpg")),
            Some(dir.join(format!("{HASH}.jpg")))
        );
        assert_eq!(asset_path(dir, "assets/evil/../x.png"), None);
    }

    /// A resource's recorded hash comes from the project's files: one that is not a
    /// content hash is decoded from, but never names a file in the cache, which a
    /// crafted `../..` would otherwise climb out of.
    #[test]
    fn a_hash_that_is_not_a_content_hash_never_names_a_cache_file() {
        assert!(is_content_hash(HASH));
        assert!(is_content_hash(&HASH.to_uppercase()));
        assert!(!is_content_hash(&HASH[1..]));
        assert!(!is_content_hash(&format!("{}/", &HASH[1..])));
        assert!(!is_content_hash("../../home/someone/Pictures/holiday"));
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let source = png_file(tmp.path(), "real.png", 40, 40);
        let disk = tmp.path().join("a").join("b").join("cache");
        let crafted = PictureKey {
            hash: "../../escaped".into(),
            edge: 10,
        };
        let decoded = load(&source, &crafted, Some(&disk)).unwrap_or_else(|| panic!("decoded"));
        assert_eq!((decoded.width, decoded.height), (10, 10));
        assert!(!tmp.path().join("a").join("escaped-10.png").exists());
        assert!(
            !disk.exists(),
            "nothing was written for it, not even the folder"
        );
        let icon = teksilo::canvas::RasterIcon::from_raw(vec![0; 4], 1, 1);
        write_thumbnail(&disk.join("../../escaped-10.png"), &icon);
        assert!(!tmp.path().join("a").join("escaped-10.png").exists());
    }

    /// A file with no content hash recorded (a linked file, a Scrivener bookmark) is
    /// keyed by its place, length and time, which change with it, under a key that has
    /// a content hash's shape and is not its content hash.
    #[test]
    fn a_file_with_no_recorded_hash_is_keyed_by_its_place_and_its_changes() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let source = png_file(tmp.path(), "linked.png", 8, 8);
        assert_eq!(file_key_hash(&source, HASH).as_deref(), Some(HASH));
        let key = file_key_hash(&source, "").unwrap_or_else(|| panic!("a key"));
        assert!(is_content_hash(&key), "{key}");
        let bytes = std::fs::read(&source).unwrap_or_default();
        assert_ne!(key, blake3::hash(&bytes).to_hex().to_string());
        assert_eq!(file_key_hash(&source, "").as_deref(), Some(key.as_str()));
        assert_eq!(
            file_key_hash(&source, "../not-a-hash").as_deref(),
            Some(key.as_str()),
            "a recorded hash of the wrong shape counts as none"
        );
        let other = png_file(tmp.path(), "other.png", 8, 8);
        assert_ne!(
            file_key_hash(&other, ""),
            Some(key.clone()),
            "another place"
        );
        png_file(tmp.path(), "linked.png", 16, 8);
        assert_ne!(
            file_key_hash(&source, ""),
            Some(key),
            "the file changed, so its thumbnail is another"
        );
        assert_eq!(file_key_hash(&tmp.path().join("gone.png"), ""), None);
    }

    #[test]
    fn a_thumbnail_keeps_the_proportions_and_is_cached_on_disk() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let source = png_file(tmp.path(), &format!("{HASH}.png"), 400, 100);
        let disk = tmp.path().join("cache");
        let small = load(&source, &key(100), Some(&disk));
        let small = small.unwrap_or_else(|| panic!("decoded"));
        assert_eq!((small.width, small.height), (100, 25));
        let cached = disk.join(format!("{HASH}-100.png"));
        assert!(cached.exists(), "written beside its peers");
        // The second read comes from the cache, even with the source gone.
        std::fs::remove_file(&source).unwrap_or_default();
        let again = load(&source, &key(100), Some(&disk)).unwrap_or_else(|| panic!("cached"));
        assert_eq!((again.width, again.height), (100, 25));
    }

    #[test]
    fn a_picture_within_the_edge_is_neither_enlarged_nor_cached() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let source = png_file(tmp.path(), "small.png", 40, 30);
        let disk = tmp.path().join("cache");
        let decoded = load(&source, &key(100), Some(&disk)).unwrap_or_else(|| panic!("decoded"));
        assert_eq!((decoded.width, decoded.height), (40, 30));
        assert!(!disk.exists());
        let full = load(&source, &key(0), Some(&disk)).unwrap_or_else(|| panic!("decoded"));
        assert_eq!((full.width, full.height), (40, 30));
    }

    #[test]
    fn a_missing_or_hostile_file_fails_without_decoding() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        assert!(load(&tmp.path().join("gone.png"), &key(64), None).is_none());
        let bogus = tmp.path().join("bogus.png");
        std::fs::write(&bogus, b"not a picture").unwrap_or_default();
        assert!(load(&bogus, &key(64), None).is_none());
        // A cached thumbnail larger than its name says is not trusted.
        let disk = tmp.path().join("cache");
        std::fs::create_dir_all(&disk).unwrap_or_default();
        png_file(&disk, &format!("{HASH}-10.png"), 50, 50);
        let source = png_file(tmp.path(), "real.png", 20, 20);
        let decoded = load(&source, &key(10), Some(&disk)).unwrap_or_else(|| panic!("decoded"));
        assert_eq!((decoded.width, decoded.height), (10, 10));
    }

    #[test]
    fn a_request_without_a_runtime_lands_ready_and_is_served_from_memory() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let source = png_file(tmp.path(), "a.png", 8, 8);
        let cache = PictureCache::new(1 << 20, None);
        let slot = cache.request(None, source.clone(), key(0));
        let PictureState::Ready(first) = slot.get() else {
            panic!("ready");
        };
        std::fs::remove_file(&source).unwrap_or_default();
        let again = cache.request(None, source, key(0));
        let PictureState::Ready(second) = again.get() else {
            panic!("still held");
        };
        assert!(Rc::ptr_eq(&first, &second));
    }

    #[test]
    fn a_failed_picture_says_so() {
        let cache = PictureCache::new(1 << 20, None);
        let slot = cache.request(None, PathBuf::from("/nowhere/x.png"), key(0));
        assert_eq!(slot.get(), PictureState::Failed);
    }

    #[test]
    fn memory_is_kept_within_the_budget_newest_first() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        // Each 16 × 16 picture is 1 KiB of pixels; the budget holds two.
        let cache = PictureCache::new(2048, None);
        let mut slots = Vec::new();
        for i in 0..3u8 {
            let path = png_file(tmp.path(), &format!("{i}.png"), 16, 16);
            let k = PictureKey {
                hash: format!("{i:064}"),
                edge: 0,
            };
            slots.push(cache.request(None, path, k));
        }
        assert!(cache.used() <= 2048, "used {}", cache.used());
        assert_eq!(slots[0].get(), PictureState::Idle, "the oldest went first");
        assert!(matches!(slots[2].get(), PictureState::Ready(_)));
    }
}
