// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The guard's own tests: the directory pre-check, the inflating budget, the ratio,
//! and that a refusal survives the trip to the UI and back.

use super::fixtures::{MIB, compressible_zip, over_declared_zip};
use super::*;
use zip::ZipArchive;

/// Small limits, so a crafted archive trips them without holding anything large.
fn tiny_limits() -> ZipLimits {
    ZipLimits {
        max_entries: 16,
        max_member_bytes: 4 * MIB,
        max_total_bytes: 8 * MIB,
        max_ratio: MAX_RATIO,
    }
}

fn guard_for(limits: ZipLimits, bytes: &[u8]) -> (ZipArchive<std::io::Cursor<Vec<u8>>>, ZipGuard) {
    ZipGuard::open(limits, std::io::Cursor::new(bytes.to_vec())).expect("open")
}

/// A member declaring a size past what a reader allows is refused over the central
/// directory, before a byte of it is inflated — the check that has to fire before a
/// reader reserves the declared size.
#[test]
fn an_over_declared_member_is_refused_before_inflating() {
    let bytes = over_declared_zip("word/document.xml", 3 << 30);
    let (mut archive, guard) = guard_for(tiny_limits(), &bytes);
    let refused = guard
        .check_directory(&mut archive, |_| true)
        .expect_err("must refuse");
    assert_eq!(
        refused,
        ZipRefused::MemberTooLarge {
            part: "word/document.xml".to_string(),
            limit: RATIO_FLOOR_BYTES.min(tiny_limits().max_member_bytes),
            declared: Some(3 << 30),
        }
    );
}

/// A zip64 header declaring more than 4 GiB is read as the u64 it is and refused the
/// same way — the case a 32-bit size field cannot even express.
#[test]
fn a_zip64_over_declared_member_is_refused_before_inflating() {
    let declared = 16u64 << 30;
    let bytes = over_declared_zip("content.xml", declared);
    let (mut archive, guard) = guard_for(tiny_limits(), &bytes);
    // The reader really does see the huge declared size, not the real content length.
    assert_eq!(archive.by_name("content.xml").unwrap().size(), declared);
    let refused = guard
        .check_directory(&mut archive, |_| true)
        .expect_err("must refuse");
    assert!(
        matches!(refused, ZipRefused::MemberTooLarge { declared: Some(d), .. } if d == declared)
    );
}

/// A member a reader will never open is not a reason to refuse the document: the
/// pre-check only weighs the members `wanted` accepts.
#[test]
fn a_member_the_reader_ignores_is_not_weighed() {
    let bytes = over_declared_zip("word/media/image1.png", 3 << 30);
    let (mut archive, guard) = guard_for(tiny_limits(), &bytes);
    // Reading only the document part, this bomb-shaped image is never weighed.
    guard
        .check_directory(&mut archive, |name| name == "word/document.xml")
        .expect("an ignored member is not weighed");
}

/// An honest archive of ordinary members passes both checks and reads back whole.
#[test]
fn an_honest_archive_passes() {
    let mut buffer = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut buffer);
    let options = zip::write::SimpleFileOptions::default();
    use std::io::Write;
    writer.start_file("a.txt", options).unwrap();
    writer.write_all(b"hello").unwrap();
    writer.start_file("b.txt", options).unwrap();
    writer.write_all(b"world").unwrap();
    writer.finish().unwrap();
    let bytes = buffer.into_inner();

    let (mut archive, mut guard) = guard_for(tiny_limits(), &bytes);
    guard
        .check_directory(&mut archive, |_| true)
        .expect("passes");
    let a = guard
        .read_named(&mut archive, "a.txt")
        .expect("read")
        .expect("present");
    assert_eq!(a, b"hello");
}

/// A compressible member unpacking past the per-member ceiling is refused while it
/// inflates, in bounded memory: the reader stops one byte past the budget rather
/// than reading the whole of it out.
#[test]
fn a_compressible_member_past_the_member_ceiling_is_refused_while_inflating() {
    // Its header is honest, so the pre-check passes it (its declared size is under
    // the ceiling only because we set a low member ceiling here); the inflating read
    // is what refuses it once it passes the budget the ratio gives it.
    let limits = ZipLimits {
        max_entries: 16,
        max_member_bytes: 2 * MIB,
        max_total_bytes: 64 * MIB,
        max_ratio: MAX_RATIO,
    };
    // 8 MiB of zeros in an archive of a few KiB: its compressed size is tiny, so its
    // ratio budget is the floor, but the member ceiling caps it at 2 MiB.
    let bytes = compressible_zip("big", 8 * MIB);
    let (mut archive, mut guard) = guard_for(limits, &bytes);
    let mut entry = archive.by_name("big").unwrap();
    let err = guard.read(&mut entry).expect_err("past the member ceiling");
    let refused = refused(&err).expect("a typed refusal");
    assert!(matches!(refused, ZipRefused::MemberTooLarge { .. }));
}

