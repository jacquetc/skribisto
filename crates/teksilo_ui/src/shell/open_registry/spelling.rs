// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One key for every spelling of a path, in the rules of the filesystem that reads it.
//!
//! # Why this exists
//!
//! The open registry compares the path a door is about to open or write with the
//! paths held open or being written, and two spellings of one file must compare
//! equal or the guard does not guard. `std::fs::canonicalize` settles that for a file
//! that exists, but the paths this registry cares most about do not exist yet: the
//! `.skrib` an import is writing appears only when it finishes. On Windows the import
//! forms spelled that target `C:\Books/Novel.skrib` while a file dialog says
//! `C:\Books\Novel.skrib`, NTFS does not tell `Novel` from `novel`, and
//! `canonicalize` answers `\\?\C:\Books\Novel.skrib` for the file once it is there.
//! Compared as strings, each of those was another project, so New Work, Save As or a
//! load went ahead into the file the import then replaced.
//!
//! [`spelling_key`] is the textual half of the answer: separators, a verbatim
//! prefix, repeated or trailing separators, `.` components and case, each in the
//! rules of one [`PathStyle`]. The filesystem half, which folder a symbolic link or
//! a short name leads to, is the caller's, before it asks for a key.
//!
//! The rules are the platform's, not the running host's, so a test on Linux can
//! prove what a Windows machine compares.

/// How a platform's filesystem spells paths and compares names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PathStyle {
    /// Linux and the other Unixes: `/` separates, and names are compared exactly.
    Unix,
    /// macOS: `/` separates, and names are compared without regard to case, as APFS
    /// and HFS+ do unless a volume was formatted otherwise.
    Mac,
    /// Windows: `\` and `/` both separate, `\\?\` is another spelling of the path it
    /// prefixes, and names are compared without regard to case, as NTFS does.
    Windows,
}

impl PathStyle {
    /// The rules of the platform this build runs on.
    pub(crate) const HOST: PathStyle = if cfg!(windows) {
        PathStyle::Windows
    } else if cfg!(target_os = "macos") {
        PathStyle::Mac
    } else {
        PathStyle::Unix
    };

    fn separator(self) -> char {
        match self {
            PathStyle::Windows => '\\',
            PathStyle::Unix | PathStyle::Mac => '/',
        }
    }

    /// Whether two names differing only in case are one file.
    ///
    /// On a case-insensitive filesystem a key that kept the case would call one
    /// file two, and a guard would let a second writer in: the project is lost. A
    /// key that folds case on a volume that happens to be case-sensitive only
    /// refuses a target the writer can rename. So the platform's default decides,
    /// and the error it risks is the recoverable one.
    fn folds_case(self) -> bool {
        match self {
            PathStyle::Unix => false,
            PathStyle::Mac | PathStyle::Windows => true,
        }
    }
}

/// The key every spelling of `path` shares under `style`.
///
/// Purely textual: `..` is left as written, since only the filesystem knows
/// whether the name before it is a symbolic link. Callers resolve what exists
/// before asking.
pub(crate) fn spelling_key(path: &str, style: PathStyle) -> String {
    let separator = style.separator();
    let spelled = match style {
        PathStyle::Windows => without_verbatim_prefix(&path.replace('/', "\\")),
        PathStyle::Unix | PathStyle::Mac => path.to_string(),
    };
    let (root, rest) = split_root(&spelled, style);
    let names: Vec<&str> = rest
        .split(separator)
        .filter(|name| !name.is_empty() && *name != ".")
        .collect();
    let key = format!("{root}{}", names.join(&separator.to_string()));
    if style.folds_case() {
        key.to_lowercase()
    } else {
        key
    }
}

