// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a file out of a folder someone else made, without following a link and
//! without blocking.
//!
//! A project folder an importer walks may have come out of an archive, and an
//! archive can hold links. Following one reads whatever it points at: a link named
//! like a scene pointing at `~/.ssh/id_rsa` imports the key as prose, one pointing
//! at `/dev/zero` reads until memory runs out, and a FIFO blocks the read forever,
//! beyond the reach of the cancel button. Every importer that reads a folder opens
//! its files through [`open_regular`](crate::regular_file::open_regular), so none of
//! that can happen.
//!
//! The file the writer picks in the import panel is opened through it too, after one
//! step. A link there was picked on purpose (a shortcut to a project kept elsewhere),
//! so the importer resolves the path first, following that link once, and opens what
//! it resolves to here. A FIFO or a device that the picked name stands for, or
//! resolves to, is then refused as anywhere else, before a byte is read, and the
//! read stops at a limit (see [`read_limited`](crate::regular_file::read_limited)).
//!
//! # Unix and Windows
//!
//! On Unix the file is opened with `O_NOFOLLOW | O_NONBLOCK`: a link at the path
//! fails the open, and a FIFO opens without waiting for a writer. What was opened
//! is then checked through the open handle, so a FIFO, a socket or a device is
//! refused before a byte is read.
//!
//! Windows has no such flag in the standard library, and opening a path there
//! follows a link. So the path is checked first with `symlink_metadata`, and the
//! open handle is checked again after. A link put in place between the check and
//! the open is still followed; a folder read for an import is not changing under
//! the reader in any case a writer meets, and a FIFO or a device cannot sit in a
//! Windows folder.

use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::path::Path;

/// The error [`open_regular`] and [`read_regular`] return for something that is
/// not a regular file (a link, a folder, a FIFO, a socket, a device), told apart
/// from the system's own errors by this payload: see [`is_not_a_regular_file`].
#[derive(Debug)]
pub struct NotARegularFile;

impl std::fmt::Display for NotARegularFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("not a regular file")
    }
}

impl std::error::Error for NotARegularFile {}

/// An [`io::Error`] carrying [`NotARegularFile`].
pub fn not_a_regular_file() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, NotARegularFile)
}

/// Whether `error` is the refusal of something that is not a regular file.
pub fn is_not_a_regular_file(error: &io::Error) -> bool {
    error
        .get_ref()
        .is_some_and(|inner| inner.is::<NotARegularFile>())
}

/// Open `path` for reading without following a link or blocking, and make sure
/// what was opened is a regular file.
///
/// A link at `path` (on Unix only the last part of the path is checked, as
/// `O_NOFOLLOW` does) and anything that is not a regular file return
/// [`NotARegularFile`]; on Unix a link fails the open itself, with the system's
/// own error (`ELOOP`), which is reported as [`NotARegularFile`] as well.
pub fn open_regular(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(not(unix))]
    {
        // Windows opens a link's target; check what is at the path first.
        let md = std::fs::symlink_metadata(path)?;
        if !md.file_type().is_file() {
            return Err(not_a_regular_file());
        }
    }
    let file = match options.open(path) {
        Ok(file) => file,
        Err(e) if opened_a_link(&e) => return Err(not_a_regular_file()),
        Err(e) => return Err(e),
    };
    if !file.metadata()?.file_type().is_file() {
        return Err(not_a_regular_file());
    }
    Ok(file)
}

/// Whether an open with `O_NOFOLLOW` failed because the path is a link. Linux and
/// macOS say `ELOOP`; FreeBSD says `EMLINK`.
#[cfg(unix)]
fn opened_a_link(e: &io::Error) -> bool {
    matches!(e.raw_os_error(), Some(code) if code == libc::ELOOP || code == libc::EMLINK)
}

#[cfg(not(unix))]
fn opened_a_link(_e: &io::Error) -> bool {
    false
}

/// Read the regular file at `path`, at most `limit` bytes of it, as
/// [`open_regular`] opens it.
///
/// The limit holds before the bytes are in memory: one byte past it is read, and
/// a file that has it returns [`io::ErrorKind::FileTooLarge`].
pub fn read_regular(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    read_limited(open_regular(path)?, limit)
}

/// Read `file` to its end, at most `limit` bytes of it, as [`read_regular`] does.
pub fn read_limited(file: File, limit: u64) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    file.take(limit.saturating_add(1)).read_to_end(&mut out)?;
    if out.len() as u64 > limit {
        return Err(io::Error::from(io::ErrorKind::FileTooLarge));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_regular_file_is_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("scene.md");
        std::fs::write(&path, "Words.").unwrap();
        assert_eq!(read_regular(&path, 100).unwrap(), b"Words.");
        // Exactly at the limit is still within it.
        assert_eq!(read_regular(&path, 6).unwrap(), b"Words.");
    }

    #[test]
    fn a_file_past_the_limit_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("scene.md");
        std::fs::write(&path, "Words.").unwrap();
        let e = read_regular(&path, 5).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::FileTooLarge);
    }

    #[test]
    fn a_folder_is_not_a_regular_file() {
        let dir = tempfile::tempdir().unwrap();
        let e = read_regular(dir.path(), 100).unwrap_err();
        assert!(is_not_a_regular_file(&e), "{e:?}");
    }

    #[cfg(unix)]
    #[test]
    fn a_link_to_a_file_is_not_followed() {
        let dir = tempfile::tempdir().unwrap();
        let secret = dir.path().join("id_rsa");
        std::fs::write(&secret, "PRIVATE KEY").unwrap();
        let link = dir.path().join("scene.md");
        std::os::unix::fs::symlink(&secret, &link).unwrap();
        let e = read_regular(&link, 100).unwrap_err();
        assert!(is_not_a_regular_file(&e), "{e:?}");
    }

    /// `/dev/zero` never ends: followed, a read of it fills memory.
    #[cfg(unix)]
    #[test]
    fn a_device_is_refused_before_a_byte_is_read() {
        let dir = tempfile::tempdir().unwrap();
        let link = dir.path().join("scene.md");
        std::os::unix::fs::symlink("/dev/zero", &link).unwrap();
        assert!(is_not_a_regular_file(
            &read_regular(&link, 100).unwrap_err()
        ));
        assert!(is_not_a_regular_file(
            &read_regular(Path::new("/dev/zero"), 100).unwrap_err()
        ));
    }

    /// A FIFO with no writer would block a plain open forever.
    #[cfg(unix)]
    #[test]
    fn a_fifo_is_refused_without_blocking() {
        let dir = tempfile::tempdir().unwrap();
        let fifo = dir.path().join("scene.md");
        let made = std::process::Command::new("mkfifo").arg(&fifo).status();
        if !made.is_ok_and(|s| s.success()) {
            eprintln!("mkfifo is not available here; skipping");
            return;
        }
        let e = read_regular(&fifo, 100).unwrap_err();
        assert!(is_not_a_regular_file(&e), "{e:?}");
    }
}
