// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Heap instrumentation for the memory investigation. Compiled out unless the
//! `memprof` feature is on.
//!
//! Two questions the operating system's RSS number cannot answer on its own:
//!
//! 1. **Is the process still holding the bytes, or has it freed them and only the
//!    allocator is sitting on the pages?** RSS conflates the two. A counting
//!    allocator answers it exactly: `live` is the sum of every allocation minus
//!    every deallocation the program itself made, so `rss - live` is allocator
//!    overhead and fragmentation, and nothing else.
//! 2. **Where did the retained bytes come from?** That needs call sites, which a
//!    counter cannot give. See the `dhat-heap` feature beside this one.
//!
//! The sampler writes one CSV row every [`SAMPLE_MS`] to the path in
//! `SKRIBISTO_MEMPROF` (default `/tmp/skribisto-memprof.csv`), so a scenario can be
//! walked by hand and read back afterwards. A companion `<path>.marker` file, if
//! present, has its first line copied into the row's `label` column. That is how a
//! scenario step ("book tab open") is stamped into the timeline without any UI
//! plumbing. Writing `trim` into that marker file additionally calls
//! `malloc_trim(0)` once, which is the decisive experiment: if RSS falls, the bytes
//! were fragmentation; if it does not, they are live.

use std::alloc::{GlobalAlloc, Layout};
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// The allocator the counters sit in front of.
///
/// Selectable so one timeline can be read against two allocators without
/// changing anything else about the run, which is the only honest way to
/// answer how much of the RSS that never comes back is glibc's arena retention
/// rather than live data.
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc as Backing;
#[cfg(not(feature = "mimalloc"))]
use std::alloc::System as Backing;

#[cfg(feature = "mimalloc")]
const BACKING: Backing = Backing;
#[cfg(not(feature = "mimalloc"))]
const BACKING: Backing = Backing;

/// Live bytes, as the program sees them: allocated minus deallocated.
static LIVE: AtomicUsize = AtomicUsize::new(0);
/// High-water mark of [`LIVE`].
static PEAK: AtomicUsize = AtomicUsize::new(0);
/// Cumulative allocation count, so a churn-heavy frame loop is distinguishable
/// from a growing one.
static ALLOCS: AtomicU64 = AtomicU64::new(0);
/// Cumulative deallocation count.
static FREES: AtomicU64 = AtomicU64::new(0);
/// Cumulative bytes ever handed out (never decreases).
static TOTAL_BYTES: AtomicU64 = AtomicU64::new(0);

/// Live bytes per power-of-two size class, `SIZE_CLASSES[n]` covering
/// `[2^(n-1), 2^n)` — the index is the number of significant bits, so class 0 is
/// the empty allocation alone and class 1 is a single byte. The last class holds
/// everything at or above 2^30, which is where the clamp lands.
const SIZE_CLASS_COUNT: usize = 32;
static SIZE_CLASSES: [AtomicUsize; SIZE_CLASS_COUNT] =
    [const { AtomicUsize::new(0) }; SIZE_CLASS_COUNT];

fn size_class(bytes: usize) -> usize {
    // `leading_zeros` on 0 is the full width, which would index past the end;
    // a zero-sized allocation cannot reach here (the allocator rejects it), but
    // the clamp costs one instruction and removes the question.
    let bits = usize::BITS - bytes.leading_zeros();
    (bits as usize).min(SIZE_CLASS_COUNT - 1)
}

fn note_alloc(bytes: usize) {
    let live = LIVE.fetch_add(bytes, Ordering::Relaxed) + bytes;
    ALLOCS.fetch_add(1, Ordering::Relaxed);
    TOTAL_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
    SIZE_CLASSES[size_class(bytes)].fetch_add(bytes, Ordering::Relaxed);
    // A compare-exchange loop, not a `fetch_max`, because the read side wants the
    // peak to be a value `LIVE` genuinely held rather than a race artefact.
    let mut peak = PEAK.load(Ordering::Relaxed);
    while live > peak {
        match PEAK.compare_exchange_weak(peak, live, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(seen) => peak = seen,
        }
    }
}

