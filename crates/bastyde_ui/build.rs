//! Stamps the release the binary was built from into `SKRIBISTO_GIT_DESCRIBE`,
//! which [`crate::version`] turns into the string the welcome window shows.
//!
//! The displayed version is a *git* fact, not a Cargo one: every crate in the
//! workspace shares one placeholder `version = "0.0.1"`, so `CARGO_PKG_VERSION`
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
