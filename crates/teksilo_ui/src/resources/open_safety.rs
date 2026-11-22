// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which files a click may hand to the desktop without asking first.
//!
//! Opening a resource hands its file to whatever the desktop opens that kind of file
//! with, and the desktop decides by the file's name. A shared project is someone else's
//! file: a `.desktop` entry, a shell script, an `.exe`, an `.app`, a Java archive or a
//! web page holding scripts would *run* on that click rather than be shown. So a short
//! list of document types opens at once, and everything else asks first and offers to
//! show the file in its folder instead, where the writer can see what it is.
//!
//! An allow-list, not a deny-list: a deny-list is right on the day it is written and
//! wrong the first time a desktop learns to run a new kind of file. The cost of the
//! allow-list is one question for a harmless file nobody thought of, which is the cheap
//! side to err on.
//!
//! The extension is the only input, and it is read the way Windows reads it (trailing
//! dots and spaces ignored), so `notes.pdf.exe` and `notes.exe.` both ask. A media type
//! is never consulted: a project stores it, and a project can say anything.
//!
//! The name judged is the name of the file the desktop gets, which for a symbolic link is
//! the file it points at (`ResourcesViewModel`'s `name_to_judge`): macOS opens what a link
//! points at, so `map.pdf` pointing at `run.command` ran it in a terminal. And a stored
//! file must be the project's own: one whose link leads out of its `resources/<uid>/`
//! folder is not opened at all ([`leaves_its_folder`]).
//!
//! Not a view-model: two predicates.

use std::path::Path;

use super::file_kind::extension;

/// What a click on a resource's file does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenVerdict {
    /// A document type: open it.
    Open,
    /// An executable, a script, a web page or an unknown type: ask first.
    Ask,
}

/// The extensions that open without a question: documents a viewer shows and does not
/// run. Office files are here because their viewers ask before running a macro, and
/// the macro-enabled spellings (`docm`, `xlsm`, …) are not.
const OPEN_WITHOUT_ASKING: &[&str] = &[
    // Documents.
    "pdf", "epub", // Pictures (not SVG: a picture made of markup can carry script).
    "png", "jpg", "jpeg", "gif", "webp", "tif", "tiff", "bmp", "heic", "heif", "avif",
    // Sound.
    "mp3", "m4a", "aac", "ogg", "oga", "opus", "flac", "wav", "aiff", "aif", // Video.
    "mp4", "m4v", "mov", "webm", "mkv", "avi", // Text (not HTML: a page can run script).
    "txt", "text", "md", "markdown", "rtf", "csv",
    // Office documents, without their macro-enabled forms.
    "doc", "docx", "odt", "ods", "odp", "xls", "xlsx", "ppt", "pptx", "pages", "numbers", "key",
];

/// Whether a click opens the file called `file_name` directly, or asks first.
pub fn verdict(file_name: &str) -> OpenVerdict {
    let ext = extension(file_name);
    if OPEN_WITHOUT_ASKING.contains(&ext.as_str()) {
        OpenVerdict::Open
    } else {
        OpenVerdict::Ask
    }
}

/// Whether the stored file at `path`, in its `resources/<uid>/` folder, is a symbolic
/// link out of that folder, or sits in a folder that is itself a link. Such a file is
/// not the project's: a shared folder project can point `map.pdf` at a script beside it
/// or at any file on the disk, and opening it hands that file over. A link to another
/// file of the same folder stays, judged by the name it leads to. `false` for a path
/// that is no link at all, and for one that is not there (it is missing, and says so).
pub fn leaves_its_folder(path: &Path) -> bool {
    let is_link =
        |p: &Path| std::fs::symlink_metadata(p).is_ok_and(|meta| meta.file_type().is_symlink());
    let Some(folder) = path.parent() else {
        return false;
    };
    if is_link(folder) {
        return true;
    }
    if !is_link(path) {
        return false;
    }
    match (std::fs::canonicalize(folder), std::fs::canonicalize(path)) {
        (Ok(folder), Ok(target)) => target.parent() != Some(folder.as_path()),
        // A link to nothing opens nothing; one whose folder cannot be read is not judged
        // to be inside it.
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documents_pictures_sound_and_video_open_without_asking() {
        for name in [
            "Village map.pdf",
            "Lighthouse.JPG",
            "Keeper interview.mp3",
            "Harbour.mov",
            "Notes.md",
            "Letter.rtf",
            "Chapter plan.docx",
            "Budget.ods",
            "Ship's log.epub",
        ] {
            assert_eq!(verdict(name), OpenVerdict::Open, "{name}");
        }
    }

    #[test]
    fn anything_that_could_run_asks_first() {
        for name in [
            "launcher.desktop",
            "setup.sh",
            "Run me.command",
            "Tool.app",
            "installer.exe",
            "fix.bat",
            "fix.cmd",
            "shortcut.lnk",
            "tool.jar",
            "page.html",
            "page.htm",
            "drawing.svg",
            "macro.docm",
            "sheet.xlsm",
            "script.ps1",
            "script.vbs",
            "archive.zip",
            "README",
            "",
        ] {
            assert_eq!(verdict(name), OpenVerdict::Ask, "{name}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_stored_file_linked_out_of_its_folder_is_not_the_projects() {
        let dir = tempfile::tempdir().expect("tmp");
        let resources = dir.path().join("resources");
        let folder = resources.join("uid-1");
        std::fs::create_dir_all(&folder).expect("folder");
        let elsewhere = dir.path().join("elsewhere");
        std::fs::create_dir_all(&elsewhere).expect("folder");
        std::fs::write(elsewhere.join("run.command"), b"#!/bin/sh").expect("write");
        std::fs::write(folder.join("notes.txt"), b"notes").expect("write");
        std::fs::write(folder.join("other.txt"), b"other").expect("write");

        // Its own file, and a link to another file of its own folder.
        assert!(!leaves_its_folder(&folder.join("notes.txt")));
        std::os::unix::fs::symlink(folder.join("other.txt"), folder.join("alias.txt"))
            .expect("link");
        assert!(!leaves_its_folder(&folder.join("alias.txt")));
        // A link out of the folder, relative or absolute, and a link to nothing.
        std::os::unix::fs::symlink("../../elsewhere/run.command", folder.join("map.pdf"))
            .expect("link");
        assert!(leaves_its_folder(&folder.join("map.pdf")));
        std::os::unix::fs::symlink(elsewhere.join("run.command"), folder.join("abs.pdf"))
            .expect("link");
        assert!(leaves_its_folder(&folder.join("abs.pdf")));
        std::os::unix::fs::symlink(folder.join("gone"), folder.join("dangling.pdf")).expect("link");
        assert!(leaves_its_folder(&folder.join("dangling.pdf")));
        // A `<uid>` folder that is a link: whatever is in it is elsewhere.
        std::os::unix::fs::symlink(&elsewhere, resources.join("uid-2")).expect("link");
        assert!(leaves_its_folder(
            &resources.join("uid-2").join("run.command")
        ));
        // Missing, not linked.
        assert!(!leaves_its_folder(&folder.join("never.pdf")));
    }

    #[test]
    fn a_disguised_name_is_read_by_its_last_extension() {
        assert_eq!(verdict("map.pdf.exe"), OpenVerdict::Ask);
        assert_eq!(verdict("map.exe."), OpenVerdict::Ask);
        assert_eq!(verdict("map.exe .. "), OpenVerdict::Ask);
        assert_eq!(verdict("archive.exe.pdf"), OpenVerdict::Open);
    }
}