/// Record a stack for an allocation big enough to be worth one.
fn note_large_alloc(ptr: *mut u8, bytes: usize) {
    let threshold = large_threshold();
    if threshold != 0 && bytes >= threshold {
        record_large(ptr, bytes);
    }
}

/// Drop a large allocation's accounting. Gated on the same threshold so a
/// freed small block never takes the lock.
fn note_large_free(ptr: *mut u8, bytes: usize) {
    let threshold = large_threshold();
    if threshold != 0 && bytes >= threshold {
        forget_large(ptr);
    }
}

fn note_free(bytes: usize) {
    LIVE.fetch_sub(bytes, Ordering::Relaxed);
    FREES.fetch_add(1, Ordering::Relaxed);
    SIZE_CLASSES[size_class(bytes)].fetch_sub(bytes, Ordering::Relaxed);
}

// ── Large-allocation attribution ─────────────────────────────────────────────
//
// A counter says *how much* is retained. It cannot say *by what*, and that is
// the question a leak actually poses. Full per-allocation profiling (dhat) can
// answer it but costs a backtrace on every `malloc`, which in a debug GUI build
// is the difference between a scenario that runs in three minutes and one that
// runs in forty.
//
// The compromise below is exact where it matters and free where it does not:
// every allocation at or above [`large_threshold`] records the stack that made
// it, and everything smaller is only counted. Retained memory of the kind this
// investigation is about (a document's shaped glyphs, a copy of a glyph atlas,
// a laid-out block table) arrives in allocations far above any sensible
// threshold, while the millions of small allocations that would dominate the
// profiling cost carry none of the retained bytes.
//
// Two rules make it safe to run inside the allocator itself:
//
// * **Re-entrancy.** Capturing a stack allocates. A thread-local flag routes
//   every allocation made *by* the profiler straight to the backing allocator,
//   untracked, so the profiler cannot recurse into itself.
// * **Capture, don't symbolise.** `backtrace::trace` walks return addresses and
//   allocates nothing per frame beyond a small array; turning those addresses
//   into function names and line numbers is deferred to the dump, where it
//   happens once per distinct site instead of once per allocation.

/// Frames kept per captured stack. Deep enough to cross the allocator, `Vec`'s
/// growth path and the collection wrapper and still land on application code.
const STACK_DEPTH: usize = 24;
/// Frames skipped from the top: this module's own `alloc`/`realloc` and the
/// `GlobalAlloc` shim under them.
const STACK_SKIP: usize = 3;

/// One captured allocation site.
struct Site {
    /// Return addresses, innermost first. Symbolised only at dump time.
    frames: Vec<usize>,
    /// Bytes currently live from this site.
    live: usize,
    /// Bytes ever allocated from this site.
    total: u64,
    /// Live allocation count.
    live_blocks: usize,
    /// High-water mark of `live`.
    peak: usize,
}

#[derive(Default)]
struct Sites {
    /// Site key (a hash of the frame list) to its accounting.
    by_key: HashMap<u64, Site>,
    /// Live pointer to `(size, site key)`, so a free can find what to subtract.
    live_ptrs: HashMap<usize, (usize, u64)>,
}

/// The attribution table. A plain `Mutex`: contention is limited to
/// allocations above the threshold, which are rare by construction.
static SITES: Mutex<Option<Sites>> = Mutex::new(None);

thread_local! {
    /// True while this thread is inside the profiler, so its own allocations
    /// take the untracked path.
    static IN_PROFILER: Cell<bool> = const { Cell::new(false) };
}

