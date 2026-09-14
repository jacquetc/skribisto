// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Finding a bundle path on disk whatever Unicode normalisation its name carries.
//!
//! # Why this exists
//!
//! [`crate::slug::slugify`] keeps a title's accented letters, so an exploded folder
//! holds files such as `…-raphaël-mireïa.scene.djot`, spelled precomposed (NFC) both on
//! disk and in `items.ron`. Those two spellings only ever agree on the machine that
//! wrote them. A synchronisation client (Nextcloud, rsync without `--iconv`), an HFS+
//! volume or a zip made by a third-party tool can hand another machine the *decomposed*
//! form — `e` followed by U+0308 rather than U+00EB — and on ext4 or NTFS those are two
//! different names. macOS never notices (APFS looks names up normalisation-
//! insensitively), so a project written there opened on Linux with "reading prose …:
//! No such file or directory" for a file that was sitting right there.
//!
//! The exploded shape exists for exactly the workflows — git, a synced folder — that
//! cross such a boundary, so the format tolerates it rather than the writer having to
//! notice.
//!
//! # What it does
//!
//! [`Locator::locate`](crate::locate::Locator::locate) validates the path through
//! [`crate::safe_path`] first —
//! containment is not relaxed by any of this: an alternative spelling is only ever
//! looked for *inside* the directory the checked path names — then returns:
//!
//! * the exact path, when it exists: the common case, one `stat`;
//! * otherwise, walking component by component, the entry whose NFC form equals the
//!   wanted component's NFC form, as far as such entries exist;
//! * with whatever components exist under no spelling appended verbatim, so a *new*
//!   file lands in the directory as it is already spelled, never in a twin beside it.
//!
//! That last point is why the writer goes through this too, not only the reader. A
//! read-only tolerance would have the next save write the NFC name as a second file and
//! prune the decomposed one as stale; synchronised back to a normalisation-insensitive
//! filesystem, those two names are one file, and whether the prose survives would
//! depend on the order the client replays a create and a delete. Writing into the
//! spelling already on disk renames nothing, so the client sees an edit, which every
//! client handles. The prunes compare names by NFC form for the same reason — and that
//! closes an older hole as well: HFS+ *returns* every name decomposed, so a byte-exact
//! prune there deleted an accented prose file in the same save that wrote it.
//!
//! # The cache
//!
//! Directory listings are cached for the lifetime of a `Locator`, one per read or write
//! of a bundle: a decomposed *file* name misses at the leaf, so without the cache a
//! project of N accented scenes would list its `text/` directory N times. Because the
//! exact path is always tried first, a file this writer creates (always under the exact
//! spelling) is found without the cache ever being consulted, so a listing taken early
//! in a write cannot go stale in a way that matters. The only removals happen in the
//! prunes, after the last lookup in that directory.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Component, Path, PathBuf};

use unicode_normalization::UnicodeNormalization;

/// `s` in Unicode Normalization Form C — the form [`crate::slug::slugify`] emits and
/// the key every file-name comparison in this crate uses.
pub fn nfc(s: &str) -> String {
    s.nfc().collect()
}

/// One directory's entries: NFC name → the name as the directory actually spells it.
type Listing = HashMap<String, OsString>;

/// Resolves bundle-relative paths to the files actually on disk. Create one per read
/// or write of a bundle; see the module docs for what the cache may and may not miss.
#[derive(Default)]
pub struct Locator {
    listings: RefCell<HashMap<PathBuf, Listing>>,
}

impl Locator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate `raw` against the bundle root (see [`crate::safe_path::bundle_relative`])
    /// and find it on disk under any normalisation. The returned path exists when the
    /// file does under some spelling; otherwise it is where the file should be created,
    /// inside whatever existing directories already spell its parents.
    pub fn locate(&self, root: &Path, raw: &str, context: &str) -> anyhow::Result<PathBuf> {
        let rel = crate::safe_path::bundle_relative(raw)
            .map_err(|e| anyhow::anyhow!("{context}: {e}"))?;
        let exact = root.join(&rel);
        if exact.exists() {
            return Ok(exact);
        }
        let mut cur = root.to_path_buf();
        let mut components = rel.components();
        while let Some(component) = components.next() {
            // `bundle_relative` admits only `Normal` components; anything else here
            // would be a bug in it, and skipping is the harmless answer.
            let Component::Normal(name) = component else {
                continue;
            };
            match self.child(&cur, name) {
                Some(found) => cur.push(found),
                None => {
                    cur.push(name);
                    cur.extend(components);
                    return Ok(cur);
                }
            }
        }
        Ok(cur)
    }

    /// One component under an already-resolved directory: `dir/name` if it exists,
    /// else the entry of `dir` spelling the same name in another normalisation, else
    /// `dir/name` as the place to create it. For a sidecar beside a prose file whose
    /// directory the caller has already resolved.
    pub fn locate_child(&self, dir: &Path, name: &str) -> PathBuf {
        match self.child(dir, OsStr::new(name)) {
            Some(found) => dir.join(found),
            None => dir.join(name),
        }
    }

    fn child(&self, dir: &Path, name: &OsStr) -> Option<OsString> {
        if dir.join(name).exists() {
            return Some(name.to_os_string());
        }
        let key = nfc(name.to_str()?);
        let mut listings = self.listings.borrow_mut();
        let listing = listings
            .entry(dir.to_path_buf())
            .or_insert_with(|| list(dir));
        listing.get(&key).cloned()
    }
}

