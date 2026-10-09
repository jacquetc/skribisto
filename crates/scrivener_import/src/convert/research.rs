// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Research files Skribisto cannot open, copied into a folder beside the new project
//! (decided 2026-09-27, UQ-37: on by default).
//!
//! A PDF, a web archive or a sound a writer kept in Scrivener's Research folder has no
//! place in a `.skrib`, and writers tend to delete the Scrivener project once they have
//! moved. So each such file is copied, as it is, into one folder beside the project, and its
//! note names where: `<folder>/<file>`, a path relative to the project's own folder.
//!
//! Staged like the project itself: the files are written into a hidden folder of this
//! import's own, which is moved into place just before the project is, and removed on every
//! other way out (a refusal, a cancellation, an error, a panic, a project that could not be
//! put in place after it). A folder that cannot be put in place fails the import whole, so
//! a project never stands with notes naming files that are not beside it. The folder's name
//! is the caller's words in the writer's language, never one the import invents; a name
//! already taken beside the project gets a number, since whatever is there is the writer's
//! and is never replaced.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use skrib_format::importing::OutputRefused;
use unicode_normalization::UnicodeNormalization;

use super::fill;

/// The longest a copied file's name may be, in characters and in bytes: well inside what
/// every file system accepts (255 bytes on Linux and macOS, 255 UTF-16 units on Windows,
/// which a name within 200 bytes never passes), with room for a number and for the hidden
/// staging folder's longer name.
const MAX_NAME_CHARS: usize = 120;
const MAX_NAME_BYTES: usize = 200;

/// The longest suffix read as a file's extension, and so kept when a long name is cut.
const MAX_EXTENSION_CHARS: usize = 16;

/// The files of one import, staged beside its project.
pub(super) struct ResearchCopy {
    /// The folder's name, beside the project.
    name: String,
    folder: PathBuf,
    staged: PathBuf,
    /// Names given so far, folded for comparison: two files may not share one on a file
    /// system that ignores case.
    taken: BTreeSet<String>,
    copied: u32,
    /// The staged folder was moved into place.
    placed: bool,
    /// The project is in place too: the folder is the writer's now.
    committed: bool,
}

impl ResearchCopy {
    /// The folder for the project at `output`, named from `pattern` with the project's own
    /// file name in its slot.
    pub(super) fn beside(output: &str, pattern: &str) -> ResearchCopy {
        let output = Path::new(output);
        let parent = output
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."))
            .to_path_buf();
        let stem = output
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let base = safe_name(&fill(pattern, &stem));
        let mut name = base.clone();
        let mut n = 1u32;
        while parent.join(&name).exists() {
            n += 1;
            name = format!("{base} {n}");
        }
        // A staging name no one else uses, so that removing it can only remove this
        // import's own files.
        let staged = parent.join(format!(
            ".{name}.importing-{}",
            &common::uid::new_uid().simple().to_string()[..8]
        ));
        ResearchCopy {
            folder: parent.join(&name),
            name,
            staged,
            taken: BTreeSet::new(),
            copied: 0,
            placed: false,
            committed: false,
        }
    }

    /// Forget what an attempt copied: the project is being read again.
    pub(super) fn restart(&mut self) {
        let _ = std::fs::remove_dir_all(&self.staged);
        self.taken.clear();
        self.copied = 0;
    }

    /// Copy one file, named after `wanted`; the path its note names, relative to the
    /// project's folder.
    pub(super) fn copy(&mut self, wanted: &str, bytes: &[u8]) -> Result<String, OutputRefused> {
        let base = safe_name(wanted);
        let (stem, extension) = match base.rsplit_once('.') {
            Some((stem, extension)) if !stem.is_empty() => {
                (stem.to_string(), format!(".{extension}"))
            }
            _ => (base.clone(), String::new()),
        };
        let mut name = base;
        let mut n = 1u32;
        while self.taken.contains(&fold(&name)) {
            n += 1;
            name = format!("{stem} ({n}){extension}");
        }
        self.taken.insert(fold(&name));
        std::fs::create_dir_all(&self.staged).map_err(|e| self.refused(&e))?;
        std::fs::write(self.staged.join(&name), bytes).map_err(|e| self.refused(&e))?;
        self.copied += 1;
        Ok(format!("{}/{name}", self.name))
    }