/// Allocations at or above this many bytes get a stack. `0` disables capture.
///
/// Read once from `SKRIBISTO_MEMPROF_LARGE` (default 64 KiB), a size no
/// ordinary UI allocation reaches and every retained buffer this investigation
/// cares about exceeds.
fn large_threshold() -> usize {
    static CACHED: AtomicUsize = AtomicUsize::new(usize::MAX);
    let cached = CACHED.load(Ordering::Relaxed);
    if cached != usize::MAX {
        return cached;
    }
    // Resolving the variable ALLOCATES, and this runs inside the allocator. Without
    // the re-entrancy guard the nested allocation calls straight back in here, finds
    // the sentinel still in place and reads the variable again, for as many frames
    // as the stack holds: setting `SKRIBISTO_MEMPROF_LARGE` at all took the process
    // down with a stack overflow before its first line of `main`. Leaving it unset
    // happened to be safe only because `env::var_os` allocates nothing for a name
    // that is not there, which is why the default path never showed the bug.
    //
    // A nested call answers `0` — counted by the plain counters, not attributed —
    // which costs one allocation's stack and ends the recursion.
    let mut resolved = None;
    without_reentry(|| {
        let v = std::env::var("SKRIBISTO_MEMPROF_LARGE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            // `usize::MAX` is the "not resolved yet" sentinel, so it cannot also be
            // a legal threshold; one byte below it disables capture just as well.
            .map(|v| v.min(usize::MAX - 1))
            .unwrap_or(64 * 1024);
        CACHED.store(v, Ordering::Relaxed);
        resolved = Some(v);
    });
    resolved.unwrap_or(0)
}

/// Run `f` with the re-entrancy guard held, or do nothing if it already is.
///
/// `try_with`, never `with`. A thread still allocates while its thread-locals are
/// being destroyed, and `with` on a destroyed slot panics, inside the global
/// allocator, on a dying thread, which is an abort rather than a diagnosable
/// error. Treating "no guard available" as "already inside" is the safe reading:
/// the allocation goes uncounted for attribution (the plain counters still see
/// it) instead of taking the process down.
fn without_reentry(f: impl FnOnce()) {
    let entered = IN_PROFILER
        .try_with(|flag| {
            if flag.get() {
                false
            } else {
                flag.set(true);
                true
            }
        })
        .unwrap_or(false);
    if !entered {
        return;
    }
    f();
    let _ = IN_PROFILER.try_with(|flag| flag.set(false));
}

fn capture_frames() -> Vec<usize> {
    let mut frames = Vec::with_capacity(STACK_DEPTH);
    let mut skipped = 0usize;
    backtrace::trace(|frame| {
        if skipped < STACK_SKIP {
            skipped += 1;
            return true;
        }
        frames.push(frame.ip() as usize);
        frames.len() < STACK_DEPTH
    });
    frames
}

fn key_of(frames: &[usize]) -> u64 {
    // FNV-1a over the return addresses: distinct stacks collide only by
    // accident, and a collision merges two sites in a report rather than
    // corrupting the accounting.
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for f in frames {
        for b in f.to_ne_bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x1000_0000_01b3);
        }
    }
    hash
}

fn record_large(ptr: *mut u8, size: usize) {
    without_reentry(|| {
        let frames = capture_frames();
        let key = key_of(&frames);
        let Ok(mut guard) = SITES.lock() else {
            return;
        };
        let sites = guard.get_or_insert_with(Sites::default);
        let site = sites.by_key.entry(key).or_insert_with(|| Site {
            frames,
            live: 0,
            total: 0,
            live_blocks: 0,
            peak: 0,
        });
        site.live += size;
        site.total += size as u64;
        site.live_blocks += 1;
        site.peak = site.peak.max(site.live);
        sites.live_ptrs.insert(ptr as usize, (size, key));
    });
}

fn forget_large(ptr: *mut u8) {
    without_reentry(|| {
        let Ok(mut guard) = SITES.lock() else {
            return;
        };
        let Some(sites) = guard.as_mut() else {
            return;
        };
        let Some((size, key)) = sites.live_ptrs.remove(&(ptr as usize)) else {
            return;
        };
        if let Some(site) = sites.by_key.get_mut(&key) {
            site.live = site.live.saturating_sub(size);
            site.live_blocks = site.live_blocks.saturating_sub(1);
        }
    });
}

