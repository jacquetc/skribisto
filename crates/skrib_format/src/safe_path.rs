// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The one place a path that came out of a bundle is allowed to become a real
//! path on this machine.
//!
//! # Why this exists
//!
//! Every other reader in this crate was written for a file the writer themselves
//! produced, and it showed. A bundle carries path strings in three places —
//! `templates.ron`'s `path`, `assets.ron`'s `path`, and each `ProseRef.path` in
//! `items.ron` — plus the entry names of the zip itself, which
//! [`crate::carry`] keeps verbatim for anything it does not model. All four were
//! joined straight onto the bundle root, and
//! [`Path::join`](std::path::Path::join) with an **absolute** argument discards
//! the base entirely:
//!
//! ```text
//! Path::new("/tmp/extract").join("/etc/passwd")  ==  "/etc/passwd"
//! ```
//!
//! So an `assets.ron` naming `/home/someone/.ssh/id_rsa` read that file into the
//! bundle, from where the next save copied it into the writer's own project; and
//! a zip entry named `../../../.bashrc` was classified unmodelled, carried, and
//! written back out through the same join on the next autosave. Neither needed a
//! malicious *writer* — only a bundle that arrived from somewhere, which a
//! `.skrib` already does: it is mailed, shared on a drive, restored from someone
//! else's backup.
//!
//! # The rule
//!
//! A path out of a bundle is **relative, and stays inside the bundle**. That is
//! the whole contract, and [`bundle_relative`] is the only way to spell it.
//!
//! Containment is decided **syntactically, before touching the filesystem** —
//! not by canonicalising and comparing prefixes. Canonicalisation resolves
//! symlinks, so it answers a question about the tree as it is *now*, and between
//! that answer and the `open` that follows, the tree can change. Rejecting
//! `..` and absolute roots outright has no such window, needs no I/O, and gives
//! the same answer on a path that does not exist yet — which the write side
//! needs.
//!
//! Rejected, and why each one is not merely theoretical:
//!
//! * **Absolute** (`/etc/passwd`, `C:\Windows\…`) — the `join` behaviour above.
//! * **Any `..` component** — the classic traversal, and the one a zip entry
//!   name reaches through [`crate::carry`].
//! * **A `\` anywhere** — on Windows it is a component separator, so a name
//!   containing one is a path pretending to be a filename and would escape
//!   there while looking inert here.
//! * **A NUL or other control character** — never produced, and it truncates a
//!   path at the libc boundary.
//! * **A leading `.` component, or an empty string** — no legitimate bundle path
//!   is written that way, and both are ways to spell a path that does not look
//!   like the one it resolves to.
//!
//! Deliberately **not** rejected: a colon. It is illegal on Windows and this
//! crate never emits one, so an earlier draft refused it for portability — but
//! every rule here is applied to bundles that already exist, and a project
//! carrying a stray `Notes: draft 2.txt` would have become unopenable, and then
//! unsavable, for a file nothing reads. Portability is not containment; only
//! containment belongs in a rule that can refuse someone's book.
//!
//! What is *not* rejected is a name this crate would not itself have generated:
//! a bundle written by an older or newer build, or by the Plume/Manuskript
//! importers, is still a legitimate bundle. Re-deriving each path from its
//! `file_id`/`content_hash` instead of validating it was considered and refused
//! for exactly that reason — [`crate::slug::prose_relpath`] folds in the item's
//! *title*, so re-deriving would stop finding the prose of every project whose
//! titles have changed since it was last written, which is most of them.

use std::path::{Component, Path, PathBuf};

/// Why a bundle-supplied path was refused. Carries the offending string so the
/// error names the file rather than only the rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafePath {
    /// The path exactly as the bundle spelled it.
    pub raw: String,
    /// Which rule it broke, as a short phrase for an error message.
    pub reason: &'static str,
}

impl std::fmt::Display for UnsafePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unsafe path in bundle: '{}' ({})",
            self.raw.escape_debug(),
            self.reason
        )
    }
}

impl std::error::Error for UnsafePath {}

