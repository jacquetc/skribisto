// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Archives whose headers or compression ratio a reader must not trust, built for
//! the tests of every reader [`super::ZipGuard`] protects.
//!
//! Two shapes, each in the form a reader meets it:
//!
//! * a **highly compressible member** ([`compressible_zip`]): a bounded run of zero
//!   bytes stored deflated, so the archive is a fraction of what it unpacks to. It is
//!   an honest archive — its headers state its true size — and it exercises the ratio
//!   and total-size checks that run while a member inflates.
//! * an **over-declared member** ([`over_declared_zip`]): a few real bytes whose
//!   central-directory header states a far larger unpacked size, through a zip64 field
//!   once that size does not fit in 32 bits. It exercises the check that runs over the
//!   central directory before any byte is inflated, which is the one that has to fire
//!   before a reader reserves the declared size.
//!
//! Both are produced by writing an ordinary archive and, for the second, rewriting one
//! size field in place, so the `zip` crate opens them exactly as it opens any archive.
//!
//! Behind the `hostile-fixtures` feature, turned on only by the dev-dependencies of the
//! crates that read zips.

use std::io::Write;

use zip::write::SimpleFileOptions;

/// One mebibyte.
pub const MIB: u64 = 1 << 20;

/// A one-member archive whose member `name` unpacks to `uncompressed_len` zero bytes,
/// stored deflated so the archive itself stays small.
///
/// The zeros are fed to the writer a mebibyte at a time, so building even a large
/// unpacked size never holds it all in memory. The headers state the true size: this
/// is a legitimate archive with a high compression ratio, not a malformed one.
pub fn compressible_zip(name: &str, uncompressed_len: u64) -> Vec<u8> {
    let mut buffer = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut buffer);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    writer.start_file(name, options).expect("start member");
    let chunk = vec![0u8; MIB as usize];
    let mut remaining = uncompressed_len;
    while remaining > 0 {
        let take = remaining.min(MIB) as usize;
        writer.write_all(&chunk[..take]).expect("write chunk");
        remaining -= take as u64;
    }
    writer.finish().expect("finish archive");
    buffer.into_inner()
}

/// A one-member archive whose member `name` holds a few real bytes but whose header
/// declares it unpacks to `declared` bytes.
///
/// For a `declared` under 4 GiB the 32-bit size fields carry it; at or above that a
/// zip64 extra field does, forced by writing the member as a large file. The real
/// content is a short string, so the archive is tiny whatever it claims.
pub fn over_declared_zip(name: &str, declared: u64) -> Vec<u8> {
    let real = b"a few honest bytes";
    let large = declared >= u64::from(u32::MAX);
    let mut buffer = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut buffer);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .large_file(large);
    writer.start_file(name, options).expect("start member");
    writer.write_all(real).expect("write member");
    writer.finish().expect("finish archive");
    let mut bytes = buffer.into_inner();

    if large {
        // The 32-bit size fields already read 0xFFFFFFFF, and a zip64 extra field
        // holds the real size as a u64. Rewrite that u64, in both the local and the
        // central copy, to the declared size. Nothing changes length.
        patch_zip64_uncompressed(&mut bytes, real.len() as u64, declared);
    } else {
        // Rewrite the 32-bit uncompressed-size field in the local file header and in
        // the central-directory header. Both are honest today and hold the real
        // length; neither move.
        patch_u32_uncompressed(&mut bytes, real.len() as u32, declared as u32);
    }
    bytes
}

/// Overwrite the 32-bit uncompressed-size field, which currently reads `from`, with
/// `to`, in the local file header and the central-directory header.
///
/// The field sits 22 bytes into a local header (signature `PK\x03\x04`) and 24 bytes
/// into a central header (`PK\x01\x02`); only the one member's headers hold `from` as
/// their size, so a match is unambiguous.
fn patch_u32_uncompressed(bytes: &mut [u8], from: u32, to: u32) {
    patch_size_field(
        bytes,
        b"PK\x03\x04",
        22,
        &from.to_le_bytes(),
        &to.to_le_bytes(),
    );
    patch_size_field(
        bytes,
        b"PK\x01\x02",
        24,
        &from.to_le_bytes(),
        &to.to_le_bytes(),
    );
}

/// Overwrite the zip64 extra field's uncompressed u64, which currently reads `from`,
/// with `to`, wherever the field appears (once in the local header, once in the
/// central one).
///
/// A zip64 extra field is `01 00`, a two-byte length, then the u64 sizes. The
/// uncompressed size is the first, so the eight bytes after the four-byte header are
/// what gets rewritten.
fn patch_zip64_uncompressed(bytes: &mut [u8], from: u64, to: u64) {
    let from = from.to_le_bytes();
    let to = to.to_le_bytes();
    let mut at = 0;
    while at + 4 + 8 <= bytes.len() {
        // A zip64 field header is id 0x0001; the length that follows is at least 8
        // when it carries the uncompressed size.
        if bytes[at] == 0x01 && bytes[at + 1] == 0x00 && bytes[at + 4..at + 12] == from {
            bytes[at + 4..at + 12].copy_from_slice(&to);
            at += 12;
        } else {
            at += 1;
        }
    }
}

/// Overwrite the `size` bytes at `offset` past every occurrence of `signature`, when
/// they currently read `from`.
fn patch_size_field(bytes: &mut [u8], signature: &[u8], offset: usize, from: &[u8], to: &[u8]) {
    let mut at = 0;
    while at + offset + from.len() <= bytes.len() {
        if bytes[at..].starts_with(signature)
            && bytes[at + offset..at + offset + from.len()] == *from
        {
            bytes[at + offset..at + offset + to.len()].copy_from_slice(to);
        }
        at += 1;
    }
}