/// `\\?\C:\…` as `C:\…`, and `\\?\UNC\server\share\…` as `\\server\share\…`.
///
/// The verbatim prefix `std::fs::canonicalize` puts on every path it returns on
/// Windows changes how Windows parses what follows, not which file it names.
fn without_verbatim_prefix(path: &str) -> String {
    let Some(rest) = path.strip_prefix(r"\\?\") else {
        return path.to_string();
    };
    match rest.get(..4) {
        Some(unc) if unc.eq_ignore_ascii_case(r"UNC\") => format!(r"\\{}", &rest[4..]),
        _ => rest.to_string(),
    }
}

/// The root `path` starts with, kept whole since it is not a name, and the rest.
///
/// Windows has four: a share (`\\`), a drive's root (`C:\`), a drive's current
/// folder (`C:`, which names a different place than `C:\`) and the current drive's
/// root (`\`). The others have one, `/`.
fn split_root(path: &str, style: PathStyle) -> (&str, &str) {
    if style == PathStyle::Windows {
        if path.starts_with(r"\\") {
            return path.split_at(2);
        }
        let bytes = path.as_bytes();
        if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
            let end = if bytes.get(2) == Some(&b'\\') { 3 } else { 2 };
            return path.split_at(end);
        }
    }
    let separator = style.separator();
    if path.starts_with(separator) {
        return path.split_at(separator.len_utf8());
    }
    ("", path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn same(a: &str, b: &str, style: PathStyle) -> bool {
        spelling_key(a, style) == spelling_key(b, style)
    }

    /// Every spelling a Windows door hands over for one file is one key: a file
    /// dialog's, the one the import forms built with a `/`, the verbatim form
    /// `canonicalize` returns, another case, a doubled or trailing separator and a
    /// `.` component.
    #[test]
    fn every_windows_spelling_of_one_file_is_one_key() {
        let dialog = r"C:\Users\writer\Documents\Novel.skrib";
        for other in [
            r"C:\Users\writer\Documents/Novel.skrib",
            "C:/Users/writer/Documents/Novel.skrib",
            r"\\?\C:\Users\writer\Documents\Novel.skrib",
            r"c:\users\WRITER\documents\novel.SKRIB",
            r"C:\Users\writer\\Documents\Novel.skrib\",
            r"C:\Users\writer\.\Documents\Novel.skrib",
        ] {
            assert!(same(dialog, other, PathStyle::Windows), "{other}");
        }
    }

    /// A project on a network share, in its three spellings.
    #[test]
    fn a_share_is_one_key_whether_verbatim_or_not() {
        let share = r"\\server\books\Novel.skrib";
        for other in [
            r"\\?\UNC\server\books\Novel.skrib",
            r"\\?\unc\SERVER\books\novel.skrib",
            "//server/books/Novel.skrib",
        ] {
            assert!(same(share, other, PathStyle::Windows), "{other}");
        }
        assert!(!same(
            share,
            r"C:\server\books\Novel.skrib",
            PathStyle::Windows
        ));
    }

    /// What names different files stays different: another drive, another folder,
    /// and a drive's current folder against its root.
    #[test]
    fn different_windows_files_stay_different() {
        let file = r"C:\Books\Novel.skrib";
        for other in [
            r"D:\Books\Novel.skrib",
            r"C:\Drafts\Novel.skrib",
            r"C:Books\Novel.skrib",
            r"\Books\Novel.skrib",
            r"Books\Novel.skrib",
        ] {
            assert!(!same(file, other, PathStyle::Windows), "{other}");
        }
    }

    /// On Linux a backslash is a character of a name and case tells names apart.
    #[test]
    fn a_unix_key_keeps_backslashes_and_case() {
        let file = "/home/writer/Books/Novel.skrib";
        for other in [
            "/home/writer//Books/Novel.skrib",
            "/home/writer/./Books/Novel.skrib/",
        ] {
            assert!(same(file, other, PathStyle::Unix), "{other}");
        }
        for other in [
            r"/home/writer/Books\Novel.skrib",
            "/home/writer/Books/novel.skrib",
            "home/writer/Books/Novel.skrib",
        ] {
            assert!(!same(file, other, PathStyle::Unix), "{other}");
        }
    }

    /// On macOS case does not tell names apart, and a backslash is still a name's.
    #[test]
    fn a_mac_key_folds_case_and_keeps_backslashes() {
        let file = "/Users/writer/Books/Novel.skrib";
        assert!(same(
            file,
            "/users/writer/books/NOVEL.skrib",
            PathStyle::Mac
        ));
        assert!(!same(
            file,
            r"/Users/writer/Books\Novel.skrib",
            PathStyle::Mac
        ));
    }

    /// A key is itself a spelling of the path, so keying it again changes nothing.
    #[test]
    fn a_key_is_its_own_key() {
        for (path, style) in [
            (r"\\?\C:\Books/Novel.skrib\", PathStyle::Windows),
            (r"\\?\UNC\server\books\Novel.skrib", PathStyle::Windows),
            ("/Users/writer//Novel.skrib", PathStyle::Mac),
            ("/home/writer/./Novel.skrib", PathStyle::Unix),
        ] {
            let once = spelling_key(path, style);
            assert_eq!(spelling_key(&once, style), once, "{path}");
        }
    }
}
