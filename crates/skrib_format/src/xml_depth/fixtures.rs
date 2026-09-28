// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A global allocator that remembers the most heap the process held at once, for
//! the tests that must show a hostile file was refused without being expanded.
//!
//! An entity bomb is a small file whose references expand to far more text than
//! it holds. Asserting only that the import failed would pass just as well if the
//! parser had expanded all of it first and failed afterwards, which is the whole
//! harm. So a test installs [`PeakAllocator`] as its binary's global allocator and
//! runs the import inside [`peak_growth`], which reports how far the heap rose
//! above where it started: a refusal made on the text stays far below the
//! expansion, and an expansion cannot.
//!
//! The allocator also refuses any allocation that would take the heap past
//! [`CEILING`]. A test that regresses then aborts its own binary with "memory
//! allocation failed" instead of taking the machine it runs on down with it.
//!
//! Behind the `hostile-fixtures` feature, turned on only by the dev-dependencies
//! of the crates whose readers allow a DTD.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

/// The most heap a test binary using [`PeakAllocator`] may hold at once.
pub const CEILING: usize = 2 << 30;

/// Heap held right now, in bytes.
static LIVE: AtomicUsize = AtomicUsize::new(0);
/// The most heap held at once since [`peak_growth`] last started measuring.
static PEAK: AtomicUsize = AtomicUsize::new(0);
/// How many allocations were ever made through [`PeakAllocator`], so a measure
/// can tell whether it is installed at all.
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

/// Serialises measurements: the heap is shared by every thread of the binary, so
/// two measured imports running at once would each see the other's allocations.
static MEASURING: Mutex<()> = Mutex::new(());

/// The system allocator, counting what it hands out.
///
/// Install it with `#[global_allocator] static A: PeakAllocator = PeakAllocator;`
/// in a test file holding nothing but measured tests, since every allocation of
/// the binary, whichever thread makes it, counts toward the same peak.
pub struct PeakAllocator;

/// Count `bytes` more as held, or refuse them if that would pass [`CEILING`].
fn reserve(bytes: usize) -> bool {
    let now = LIVE
        .fetch_add(bytes, Ordering::Relaxed)
        .saturating_add(bytes);
    if now > CEILING {
        LIVE.fetch_sub(bytes, Ordering::Relaxed);
        return false;
    }
    PEAK.fetch_max(now, Ordering::Relaxed);
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    true
}

fn release(bytes: usize) {
    LIVE.fetch_sub(bytes, Ordering::Relaxed);
}

// SAFETY: every method hands the request to `System` unchanged and only counts
// around it, so each upholds exactly the contract `System` does. A refusal
// returns null, which `GlobalAlloc` allows for any allocation that cannot be met.
unsafe impl GlobalAlloc for PeakAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !reserve(layout.size()) {
            return std::ptr::null_mut();
        }
        // SAFETY: the caller's guarantees for `layout` are passed on as they are.
        let ptr = unsafe { System.alloc(layout) };
        if ptr.is_null() {
            release(layout.size());
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if !reserve(layout.size()) {
            return std::ptr::null_mut();
        }
        // SAFETY: as for `alloc`.
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if ptr.is_null() {
            release(layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` came from this allocator, which is `System`, with `layout`.
        unsafe { System.dealloc(ptr, layout) };
        release(layout.size());
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        if new_size > old_size && !reserve(new_size - old_size) {
            return std::ptr::null_mut();
        }
        // SAFETY: `ptr` came from `System` with `layout`, and the caller vouches
        // for `new_size`.
        let moved = unsafe { System.realloc(ptr, layout, new_size) };
        if moved.is_null() {
            // The old block is still held, at its old size.
            if new_size > old_size {
                release(new_size - old_size);
            }
        } else if new_size < old_size {
            release(old_size - new_size);
        }
        moved
    }
}

/// Run `work` and return its result with the most the heap rose above what was
/// held when it started, in bytes.
///
/// `None` in place of the growth when [`PeakAllocator`] is not the binary's
/// global allocator: nothing was counted, and a growth of zero would pass every
/// bound a test could set.
pub fn peak_growth<T>(work: impl FnOnce() -> T) -> (T, Option<usize>) {
    let _one_at_a_time = MEASURING.lock().unwrap_or_else(PoisonError::into_inner);
    let allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let before = LIVE.load(Ordering::Relaxed);
    PEAK.store(before, Ordering::Relaxed);
    let value = work();
    let peak = PEAK.load(Ordering::Relaxed);
    let counted = ALLOCATIONS.load(Ordering::Relaxed) > allocations;
    (value, counted.then(|| peak.saturating_sub(before)))
}

/// One mebibyte.
pub const MIB: usize = 1 << 20;