/// The names in `dir`, keyed by NFC form. Two entries with the same NFC form (two
/// spellings of one name, side by side on a byte-exact filesystem) resolve to the
/// first in byte order, so a lookup is deterministic; a name that is not UTF-8 cannot
/// be one this crate wrote and is left out.
fn list(dir: &Path) -> Listing {
    let mut names: Vec<OsString> = match fs::read_dir(dir) {
        Ok(entries) => entries.flatten().map(|e| e.file_name()).collect(),
        Err(_) => return HashMap::new(),
    };
    names.sort();
    let mut out = HashMap::new();
    for name in names {
        if let Some(s) = name.to_str() {
            out.entry(nfc(s)).or_insert(name);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const NFC_DIR: &str = "01-première-partie";
    const NFC_FILE: &str = "0a1b2c3d-l-été.scene.djot";

    fn nfd(s: &str) -> String {
        s.nfd().collect()
    }

    /// A tree spelled decomposed on disk, or `None` where the filesystem normalises
    /// names itself (then the situation this module handles cannot arise).
    fn decomposed_tree() -> Option<(tempfile::TempDir, PathBuf)> {
        let dir = tempfile::tempdir().unwrap();
        let text = dir.path().join("binders").join(nfd(NFC_DIR)).join("text");
        fs::create_dir_all(&text).unwrap();
        let file = text.join(nfd(NFC_FILE));
        fs::write(&file, "Prose.").unwrap();
        let byte_exact = !dir
            .path()
            .join("binders")
            .join(NFC_DIR)
            .join("text")
            .join(NFC_FILE)
            .exists();
        byte_exact.then_some((dir, file))
    }

    #[test]
    fn nfc_folds_a_decomposed_spelling_and_nothing_else() {
        assert_eq!(nfc(&nfd("raphaël")), "raphaël");
        assert_eq!(nfc("plain"), "plain");
        assert_ne!(nfc("raphaël"), nfc("raphael"));
    }

    #[test]
    fn an_exact_path_is_returned_without_a_listing() {
        let dir = tempfile::tempdir().unwrap();
        let text = dir.path().join("binders").join(NFC_DIR).join("text");
        fs::create_dir_all(&text).unwrap();
        fs::write(text.join(NFC_FILE), "Prose.").unwrap();
        let rel = format!("binders/{NFC_DIR}/text/{NFC_FILE}");
        let locator = Locator::new();
        let found = locator.locate(dir.path(), &rel, "prose").unwrap();
        assert_eq!(found, dir.path().join(&rel));
        assert!(
            locator.listings.borrow().is_empty(),
            "no directory was listed"
        );
    }

    #[test]
    fn a_decomposed_name_is_found_at_every_level() {
        let Some((dir, on_disk)) = decomposed_tree() else {
            return;
        };
        let rel = format!("binders/{NFC_DIR}/text/{NFC_FILE}");
        let found = Locator::new().locate(dir.path(), &rel, "prose").unwrap();
        assert_eq!(found, on_disk);
        assert_eq!(fs::read_to_string(found).unwrap(), "Prose.");
    }

    #[test]
    fn a_missing_leaf_lands_in_the_existing_spelling_of_its_directory() {
        let Some((dir, on_disk)) = decomposed_tree() else {
            return;
        };
        let rel = format!("binders/{NFC_DIR}/text/0a1b2c3d-l-été.scene.comments.ron");
        let found = Locator::new().locate(dir.path(), &rel, "sidecar").unwrap();
        assert!(!found.exists());
        assert_eq!(
            found.parent(),
            on_disk.parent(),
            "same directory, as spelled on disk"
        );
        assert_eq!(
            found.file_name().and_then(|n| n.to_str()),
            Some("0a1b2c3d-l-été.scene.comments.ron"),
            "the new name keeps the requested spelling"
        );
    }

    #[test]
    fn locate_child_matches_a_sidecar_under_either_spelling() {
        let Some((dir, on_disk)) = decomposed_tree() else {
            return;
        };
        let text = on_disk.parent().unwrap();
        let sidecar_nfc = "0a1b2c3d-l-été.scene.comments.ron";
        fs::write(text.join(sidecar_nfc), "[]").unwrap();
        let locator = Locator::new();
        // Asked for by its decomposed spelling, found under its precomposed one.
        let found = locator.locate_child(text, &nfd(sidecar_nfc));
        assert_eq!(found, text.join(sidecar_nfc));
        // And a name under no spelling is simply where it would be created.
        let missing = locator.locate_child(text, "nothing-here.ron");
        assert_eq!(missing, text.join("nothing-here.ron"));
        drop(dir);
    }

    #[test]
    fn a_missing_middle_directory_yields_the_verbatim_path() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("binders")).unwrap();
        let rel = "binders/02-nope/text/x.scene.djot";
        let found = Locator::new().locate(dir.path(), rel, "prose").unwrap();
        assert_eq!(found, dir.path().join(rel));
        assert!(!found.exists());
    }

    #[test]
    fn containment_is_not_relaxed() {
        let dir = tempfile::tempdir().unwrap();
        let locator = Locator::new();
        for bad in ["../x.djot", "/etc/passwd", "binders/../x", ""] {
            let err = locator.locate(dir.path(), bad, "prose").unwrap_err();
            assert!(
                err.to_string().starts_with("prose: unsafe path"),
                "{bad}: {err}"
            );
        }
    }

    #[test]
    fn a_file_created_after_the_listing_is_still_found() {
        let Some((dir, on_disk)) = decomposed_tree() else {
            return;
        };
        let text = on_disk.parent().unwrap();
        let locator = Locator::new();
        // Populate the cache for `text/` with a miss…
        let later = "0a1b2c3d-l-été.scene.footnotes.ron";
        let planned = locator.locate_child(text, later);
        assert!(!planned.exists());
        // …then create the file exactly there, as the writer does.
        fs::write(&planned, "[]").unwrap();
        assert_eq!(locator.locate_child(text, later), planned);
        drop(dir);
    }
}