/// Write the attribution report to `path`, ranked by bytes currently live.
///
/// Symbolisation happens here and only here: once per distinct site, not once
/// per allocation, which is what makes the capture affordable in the first place.
pub fn dump_sites(path: &std::path::Path) -> std::io::Result<()> {
    // Under the re-entrancy guard for its whole length. Symbolising allocates
    // freely, and without this the report's own working set is recorded as if it
    // were the application's, and then, because the frees happen inside a later
    // profiler call, never un-recorded. The first run of this function without
    // the guard put a 24 MB phantom at the top of the second run's ranking.
    let entered = IN_PROFILER
        .try_with(|flag| {
            let was = flag.get();
            flag.set(true);
            !was
        })
        .unwrap_or(false);
    let out = dump_sites_inner(path);
    if entered {
        let _ = IN_PROFILER.try_with(|flag| flag.set(false));
    }
    out
}

fn dump_sites_inner(path: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;

    // Snapshot under the lock, then release it: symbolising walks the binary's
    // debug info and allocates freely, and doing that while holding the
    // allocator's own table invites a deadlock.
    let snapshot: Vec<(u64, Vec<usize>, usize, u64, usize, usize)> = {
        let Ok(guard) = SITES.lock() else {
            return Ok(());
        };
        match guard.as_ref() {
            None => Vec::new(),
            Some(sites) => sites
                .by_key
                .iter()
                .map(|(k, s)| (*k, s.frames.clone(), s.live, s.total, s.live_blocks, s.peak))
                .collect(),
        }
    };

    let mut ranked = snapshot;
    ranked.sort_by_key(|a| std::cmp::Reverse(a.2));
    let totals = self::snapshot();

    let mut out = std::fs::File::create(path)?;
    let s = totals;
    writeln!(
        out,
        "# memprof large-allocation sites (threshold {} bytes)",
        large_threshold()
    )?;
    writeln!(
        out,
        "# live overall {:.1} MB in {} allocations; {} sites recorded",
        s.live as f64 / (1 << 20) as f64,
        s.allocs - s.frees,
        ranked.len()
    )?;
    let attributed: usize = ranked.iter().map(|r| r.2).sum();
    writeln!(
        out,
        "# attributed to large allocations: {:.1} MB ({:.0}% of live)",
        attributed as f64 / (1 << 20) as f64,
        if s.live > 0 {
            attributed as f64 * 100.0 / s.live as f64
        } else {
            0.0
        }
    )?;

    // The rest of `live` sits in allocations below the threshold, which carry no
    // stack. The histogram is what can still be said about them, and it says
    // enough to tell two very different stories apart: a few huge buffers, or a
    // great many small ones. Shaped-glyph vectors are per-run and land in the
    // 1-32 KiB classes; an atlas copy or a widget arena lands above 1 MiB.
    writeln!(out, "#\n# live bytes by allocation size class:")?;
    let classes = size_classes();
    for (i, bytes) in classes.iter().enumerate() {
        if *bytes == 0 {
            continue;
        }
        let lo = if i == 0 { 0 } else { 1usize << (i - 1) };
        let hi = 1usize << i;
        writeln!(
            out,
            "#   {:>10} .. {:<10} {:>9.2} MB  ({:>4.1}%)",
            lo,
            hi,
            *bytes as f64 / (1 << 20) as f64,
            if s.live > 0 {
                *bytes as f64 * 100.0 / s.live as f64
            } else {
                0.0
            }
        )?;
    }
    for (_, frames, live, total, blocks, peak) in ranked.iter().take(60) {
        if *live == 0 {
            continue;
        }
        writeln!(
            out,
            "
{:.2} MB live in {} blocks (peak {:.2} MB, {:.1} MB ever allocated)",
            *live as f64 / (1 << 20) as f64,
            blocks,
            *peak as f64 / (1 << 20) as f64,
            *total as f64 / (1 << 20) as f64
        )?;
        for ip in frames {
            let mut printed = false;
            backtrace::resolve(*ip as *mut std::ffi::c_void, |sym| {
                if printed {
                    return;
                }
                printed = true;
                let name = sym
                    .name()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "<unknown>".into());
                match (sym.filename(), sym.lineno()) {
                    (Some(f), Some(l)) => {
                        let _ = writeln!(out, "    {name} ({}:{l})", f.display());
                    }
                    _ => {
                        let _ = writeln!(out, "    {name}");
                    }
                }
            });
            if !printed {
                let _ = writeln!(out, "    <{ip:#x}>");
            }
        }
    }
    out.flush()
}