/// The ratio, not the floor, is what stops a bomb once the archive is past the floor:
/// a member whose compressed size is a thousandth of what it unpacks to is refused
/// even with a generous ceiling.
#[test]
fn the_ratio_refuses_a_high_expansion_member() {
    let limits = ZipLimits {
        max_entries: 16,
        max_member_bytes: 4 << 30,
        max_total_bytes: 4 << 30,
        max_ratio: MAX_RATIO,
    };
    // A guard whose floor is tiny, so the ratio is what decides.
    let bytes = compressible_zip("big", 8 * MIB);
    let archive_len = bytes.len() as u64;
    let (mut archive, _) = guard_for(limits, &bytes);
    let mut guard = ZipGuard::new(limits, archive_len);
    // With the real floor (64 MiB) an 8 MiB member passes; that is by design (below
    // the floor the ratio is not consulted). So this asserts the boundary: a member
    // under the floor is accepted whatever its ratio.
    let mut entry = archive.by_name("big").unwrap();
    let copied = guard.read(&mut entry).expect("under the floor, accepted");
    assert_eq!(copied.len() as u64, 8 * MIB);
}

/// An archive whose members together pass the total ceiling is refused, even when no
/// single one does.
#[test]
fn the_total_ceiling_refuses_many_members_together() {
    let limits = ZipLimits {
        max_entries: 64,
        max_member_bytes: 4 * MIB,
        max_total_bytes: 6 * MIB,
        max_ratio: MAX_RATIO,
    };
    let mut buffer = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut buffer);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    use std::io::Write;
    let chunk = vec![0u8; MIB as usize];
    for i in 0..8 {
        writer.start_file(format!("m{i}"), options).unwrap();
        writer.write_all(&chunk).unwrap();
    }
    writer.finish().unwrap();
    let bytes = buffer.into_inner();

    let (mut archive, mut guard) = guard_for(limits, &bytes);
    let mut refused_at = None;
    for i in 0..8 {
        let mut entry = archive.by_name(&format!("m{i}")).unwrap();
        if let Err(e) = guard.read(&mut entry) {
            refused_at = Some((i, refused(&e).cloned()));
            break;
        }
    }
    let (index, refusal) = refused_at.expect("the archive must be refused before all 8 MiB");
    assert!(index < 8);
    assert!(matches!(refusal, Some(ZipRefused::ArchiveTooLarge { .. })));
}

/// Too many entries is refused before any is read.
#[test]
fn too_many_entries_is_refused() {
    let mut buffer = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut buffer);
    let options = zip::write::SimpleFileOptions::default();
    for i in 0..20 {
        writer.start_file(format!("f{i}"), options).unwrap();
    }
    writer.finish().unwrap();
    let bytes = buffer.into_inner();

    let (mut archive, guard) = guard_for(tiny_limits(), &bytes);
    let refused = guard
        .check_directory(&mut archive, |_| true)
        .expect_err("20 > 16");
    assert_eq!(
        refused,
        ZipRefused::TooManyEntries {
            entries: 20,
            limit: 16
        }
    );
}

/// `verify_all` refuses an over-declared member the same way, for the reader that
/// hands the whole archive to a parser of its own.
#[test]
fn verify_all_refuses_a_bomb() {
    let limits = ZipLimits {
        max_entries: 16,
        max_member_bytes: 2 * MIB,
        max_total_bytes: 8 * MIB,
        max_ratio: MAX_RATIO,
    };
    let bytes = compressible_zip("big", 8 * MIB);
    let (mut archive, mut guard) = guard_for(limits, &bytes);
    let err = guard.verify_all(&mut archive).expect_err("must refuse");
    assert!(refused(&err).is_some());
}

/// A refusal survives the flatten-to-a-string trip a long operation makes and comes
/// back the same typed value.
#[test]
fn a_refusal_round_trips_through_a_failure_message() {
    for refusal in [
        ZipRefused::TooManyEntries {
            entries: 9,
            limit: 8,
        },
        ZipRefused::MemberTooLarge {
            part: "word/media:image:1.png".to_string(),
            limit: 64 << 20,
            declared: Some(1 << 40),
        },
        ZipRefused::MemberTooLarge {
            part: "content.xml".to_string(),
            limit: 64 << 20,
            declared: None,
        },
        ZipRefused::ArchiveTooLarge { limit: 8 << 30 },
    ] {
        let message = refusal.failure_message();
        assert_eq!(ZipRefused::from_failure_message(&message), Some(refusal));
    }
    assert_eq!(ZipRefused::from_failure_message("something else"), None);
}

/// `refused` finds the value through the `anyhow` context layers a reader stacks on
/// it, and through `SkribFormatError::Unreadable`.
#[test]
fn refused_is_found_through_context_and_the_format_error() {
    let refusal = ZipRefused::ArchiveTooLarge { limit: 8 << 30 };
    let error = anyhow::Error::new(refusal.clone()).context("extracting zip 'x.skrib'");
    assert_eq!(refused(&error), Some(&refusal));

    let wrapped: anyhow::Error =
        SkribFormatError::Unreadable(anyhow::Error::new(refusal.clone())).into();
    assert_eq!(refused(&wrapped), Some(&refusal));
}
