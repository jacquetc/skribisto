// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Stamps the release the binary was built from into `SKRIBISTO_GIT_DESCRIBE`,
//! which [`crate::version`] turns into the string the welcome window shows.
//!
//! The displayed version is a *git* fact, not a Cargo one: every crate in the
//! workspace shares one placeholder `version = "3.0.0"`, so `CARGO_PKG_VERSION`
//! cannot name a release. `git describe` can — and it also says whether HEAD is
//! the tag or merely descends from it, which is what keeps a dev build from
//! presenting itself as a release.
//!
//! Packagers building outside a git checkout (a source tarball) can set
//! `SKRIBISTO_GIT_DESCRIBE` themselves — e.g. `SKRIBISTO_GIT_DESCRIBE=v3.0.0` —
//! and it is taken verbatim. With neither git nor the override, the var is
//! stamped empty and `version.rs` falls back to `CARGO_PKG_VERSION`.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SKRIBISTO_GIT_DESCRIBE");

    let describe = std::env::var("SKRIBISTO_GIT_DESCRIBE")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(git_describe)
        .unwrap_or_default();

    println!("cargo:rustc-env=SKRIBISTO_GIT_DESCRIBE={describe}");

    stamp_channel();
    embed_windows_resources();
}

/// Stamps how this binary was distributed into `SKRIBISTO_CHANNEL`, which
/// [`crate::updates::Channel`] parses.
///
/// The application cannot work this out at runtime with any accuracy. A Flatpak
/// can be recognised from `/.flatpak-info`, but that says nothing about whether
/// it came from Flathub (which updates itself, so the application must stay
/// quiet) or from the bundle attached to a GitHub release (which does not, so it
/// is the only signal that user will ever get). A Windows installer and a
/// portable zip are the same executable in different places. A distribution
/// package leaves no runtime trace at all. Only the recipe that produced the
/// artifact knows, so the recipe is what says so.
///
/// Unset means `source`: a `cargo build`, a distribution rebuild, or anything
/// else that did not go through one of this project's packaging workflows.
/// [`Channel::Source`] does not check on its own, so the failure direction for
/// an unstamped build is silence rather than a wrong instruction.
///
/// No validation here on purpose. A build script that panics on a typo is a
/// packager's ruined afternoon; an unrecognised value parses to
/// [`Channel::Unknown`], which is also silent.
fn stamp_channel() {
    println!("cargo:rerun-if-env-changed=SKRIBISTO_CHANNEL");
    let channel = std::env::var("SKRIBISTO_CHANNEL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "source".to_string());
    println!("cargo:rustc-env=SKRIBISTO_CHANNEL={channel}");
}

/// For a Windows *target*, embed the app icon + version metadata into
/// `skribisto.exe` as Win32 resources so the taskbar, Explorer, and the NSIS
/// installer show the branded icon and correct version.
///
/// Gated on the target rather than the host. Build scripts are compiled for the
/// host, so `cfg!(windows)` is false when CI cross-compiles the Windows release
/// from Linux — host-gating would quietly produce an unbranded exe with no
/// VERSIONINFO. `CARGO_CFG_TARGET_OS` names the real target, and is what
/// `winresource` itself reads to pick a resource compiler: `rc.exe` from the
/// Windows SDK on a Windows host, `llvm-rc` when cross-compiling to the msvc
/// target (override either with `RC_PATH`). winit 0.30 sets PerMonitorV2 DPI
/// awareness programmatically, so no application manifest is embedded here.
fn embed_windows_resources() {
    println!("cargo:rerun-if-env-changed=RC_PATH");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    const ICON: &str = "../../resources/windows/skribisto.ico";
    println!("cargo:rerun-if-changed={ICON}");

    let mut res = winresource::WindowsResource::new();
    res.set_icon(ICON);
    // FileVersion / ProductVersion default to CARGO_PKG_VERSION; add the rest.
    res.set("ProductName", "Skribisto");
    res.set("FileDescription", "Skribisto — writing software");
    res.set("LegalCopyright", "GPL-3.0-only");
    if let Err(e) = res.compile() {
        // A resource-embedding failure must not hard-fail the build; the exe is
        // still functional, just without the branded icon.
        println!("cargo:warning=failed to embed Windows resources: {e}");
    }
}

/// `v3.0.0-alpha1` on a tag, `v3.0.0-alpha1-31-g5c62c8cac` off one, the bare
/// short hash in a repo with no tags at all. `None` outside a git checkout (or
/// with no `git` on PATH).
fn git_describe() -> Option<String> {
    watch_git_refs();
    git(&["describe", "--tags", "--always", "--abbrev=9"])
}

/// Tell cargo which git files invalidate the stamp.
///
/// Emitting any `rerun-if-changed` opts this build script out of cargo's
/// default "rerun when any file in the package changed" rule — so without these
/// lines the version would freeze at whatever HEAD happened to be the first time
/// the crate was compiled, and every later commit would ship a stale string.
///
/// `--absolute-git-dir` is the *worktree's* git dir (this repo is worked on in
/// linked worktrees), which is where its `HEAD` lives; refs and tags live in the
/// common dir shared with the main checkout. Paths that don't exist are skipped:
/// cargo treats a missing path as changed and would rerun the script on every
/// single build.
fn watch_git_refs() {
    let Some(git_dir) = git(&["rev-parse", "--absolute-git-dir"]).map(PathBuf::from) else {
        return;
    };
    // Relative to this script's CWD (the package root) — canonicalize so the
    // path we hand cargo doesn't depend on that.
    let common = git(&["rev-parse", "--git-common-dir"])
        .map(PathBuf::from)
        .and_then(|p| p.canonicalize().ok())
        .unwrap_or_else(|| git_dir.clone());

    let watched = [
        git_dir.join("HEAD"),       // branch switch, commit on a detached HEAD
        common.join("packed-refs"), // tags and branches, once packed
        common.join("refs").join("heads"),
        common.join("refs").join("tags"), // a fresh `git tag` must re-stamp
    ];
    for path in watched.iter().filter(|p| Path::exists(p)) {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

/// Run `git` in the package dir, returning its trimmed stdout on success.
fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (!text.is_empty()).then_some(text)
}