/// The backing allocator with a counter in front of it.
///
/// Deliberately not a sampling profiler: every allocation is counted, because the
/// question is a *total* (how many bytes are live) and a sample cannot answer a
/// total. The cost is two relaxed atomics per allocation, which is measurable in a
/// microbenchmark and invisible next to a frame of layout.
pub struct TrackingAlloc;

// SAFETY: every method forwards to `BACKING`, which is a correct allocator, and the
// bookkeeping around each call touches only atomics. It allocates nothing itself,
// so it cannot re-enter.
unsafe impl GlobalAlloc for TrackingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { BACKING.alloc(layout) };
        if !ptr.is_null() {
            note_alloc(layout.size());
            note_large_alloc(ptr, layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        note_free(layout.size());
        note_large_free(ptr, layout.size());
        unsafe { BACKING.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { BACKING.alloc_zeroed(layout) };
        if !ptr.is_null() {
            note_alloc(layout.size());
            note_large_alloc(ptr, layout.size());
        }
        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // The free is noted *before* the call, while `ptr` is still the address
        // the table knows; afterwards it may have been reused by another thread.
        note_large_free(ptr, layout.size());
        let out = unsafe { BACKING.realloc(ptr, layout, new_size) };
        if !out.is_null() {
            note_free(layout.size());
            note_alloc(new_size);
            note_large_alloc(out, new_size);
        } else {
            // The realloc failed, so `ptr` is untouched and the caller still owns
            // it. Undo the un-attribution above, or the block goes missing from
            // the report while its bytes stay in `live` — the report would then
            // read as unattributed fragmentation.
            note_large_alloc(ptr, layout.size());
        }
        out
    }
}

/// One row of the timeline.
#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    /// Bytes the program itself is holding.
    pub live: usize,
    /// High-water mark of `live` since process start.
    pub peak: usize,
    /// Cumulative allocations and deallocations.
    pub allocs: u64,
    pub frees: u64,
    /// Cumulative bytes ever handed out.
    pub total_bytes: u64,
    /// Resident set size, from the kernel.
    pub rss: usize,
    /// Virtual size, from the kernel.
    pub vsz: usize,
    /// The anonymous half of RSS: heap, stacks, and anonymous mmaps.
    ///
    /// The half that matters. RSS also counts file-backed pages, and an
    /// unoptimised debug binary is hundreds of megabytes of those: reading RSS
    /// alone makes a build flag look like a leak.
    pub rss_anon: usize,
    /// The file-backed half: the binary's own text and rodata, mapped
    /// libraries, mapped fonts and dictionaries.
    pub rss_file: usize,
}

impl Snapshot {
    /// What the allocator is holding that the program has already given back:
    /// free-list residue, arena retention and fragmentation.
    ///
    /// Measured against **anonymous** RSS, not RSS: file-backed pages are the
    /// binary and the mapped assets, which no allocator ever touched. Saturating,
    /// because a page allocated but never written is not resident.
    pub fn allocator_overhead(&self) -> usize {
        self.rss_anon.saturating_sub(self.live)
    }
}