/// Validate a bundle-relative path and return it as a [`PathBuf`] safe to join
/// onto the bundle root.
///
/// See the module docs for the rule and the reasoning. This is deliberately the
/// *only* export: a caller that wants "just sanitise it for me" would quietly
/// rewrite a path to something that does not name the file the manifest meant,
/// and a bundle whose prose silently reads back empty is worse than one that
/// refuses to open.
pub fn bundle_relative(raw: &str) -> Result<PathBuf, UnsafePath> {
    let bad = |reason: &'static str| UnsafePath {
        raw: raw.to_string(),
        reason,
    };

    if raw.is_empty() {
        return Err(bad("empty"));
    }
    if raw.contains('\\') {
        return Err(bad("contains a backslash"));
    }
    if raw.chars().any(|c| c.is_control()) {
        return Err(bad("contains a control character"));
    }

    let path = Path::new(raw);
    if path.is_absolute() {
        return Err(bad("absolute"));
    }

    let mut components = 0usize;
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                // `Path` yields OsStr; everything this crate writes is UTF-8, and
                // a component that is not tells us nothing good about the source.
                part.to_str().ok_or_else(|| bad("not valid UTF-8"))?;
                components += 1;
            }
            Component::ParentDir => return Err(bad("contains '..'")),
            Component::CurDir => return Err(bad("contains '.'")),
            // `RootDir` is caught by `is_absolute` above on Unix; `Prefix` is the
            // Windows drive/UNC form. Both are here so the match stays exhaustive
            // and a future platform cannot slip through a wildcard.
            Component::RootDir => return Err(bad("absolute")),
            Component::Prefix(_) => return Err(bad("has a drive or UNC prefix")),
        }
    }
    if components == 0 {
        return Err(bad("names no file"));
    }

    Ok(path.to_path_buf())
}

/// Validate `raw` and join it onto `root`.
///
/// The pairing every caller actually wants, kept here so no call site can
/// perform the join and forget the check. `context` names what was being read or
/// written, for the error message.
pub fn join_checked(root: &Path, raw: &str, context: &str) -> anyhow::Result<PathBuf> {
    let rel = bundle_relative(raw).map_err(|e| anyhow::anyhow!("{context}: {e}"))?;
    Ok(root.join(rel))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_bundle_paths_are_accepted() {
        for ok in [
            "assets/abc123.png",
            "templates/0a1b-character-sheet.djot",
            "binders/01-manuscript/text/0a1b2c3d-the-ferry.scene.djot",
            "binders/01-manuscript/text/0a1b2c3d-the-ferry.scene.comments.ron",
            "history/9f8e7d.djot",
            "project.skrib",
        ] {
            assert!(bundle_relative(ok).is_ok(), "should accept {ok}");
        }
    }

    /// The read primitive: `Path::join` with an absolute argument throws the base
    /// away. This asserts the standard-library behaviour the module exists to
    /// defend against, so the reasoning cannot rot silently.
    // Demonstrating this behaviour *is* the test, so the lint that warns about
    // it is agreeing with the module rather than finding a bug in it.
    #[allow(clippy::join_absolute_paths)]
    #[test]
    fn join_with_an_absolute_path_discards_the_base() {
        assert_eq!(
            Path::new("/tmp/extract").join("/etc/passwd"),
            Path::new("/etc/passwd")
        );
        assert!(bundle_relative("/etc/passwd").is_err());
    }

    #[test]
    fn traversal_is_refused() {
        for bad in [
            "../../../.bashrc",
            "assets/../../escape",
            "..",
            "a/../../b",
            "./sneaky",
        ] {
            assert!(bundle_relative(bad).is_err(), "should refuse {bad}");
        }
    }

    #[test]
    fn windows_shapes_are_refused_on_every_platform() {
        for bad in [
            r"C:\Windows\System32\drivers\etc\hosts",
            r"..\..\evil",
            r"assets\x.png",
            r"\\server\share\x",
        ] {
            assert!(bundle_relative(bad).is_err(), "should refuse {bad}");
        }
    }

    /// Containment is the rule; portability is not. A colon is illegal on
    /// Windows and never written here, but refusing it would make an existing
    /// project carrying one unopenable — and then unsavable — for a file
    /// nothing reads.
    #[test]
    fn a_colon_is_allowed_because_it_cannot_escape() {
        assert!(bundle_relative("Notes: draft 2.txt").is_ok());
        assert!(bundle_relative("pro/a:b.ron").is_ok());
    }

    #[test]
    fn empty_and_control_characters_are_refused() {
        assert!(bundle_relative("").is_err());
        assert!(bundle_relative("a\0b").is_err());
        assert!(bundle_relative("a\nb").is_err());
    }

    #[test]
    fn the_error_names_the_offending_path() {
        let e = bundle_relative("../secret").unwrap_err();
        assert_eq!(e.raw, "../secret");
        assert!(e.to_string().contains("../secret"), "{e}");
        assert!(e.to_string().contains(".."), "{e}");
    }

    #[test]
    fn join_checked_refuses_rather_than_escaping() {
        let root = Path::new("/tmp/bundle");
        assert!(join_checked(root, "assets/a.png", "asset").is_ok());
        let err = join_checked(root, "/etc/passwd", "asset").unwrap_err();
        assert!(err.to_string().contains("asset"), "{err}");
    }
}
