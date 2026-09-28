// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the writer round-trip tests share: where an export is written before the
//! test reads it back.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// A path in the temporary directory that no other export of this process has been
/// given, for one export to write and its test to read back.
///
/// The tests used to name the file after the process id and the clock's
/// nanoseconds. The tests of one file run in parallel in one process, so the id is
/// the same for every one of them, and two tests that read the clock in the same
/// tick wrote their exports to one file: one test then read the other's document
/// back and failed on a comment it never wrote. The Windows clock moves in steps of
/// 100 ns, which is how it showed there first, as a comment's uid coming back as
/// another test's. A counter never repeats within a process, and the id keeps two
/// processes apart.
pub fn unique_temp_path(stem: &str, extension: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{stem}_{}_{n}.{extension}", std::process::id()))
}

/// Every export of a run gets a path of its own, however many tests export at once.
#[test]
fn no_two_exports_are_given_the_same_path() {
    const THREADS: usize = 8;
    const PER_THREAD: usize = 2_000;
    let start = std::sync::Arc::new(std::sync::Barrier::new(THREADS));
    let workers: Vec<_> = (0..THREADS)
        .map(|_| {
            let start = std::sync::Arc::clone(&start);
            std::thread::spawn(move || {
                start.wait();
                (0..PER_THREAD)
                    .map(|_| unique_temp_path("roundtrip", "odt"))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut seen = std::collections::HashSet::new();
    for worker in workers {
        let paths = match worker.join() {
            Ok(paths) => paths,
            Err(panic) => std::panic::resume_unwind(panic),
        };
        for path in paths {
            assert!(
                seen.insert(path.clone()),
                "{} was given twice",
                path.display()
            );
        }
    }
    assert_eq!(seen.len(), THREADS * PER_THREAD);
}