/// Read the counters and the kernel's own numbers together.
pub fn snapshot() -> Snapshot {
    let (rss, vsz) = read_statm();
    let (rss_anon, rss_file) = read_smaps_rollup();
    Snapshot {
        live: LIVE.load(Ordering::Relaxed),
        peak: PEAK.load(Ordering::Relaxed),
        allocs: ALLOCS.load(Ordering::Relaxed),
        frees: FREES.load(Ordering::Relaxed),
        total_bytes: TOTAL_BYTES.load(Ordering::Relaxed),
        rss,
        vsz,
        rss_anon,
        rss_file,
    }
}

/// `(anonymous, file-backed)` resident bytes, from `/proc/self/smaps_rollup`.
///
/// One read of one small file, which is why the sampler can afford it twice a
/// second: the rollup is summed by the kernel, unlike `/proc/self/smaps`, which
/// would mean parsing a few thousand mapping records per tick.
#[cfg(target_os = "linux")]
fn read_smaps_rollup() -> (usize, usize) {
    let Ok(text) = std::fs::read_to_string("/proc/self/smaps_rollup") else {
        return (0, 0);
    };
    let mut anon = 0usize;
    let mut rss = 0usize;
    for line in text.lines() {
        let Some((key, rest)) = line.split_once(':') else {
            continue;
        };
        // Every value in this file is in kB, whatever the page size.
        let kb = rest
            .split_whitespace()
            .next()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        match key {
            // Two spellings across kernel versions, and only one of them is
            // present at a time: 6.x publishes `Anonymous`, and the split
            // `Rss_Anon`/`Rss_File` pair appears on kernels that carry the
            // per-type breakdown. Reading only one name silently reports zero
            // anonymous bytes on the other kernel, which reads as "nothing on
            // the heap" rather than as "this parse is wrong".
            "Anonymous" | "Rss_Anon" => anon = kb * 1024,
            "Rss" => rss = kb * 1024,
            _ => {}
        }
    }
    (anon, rss.saturating_sub(anon))
}

#[cfg(not(target_os = "linux"))]
fn read_smaps_rollup() -> (usize, usize) {
    (0, 0)
}

/// Live bytes per power-of-two size class, smallest first.
pub fn size_classes() -> [usize; SIZE_CLASS_COUNT] {
    std::array::from_fn(|i| SIZE_CLASSES[i].load(Ordering::Relaxed))
}

/// `(rss, vsz)` in bytes, or `(0, 0)` where `/proc` is not available.
///
/// Linux-only by construction: `/proc/self/statm` is where the kernel publishes
/// the one number this whole investigation is about. Elsewhere the counters below
/// still work and only the `rss`/`vsz` columns read zero.
#[cfg(target_os = "linux")]
fn read_statm() -> (usize, usize) {
    let page = page_size();
    let Ok(text) = std::fs::read_to_string("/proc/self/statm") else {
        return (0, 0);
    };
    let mut it = text.split_whitespace();
    let vsz = it.next().and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
    let rss = it.next().and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
    (rss * page, vsz * page)
}

#[cfg(not(target_os = "linux"))]
fn read_statm() -> (usize, usize) {
    (0, 0)
}

#[cfg(target_os = "linux")]
fn page_size() -> usize {
    // SAFETY: `sysconf` with a valid name is always safe; a negative answer means
    // "no limit / unknown", which the fallback covers.
    let raw = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if raw > 0 { raw as usize } else { 4096 }
}

#[cfg(not(target_os = "linux"))]
fn page_size() -> usize {
    4096
}