    pub(super) fn copied(&self) -> u32 {
        self.copied
    }

    /// Move the staged folder into place, just before the project is: its name, or `None`
    /// when nothing was copied. Until [`Self::keep`], dropping this removes the folder again.
    pub(super) fn place(&mut self) -> Result<Option<String>, OutputRefused> {
        if self.copied == 0 {
            return Ok(None);
        }
        // A folder made under that name while the import ran is someone else's: renaming
        // over it would replace it where it is empty, and fail where it is not.
        if std::fs::symlink_metadata(&self.folder).is_ok() {
            return Err(self.refused(&std::io::Error::from(std::io::ErrorKind::AlreadyExists)));
        }
        std::fs::rename(&self.staged, &self.folder).map_err(|e| self.refused(&e))?;
        self.placed = true;
        Ok(Some(self.name.clone()))
    }

    /// The project is in place: the folder beside it stays.
    pub(super) fn keep(mut self) {
        self.committed = true;
    }

    fn refused(&self, error: &std::io::Error) -> OutputRefused {
        let folder = self
            .folder
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        OutputRefused::NotWritten {
            file: self.name.clone(),
            folder,
            disk_full: matches!(
                error.kind(),
                std::io::ErrorKind::StorageFull | std::io::ErrorKind::QuotaExceeded
            ),
            reason: error.to_string(),
        }
    }
}

impl Drop for ResearchCopy {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        // Nothing to report to: this runs on the way out of an import that failed or was
        // cancelled, and the folder may never have been created. A folder already placed
        // is this import's own, under a name nothing held when it was placed.
        let _ = std::fs::remove_dir_all(if self.placed {
            &self.folder
        } else {
            &self.staged
        });
    }
}

