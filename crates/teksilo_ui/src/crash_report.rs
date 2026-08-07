// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Panic diagnostics — a crash log written from the panic hook.
//!
//! # Why this exists, and what it deliberately does *not* do
//!
//! The workspace release profile used to set `panic = "abort"`. That silently
//! disabled [`std::panic::catch_unwind`] in
//! [`common::long_operation`], the boundary that turns a
//! panic inside a long operation (save, export, analysis, import…) into a
//! reported `Failed` instead of a dead process. Restoring `unwind` (see the
//! comment on `[profile.release]` in the workspace `Cargo.toml`) is what
//! actually protects a writer's unsaved prose: a panicking background save now
//! surfaces as a failure toast with the app — and every open document — still
//! alive.
//!
//! This module is the *other* half: whatever the panic was, record enough to
//! diagnose it. A panic hook runs before the process unwinds or aborts, so it is
//! the last place that can see the message, the location and the backtrace.
//!
//! **It does not try to dump prose.** That is a deliberate design decision, not
//! an omission. The documents live in [`OpenDoc`](crate::models::OpenDoc), an
//! `Rc`-based, single-threaded structure; a panic hook is `Send + Sync` and can
//! fire on any thread, so it cannot legally touch them. Reaching into the
//! backend store instead would mean taking locks from inside a hook that may
//! well have been triggered *while those very locks were held* — turning one
//! crash into a deadlock or a double panic. The hook therefore touches no
//! application state at all: it formats strings and writes one file.
//!
//! Prose safety is covered by the layers that are allowed to block: autosave,
//! `backup_now`, and the restored `catch_unwind` boundary above.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// Guards against a panic *inside* the hook (or a second thread panicking while
/// the first is still writing) turning into unbounded recursion. The first
/// panic is the one worth recording; any nested one falls straight through to
/// the default hook.
static REPORTING: AtomicBool = AtomicBool::new(false);

/// Directory crash logs are written to: `<data_dir>/crash-reports`.
///
/// `None` when the platform gives us no data directory, which is the one case
/// where there is nowhere sensible to write — the hook then degrades to stderr.
pub fn crash_dir() -> Option<PathBuf> {
    teksilo::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .map(|paths| paths.data_dir().join("crash-reports"))
}

/// Install the panic hook. Call once, as early in `main` as possible — before
/// any window, thread or store exists, so that a panic during startup is
/// covered too.
///
/// Chains to the previously installed hook (normally libstd's, which prints the
/// familiar message to stderr) rather than replacing it: losing the console
/// output would make a terminal run strictly worse to debug.
pub fn install() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Re-entrant panic (or a concurrent one): skip straight to the default
        // hook. `swap` rather than `store` so exactly one caller wins.
        if !REPORTING.swap(true, Ordering::SeqCst) {
            let report = render(info);
            match write_report(&report) {
                Some(path) => {
                    // Deliberately stderr and not `log`: the logging backend may
                    // itself be mid-panic, and this line is the pointer a user
                    // pastes into a bug report.
                    eprintln!("skribisto: crash report written to {}", path.display());
                }
                None => eprintln!("skribisto: could not write a crash report\n{report}"),
            }
            REPORTING.store(false, Ordering::SeqCst);
        }
        previous(info);
    }));
}

/// Format the report body. Pure — no I/O, no application state — so it is
/// directly testable.
fn render(info: &std::panic::PanicHookInfo<'_>) -> String {
    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("<unnamed>").to_string();

    // The payload is `&str` for a literal `panic!("…")` and `String` for an
    // interpolated one; anything else is a `panic_any` we can only describe.
    let message = info
        .payload()
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| info.payload().downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "<non-string panic payload>".to_string());

    let location = info
        .location()
        .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
        .unwrap_or_else(|| "<unknown location>".to_string());

    // `Backtrace::force_capture` ignores RUST_BACKTRACE: a crash report with no
    // backtrace is close to useless, and the user cannot set an env var
    // retroactively for the crash that already happened.
    let backtrace = std::backtrace::Backtrace::force_capture();

    format!(
        "Skribisto crash report\n\
         ----------------------\n\
         version : {version}\n\
         pid     : {pid}\n\
         thread  : {thread_name}\n\
         location: {location}\n\
         message : {message}\n\
         \n\
         backtrace:\n{backtrace}\n",
        version = env!("CARGO_PKG_VERSION"),
        pid = std::process::id(),
    )
}

/// Write `report` under [`crash_dir`], returning where it landed.
///
/// Every failure here is swallowed into `None`: a hook that propagated an error
/// would abort the process before the default hook printed anything, which is
/// precisely the diagnosis-destroying behaviour this module exists to prevent.
fn write_report(report: &str) -> Option<PathBuf> {
    let dir = crash_dir()?;
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join(file_name(std::process::id()));
    let mut file = std::fs::File::create(&path).ok()?;
    file.write_all(report.as_bytes()).ok()?;
    // Flush explicitly: the process is about to die, and `Drop` is not
    // guaranteed to run the way it would on a normal return.
    file.flush().ok()?;
    Some(path)
}