/// Ask glibc to return free pages at the top of every arena to the kernel.
///
/// The decisive experiment for "is this a leak or is it fragmentation": call it
/// after a teardown and compare RSS. Returns whether glibc reported having
/// released anything.
pub fn trim() -> bool {
    // mimalloc has no `malloc_trim`: it returns pages on its own decay schedule,
    // which is the whole reason to compare the two. Reporting `false` is honest,
    // since nothing was asked to be released, and it keeps the CSV column meaning one
    // thing.
    #[cfg(feature = "mimalloc")]
    {
        false
    }
    #[cfg(all(not(feature = "mimalloc"), target_os = "linux", target_env = "gnu"))]
    {
        // SAFETY: `malloc_trim` takes a pad in bytes and is safe to call at any
        // time from any thread; it only walks the allocator's own free lists.
        unsafe { libc::malloc_trim(0) != 0 }
    }
    #[cfg(not(any(feature = "mimalloc", all(target_os = "linux", target_env = "gnu"))))]
    {
        false
    }
}

/// How often the sampler writes a row.
const SAMPLE_MS: u64 = 500;

/// Where the timeline goes, and where the marker file is looked for.
fn output_path() -> std::path::PathBuf {
    std::env::var_os("SKRIBISTO_MEMPROF")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp/skribisto-memprof.csv"))
}