/// `name` as one safe file name: no separator, no character a file system refuses, no name
/// Windows reserves, not only dots, and not too long, in characters or in bytes. A name cut
/// to fit keeps its extension, so the file still opens with the application it belongs to.
fn safe_name(name: &str) -> String {
    let mut out: String = name
        .nfc()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    out = trim_name(&out);
    let (stem, extension) = match out.rsplit_once('.') {
        Some((stem, extension))
            if !stem.is_empty()
                && !extension.is_empty()
                && extension.chars().count() <= MAX_EXTENSION_CHARS
                && extension.chars().all(char::is_alphanumeric) =>
        {
            (stem.to_string(), format!(".{extension}"))
        }
        _ => (out.clone(), String::new()),
    };
    let chars_left = MAX_NAME_CHARS.saturating_sub(extension.chars().count());
    let bytes_left = MAX_NAME_BYTES.saturating_sub(extension.len());
    let mut kept = String::new();
    for c in stem.chars().take(chars_left) {
        if kept.len() + c.len_utf8() > bytes_left {
            break;
        }
        kept.push(c);
    }
    // A cut can leave a space or a dot at the end, which Windows refuses.
    let kept = trim_name(&kept);
    out = if kept.is_empty() && !extension.is_empty() {
        format!("_{extension}")
    } else {
        format!("{kept}{extension}")
    };
    let stem = out
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let reserved = ["CON", "PRN", "AUX", "NUL"].contains(&stem.as_str())
        || ((stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.len() == 4
            && stem[3..].chars().all(|c| c.is_ascii_digit()));
    if out.is_empty() {
        out = "_".to_string();
    } else if reserved {
        out.insert(0, '_');
    }
    out
}

/// `name` without the spaces and dots a file system drops or refuses at its ends.
fn trim_name(name: &str) -> String {
    name.trim().trim_matches('.').trim().to_string()
}

fn fold(name: &str) -> String {
    name.nfc().flat_map(char::to_lowercase).collect()
}

#[cfg(test)]
mod tests {
    use super::super::TITLE_SLOT;
    use super::*;

    #[test]
    fn a_file_name_is_made_safe_and_never_a_path() {
        assert_eq!(safe_name("../../etc/passwd"), "_.._etc_passwd");
        assert_eq!(safe_name("Tide: tables?.pdf"), "Tide_ tables_.pdf");
        assert_eq!(safe_name("CON.pdf"), "_CON.pdf");
        assert_eq!(safe_name("  "), "_");
    }

    /// A long title in a script of several bytes a character (a web page kept as research,
    /// in Japanese) is cut to what a file system holds, its extension kept: past 255 bytes
    /// the copy failed and took the whole import with it.
    #[test]
    fn a_long_name_is_cut_to_fit_every_file_system_and_keeps_its_extension() {
        let long = format!("{}.pdf", "\u{65E5}".repeat(110));
        let name = safe_name(&long);
        assert!(name.len() <= MAX_NAME_BYTES, "{} bytes", name.len());
        assert!(name.ends_with(".pdf"), "{name}");
        let ascii = format!("{} .pdf", "Tide tables ".repeat(20));
        let name = safe_name(&ascii);
        assert!(name.chars().count() <= MAX_NAME_CHARS);
        assert!(name.ends_with(".pdf") && !name.contains(" .pdf"), "{name}");

        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("{e}"),
        };
        let output = dir.path().join("Novel.skrib");
        let mut copy =
            ResearchCopy::beside(&output.to_string_lossy(), &format!("{TITLE_SLOT} research"));
        for _ in 0..2 {
            let copied = copy.copy(&long, b"%PDF");
            assert!(copied.is_ok(), "{copied:?}");
        }
        assert!(copy.place().is_ok());
    }

    #[test]
    fn files_are_staged_then_moved_beside_the_project() {
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("{e}"),
        };
        std::fs::create_dir(dir.path().join("Novel research")).unwrap_or_default();
        let output = dir.path().join("Novel.skrib");
        let mut copy =
            ResearchCopy::beside(&output.to_string_lossy(), &format!("{TITLE_SLOT} research"));
        assert_eq!(
            copy.name, "Novel research 2",
            "a folder already there is never replaced"
        );
        assert_eq!(
            copy.copy("Tides.pdf", b"%PDF").ok().as_deref(),
            Some("Novel research 2/Tides.pdf")
        );
        assert_eq!(
            copy.copy("tides.PDF", b"%PDF").ok().as_deref(),
            Some("Novel research 2/tides (2).PDF"),
            "a name that differs only by case gets a number"
        );
        assert!(
            !dir.path().join("Novel research 2").exists(),
            "staged, not in place yet"
        );
        assert_eq!(
            copy.place().ok().flatten().as_deref(),
            Some("Novel research 2")
        );
        copy.keep();
        assert!(dir.path().join("Novel research 2/Tides.pdf").is_file());
    }

    /// Placed, then the project could not be: the folder goes with it.
    #[test]
    fn a_placed_folder_goes_when_the_project_does_not_follow() {
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("{e}"),
        };
        let output = dir.path().join("Novel.skrib");
        {
            let mut copy =
                ResearchCopy::beside(&output.to_string_lossy(), &format!("{TITLE_SLOT} research"));
            assert!(copy.copy("Tides.pdf", b"%PDF").is_ok());
            assert!(copy.place().is_ok());
            assert!(dir.path().join("Novel research/Tides.pdf").is_file());
        }
        assert_eq!(
            std::fs::read_dir(dir.path())
                .map(|d| d.count())
                .unwrap_or(1),
            0
        );
    }

    #[test]
    fn an_import_that_does_not_finish_leaves_no_folder() {
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("{e}"),
        };
        let output = dir.path().join("Novel.skrib");
        {
            let mut copy =
                ResearchCopy::beside(&output.to_string_lossy(), &format!("{TITLE_SLOT} research"));
            assert!(copy.copy("Tides.pdf", b"%PDF").is_ok());
        }
        assert_eq!(
            std::fs::read_dir(dir.path())
                .map(|d| d.count())
                .unwrap_or(1),
            0,
            "nothing is left beside the project"
        );
    }
}