/// `skribisto-crash-<pid>-<nanos>.log`.
///
/// The timestamp is monotonic-since-epoch nanoseconds rather than a formatted
/// date because this runs in a panic hook: `chrono`'s local-time lookup can
/// allocate and consult the environment, and one pid can crash more than once
/// across a session's child processes. A raw counter cannot collide and cannot
/// fail.
fn file_name(pid: u32) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("skribisto-crash-{pid}-{nanos}.log")
}

/// Crash logs from previous runs, newest first — the input to "you crashed last
/// time, here is the report" surfacing, and to any future prune policy.
pub fn existing_reports(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut found: Vec<(std::time::SystemTime, PathBuf)> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "log")
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("skribisto-crash-"))
        })
        .map(|p| {
            let when = p
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            (when, p)
        })
        .collect();
    found.sort_by_key(|b| std::cmp::Reverse(b.0));
    found.into_iter().map(|(_, p)| p).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `std::panic::set_hook` is **process-global** while `cargo test` runs test
    /// functions on parallel threads, so a hook installed to observe *this*
    /// test's panic also fires for every other test panicking at the same
    /// moment — including the deliberate `debug_assert!` panics elsewhere in
    /// this binary. Two defences, both needed:
    ///
    /// 1. This mutex serialises the tests that swap the hook, so they cannot
    ///    clobber each other's `take_hook`/`set_hook` pairing.
    /// 2. `capture_panic` below filters on the panicking thread's *name*, so an
    ///    unrelated concurrent panic on another thread is passed through to the
    ///    previous hook instead of being mistaken for ours.
    static HOOK_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Run `body` on a uniquely-named thread with a hook installed that records
    /// [`render`]'s output for panics originating on *that* thread only.
    fn capture_panic(thread_name: &str, body: fn()) -> String {
        let _serialised = HOOK_GUARD.lock().unwrap_or_else(|e| e.into_inner());

        let captured = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let sink = std::sync::Arc::clone(&captured);
        let wanted = thread_name.to_string();

        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let current = std::thread::current();
            if current.name() == Some(wanted.as_str()) {
                *sink.lock().unwrap_or_else(|e| e.into_inner()) = render(info);
            } else {
                previous(info);
            }
        }));

        let outcome = std::thread::Builder::new()
            .name(thread_name.to_string())
            .spawn(move || {
                let _ = std::panic::catch_unwind(body);
            })
            .expect("spawning the capture thread")
            .join();

        // Restore libstd's hook rather than the captured `previous` — the
        // closure above owns that one, and it cannot be moved back out.
        let _ = std::panic::take_hook();
        outcome.expect("the capture thread must not fail to join");

        captured.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// The report must name the thread, the location and the message — the three
    /// things a bug report is useless without.
    #[test]
    fn a_report_records_thread_location_and_message() {
        let report = capture_panic("audited-thread", || panic!("a deliberate test panic"));

        assert!(
            report.contains("audited-thread"),
            "the report must name the panicking thread: {report}"
        );
        assert!(
            report.contains("a deliberate test panic"),
            "the report must carry the panic message: {report}"
        );
        assert!(
            report.contains("crash_report.rs:"),
            "the report must locate the panic in this file: {report}"
        );
        assert!(
            report.contains("backtrace:"),
            "the report must carry a backtrace section: {report}"
        );
    }

    /// A `String` payload (an interpolated `panic!("{x}")`) must be recovered
    /// too — the `&str` downcast alone silently loses every formatted message.
    #[test]
    fn an_interpolated_panic_message_survives() {
        let report = capture_panic("interpolated-payload-thread", || {
            let detail = "interpolated detail";
            panic!("boom: {detail}")
        });
        assert!(
            report.contains("boom: interpolated detail"),
            "a String payload must be recovered, not reported as non-string: {report}"
        );
    }

    /// Two crashes in the same process must not overwrite each other's report.
    #[test]
    fn report_file_names_do_not_collide() {
        let a = file_name(4242);
        let b = file_name(4242);
        assert_ne!(a, b, "same-pid reports must still get distinct names");
        assert!(
            a.starts_with("skribisto-crash-4242-"),
            "unexpected name: {a}"
        );
        assert!(a.ends_with(".log"), "unexpected name: {a}");
    }

    /// `existing_reports` lists only our own logs, and never errors on a
    /// missing or unreadable directory.
    #[test]
    fn existing_reports_filters_and_tolerates_a_missing_dir() {
        let dir = std::env::temp_dir().join(format!("skribisto-crash-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(
            existing_reports(&dir).is_empty(),
            "a missing directory must read as 'no reports', not panic"
        );

        std::fs::create_dir_all(&dir).expect("creating the test directory");
        std::fs::write(dir.join("skribisto-crash-1-1.log"), "a").expect("writing a report");
        std::fs::write(dir.join("unrelated.log"), "b").expect("writing an unrelated log");
        std::fs::write(dir.join("skribisto-crash-2-2.txt"), "c").expect("writing a non-log");

        let found = existing_reports(&dir);
        assert_eq!(found.len(), 1, "only our own .log files count: {found:?}");
        assert!(
            found[0].ends_with("skribisto-crash-1-1.log"),
            "unexpected match: {found:?}"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
