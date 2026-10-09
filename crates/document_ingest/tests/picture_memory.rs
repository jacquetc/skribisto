// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a crafted picture costs in memory, measured.
//!
//! A picture is somebody else's bytes, and a few kilobytes of them can describe a picture of
//! hundreds of megabytes. `document_ingest::pictures` promises a ceiling on what converting
//! one may hold ([`pictures::MAX_DECODE_BYTES`]) and that a picture too large for it is
//! refused before its pixels are allocated. Those are claims about allocation, so this
//! counts every allocation the process makes, and holds each case to what it may peak at.
//!
//! Its own test binary, with one test running the cases in turn: the allocator counts the
//! whole process, and a second test running beside it would be counted too.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use document_ingest::pictures::{self, PictureRefusal};

/// The system allocator, counting what is held now and the most held since the last reset.
struct Counting;

static HELD: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller's contract for `GlobalAlloc::alloc`, passed on unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            let now = HELD.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
            PEAK.fetch_max(now, Ordering::SeqCst);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: as `alloc`.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            let now = HELD.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
            PEAK.fetch_max(now, Ordering::SeqCst);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: as `alloc`.
        unsafe { System.dealloc(pointer, layout) };
        HELD.fetch_sub(layout.size(), Ordering::SeqCst);
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: as `alloc`.
        let moved = unsafe { System.realloc(pointer, layout, new_size) };
        if !moved.is_null() {
            if new_size >= layout.size() {
                let grown = new_size - layout.size();
                let now = HELD.fetch_add(grown, Ordering::SeqCst) + grown;
                // A reallocation that moves holds both blocks for a moment; counting the
                // larger alone is what the process holds once it returns.
                PEAK.fetch_max(now, Ordering::SeqCst);
            } else {
                HELD.fetch_sub(layout.size() - new_size, Ordering::SeqCst);
            }
        }
        moved
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// The most `work` held beyond what was held when it began, and what it returned.
fn peak_of<T>(work: impl FnOnce() -> T) -> (usize, T) {
    let before = HELD.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    let out = work();
    (PEAK.load(Ordering::SeqCst).saturating_sub(before), out)
}

const MIB: usize = 1 << 20;

/// A PackBits TIFF of one flat colour, `bits` bits a channel, four channels, every strip the
/// same bytes: a file of kilobytes that decodes to `width` × `height` pixels.
fn flat_tiff(width: u32, height: u32, bits: u16) -> Vec<u8> {
    let row = width as usize * 4 * (bits as usize / 8);
    assert_eq!(row % 128, 0, "whole PackBits runs");
    let rows_per_strip: u32 = 16;
    let strips = height.div_ceil(rows_per_strip);
    // A run of 128 copies of one byte is the header 0x81 and the byte.
    let strip: Vec<u8> = std::iter::repeat_n([0x81u8, 0x7f], row / 128 * rows_per_strip as usize)
        .flatten()
        .collect();
    let entries: [(u16, u16, u32, u32); 11] = [
        (256, 4, 1, width),
        (257, 4, 1, height),
        (258, 3, 4, 0), // bits per sample, written after the directory
        (259, 3, 1, 32773),
        (262, 3, 1, 2),
        (273, 4, strips, 0), // strip offsets, after it too
        (277, 3, 1, 4),
        (278, 4, 1, rows_per_strip),
        (279, 4, strips, 0), // strip byte counts
        (284, 3, 1, 1),
        (338, 3, 1, 2),
    ];
    let directory_end = 8 + 2 + 12 * entries.len() as u32 + 4;
    let bits_at = directory_end;
    let offsets_at = bits_at + 8;
    let counts_at = offsets_at + 4 * strips;
    let data_at = counts_at + 4 * strips;
    let mut out = b"II*\0".to_vec();
    out.extend_from_slice(&8u32.to_le_bytes());
    out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    for (tag, kind, count, value) in entries {
        let value = match tag {
            258 => bits_at,
            273 => offsets_at,
            279 => counts_at,
            _ => value,
        };
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        if kind == 3 && count == 1 {
            out.extend_from_slice(&(value as u16).to_le_bytes());
            out.extend_from_slice(&[0, 0]);
        } else {
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
    out.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..4 {
        out.extend_from_slice(&bits.to_le_bytes());
    }
    for _ in 0..strips {
        out.extend_from_slice(&data_at.to_le_bytes());
    }
    for _ in 0..strips {
        out.extend_from_slice(&(strip.len() as u32).to_le_bytes());
    }
    out.extend_from_slice(&strip);
    out
}

/// A GIF with a `width` × `height` screen and `frames` frames of one red pixel in its corner.
fn one_pixel_frames(width: u16, height: u16, frames: usize) -> Vec<u8> {
    let mut out = b"GIF89a".to_vec();
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    out.extend_from_slice(&[0, 0, 0]);
    for _ in 0..frames {
        // Image descriptor: at 0,0, one by one, a local colour table of two entries.
        out.push(0x2c);
        out.extend_from_slice(&[0, 0, 0, 0, 1, 0, 1, 0, 0x80]);
        out.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        // LZW code size 2, then clear, pixel 0, end, in one sub-block.
        out.extend_from_slice(&[2, 2, 0x44, 0x01, 0]);
    }
    out.push(0x3b);
    out
}

#[test]
fn a_crafted_picture_stays_under_its_ceilings() {
    // A 64-megapixel TIFF at sixteen bits a channel is half a gigabyte of pixels from 35
    // kilobytes of file. Converting it held a gigabyte: its pixels, and the TIFF decoder's own
    // copy of them, which the ceiling was never handed. It is refused now before either.
    let tiff = flat_tiff(16_384, 3_906, 16);
    assert!(tiff.len() < 64 * 1024);
    let (peak, result) = peak_of(|| pictures::ingest(tiff, "scan.tif"));
    assert_eq!(result.err(), Some(PictureRefusal::TooLarge));
    assert!(
        peak < 16 * MIB,
        "refused before its pixels were allocated, yet held {} MiB",
        peak / MIB
    );

    // A GIF's first frame is decoded and the rest are not. Learning that there was a second
    // frame used to decode it too, and the frame iterator composes each frame onto a canvas
    // of its own: the picture held three times at once for one kept.
    let (width, height) = (2_048u16, 2_048u16);
    let screen = usize::from(width) * usize::from(height) * 4;
    let gif = one_pixel_frames(width, height, 3);
    let (peak, result) = peak_of(|| pictures::ingest(gif, "map.gif"));
    let ingested = result.expect("a GIF of three frames converts");
    assert!(ingested.first_frame_only, "and says it had more than one");
    assert_eq!(
        (ingested.picture.width, ingested.picture.height),
        (2_048, 2_048)
    );
    assert!(
        peak < screen + screen / 2,
        "the first frame alone, {} MiB, yet held {} MiB",
        screen / MIB,
        peak / MIB
    );

    // A picture written as base64 inside an OpenDocument file is counted before it is
    // decoded: past the ceiling, nothing is decoded and nothing is gathered.
    let digits = (pictures::MAX_PICTURE_BYTES as usize / 3 + 1) * 4;
    let parts = std::iter::repeat_n("AAAA", digits / 4);
    let (peak, result) = peak_of(|| pictures::decode_base64_picture(parts));
    assert_eq!(result.err(), Some(PictureRefusal::TooLarge));
    assert!(
        peak < MIB,
        "counted, not decoded, yet held {} MiB",
        peak / MIB
    );
}