/// Start the sampler thread. Idempotent: a second call is a no-op.
///
/// Called from `run()` before anything else allocates in earnest, so the first row
/// is a genuine baseline.
pub fn start_sampler() {
    use std::io::Write;
    use std::sync::atomic::AtomicBool;

    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    let path = output_path();
    let marker_path = {
        let mut p = path.clone().into_os_string();
        p.push(".marker");
        std::path::PathBuf::from(p)
    };
    // Truncate rather than append: a run's timeline is only meaningful against its
    // own baseline, and two runs concatenated read as one impossible sawtooth.
    let mut file = match std::fs::File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("memprof: cannot write {}: {e}", path.display());
            return;
        }
    };
    let _ = std::fs::remove_file(&marker_path);
    let _ = writeln!(
        file,
        "t_ms,live,peak,rss,rss_anon,rss_file,vsz,overhead,allocs,frees,total_bytes,docs,label"
    );
    let _ = file.flush();

    std::thread::Builder::new()
        .name("memprof".into())
        .spawn(move || {
            let start = std::time::Instant::now();
            let mut last_label = String::new();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(SAMPLE_MS));
                let mut label = std::fs::read_to_string(&marker_path)
                    .unwrap_or_default()
                    .lines()
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .to_owned();
                // `trim` is a command rather than a label: run it once, then report
                // the *post-trim* row so the CSV shows what the release achieved.
                if label == "trim" {
                    let released = trim();
                    label = format!("trim(released={released})");
                    let _ = std::fs::write(&marker_path, "");
                }
                // `sites` is the other command: write the large-allocation
                // attribution report beside the timeline. Answering "50 MB is
                // retained, by what?" is the whole reason the capture exists,
                // and the natural moment to ask is a scenario step, not process
                // exit (a leak is interesting while the app is still running).
                if let Some(tag) = label.strip_prefix("sites:") {
                    let mut out = path.clone().into_os_string();
                    out.push(format!(".sites-{tag}.txt"));
                    let out = std::path::PathBuf::from(out);
                    match dump_sites(&out) {
                        Ok(()) => label = format!("sites({})", out.display()),
                        Err(e) => label = format!("sites-failed({e})"),
                    }
                    let _ = std::fs::write(&marker_path, "");
                }
                if label.is_empty() {
                    label = last_label.clone();
                } else {
                    last_label = label.clone();
                }
                let s = snapshot();
                // How many document bodies are still alive. The one column here
                // that names an *owner* rather than a byte count: a leak the
                // rest of the row can only describe as "the rope and the block
                // table are still resident" is a retained document, and this
                // says how many. An atomic load, so it costs nothing to take on
                // every tick.
                let docs = teksilo::text_document::live_document_count();
                let _ = writeln!(
                    file,
                    "{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    start.elapsed().as_millis(),
                    s.live,
                    s.peak,
                    s.rss,
                    s.rss_anon,
                    s.rss_file,
                    s.vsz,
                    s.allocator_overhead(),
                    s.allocs,
                    s.frees,
                    s.total_bytes,
                    docs,
                    label
                );
                let _ = file.flush();
            }
        })
        .ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_class_buckets_by_power_of_two() {
        assert_eq!(size_class(1), 1);
        assert_eq!(size_class(2), 2);
        assert_eq!(size_class(3), 2);
        assert_eq!(size_class(4), 3);
        assert_eq!(size_class(255), 8);
        assert_eq!(size_class(256), 9);
        // Anything enormous saturates into the last bucket rather than indexing
        // past the end of the table.
        assert_eq!(size_class(usize::MAX), SIZE_CLASS_COUNT - 1);
    }

    #[test]
    fn the_counters_see_a_four_megabyte_vec() {
        let before = snapshot();
        let v: Vec<u8> = vec![0; 4 << 20];
        let during = snapshot();
        drop(v);
        let after = snapshot();

        // Assertions on the MONOTONIC counters only. `live` is shared with every
        // other thread in the process. The test harness runs cases in parallel,
        // and one of them freeing a buffer between two reads can make `live` fall
        // across an allocation this thread just made. `total_bytes`, `allocs` and
        // `frees` never decrease, so a claim about them cannot be raced away.
        assert!(
            during.total_bytes >= before.total_bytes + (4 << 20),
            "a 4 MiB allocation must show up in total_bytes"
        );
        assert!(during.allocs > before.allocs);
        assert!(
            after.frees > during.frees,
            "dropping the vec must count a free"
        );
        // The peak is a high-water mark, so it can only have risen.
        assert!(after.peak >= before.peak);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn snapshot_reports_a_plausible_rss() {
        let s = snapshot();
        assert!(s.rss > 0, "no RSS read from /proc/self/statm");
        assert!(s.vsz >= s.rss);
        assert!(s.peak >= s.live);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn anonymous_and_file_rss_add_up_to_rss() {
        let s = snapshot();
        assert!(
            s.rss_anon > 0,
            "no anonymous RSS in /proc/self/smaps_rollup"
        );
        assert!(
            s.rss_file > 0,
            "a mapped binary always has file-backed pages"
        );
        // The two are sampled from a different file than `rss`, a moment apart, so
        // they agree only approximately; an order of magnitude apart would mean the
        // parse is wrong, which is what this guards.
        let sum = s.rss_anon + s.rss_file;
        assert!(
            sum * 2 > s.rss && sum < s.rss * 2,
            "rss={} anon+file={sum}",
            s.rss
        );
    }

    /// Setting the documented threshold variable must not take the process down.
    ///
    /// Resolving it allocates, and it is resolved from inside the global
    /// allocator; before the re-entrancy guard went around that read, every
    /// nested allocation read it again and the process died of a stack overflow
    /// before `main`. Only reachable from a fresh process, because the answer is
    /// cached in a static after the first call — hence the re-exec of this same
    /// test binary rather than an in-process assertion.
    #[test]
    fn setting_the_threshold_variable_does_not_recurse_into_the_allocator() {
        let Ok(exe) = std::env::current_exe() else {
            return;
        };
        let out = std::process::Command::new(exe)
            .args([
                "memprof::tests::the_counters_see_a_four_megabyte_vec",
                "--exact",
            ])
            .env("SKRIBISTO_MEMPROF_LARGE", "65536")
            .output();
        let Ok(out) = out else {
            return;
        };
        assert!(
            out.status.success(),
            "a child with SKRIBISTO_MEMPROF_LARGE set exited {:?}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn overhead_is_measured_against_anonymous_rss_only() {
        // The distinction this whole module exists to draw: a 400 MB debug binary
        // contributes hundreds of megabytes of *file* RSS, and counting those as
        // allocator overhead is how a build flag gets mistaken for a leak.
        let s = snapshot();
        assert_eq!(s.allocator_overhead(), s.rss_anon.saturating_sub(s.live));
    }
}
