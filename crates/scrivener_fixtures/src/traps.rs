// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The names real projects are found with: sync services' copies, cloud
//! placeholders, Scrivener's own backups.
//!
//! Each scheme is written the way the service writes it, as observed in real
//! projects or documented by the service: Dropbox's localised "conflicted copy"
//! with a date, the ISO-stamped `-conflicted-` copies, `[Conflict]`, Syncthing,
//! Resilio, git merge leftovers, numbered and iCloud-style copies, and OneDrive's
//! machine-name suffix.

/// A sync service's way of naming the copy it keeps of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyScheme {
    Dropbox,
    DropboxLocalised,
    IsoConflicted,
    BracketConflict,
    Syncthing,
    Resilio,
    GitMerge,
    Numbered,
    Underscore,
    SpaceNumber,
    /// OneDrive: `name-MACHINE.ext`.
    OneDrive,
}

impl CopyScheme {
    pub const ALL: [CopyScheme; 11] = [
        CopyScheme::Dropbox,
        CopyScheme::DropboxLocalised,
        CopyScheme::IsoConflicted,
        CopyScheme::BracketConflict,
        CopyScheme::Syncthing,
        CopyScheme::Resilio,
        CopyScheme::GitMerge,
        CopyScheme::Numbered,
        CopyScheme::Underscore,
        CopyScheme::SpaceNumber,
        CopyScheme::OneDrive,
    ];
}

/// The name the scheme gives a copy of `name`.
pub fn copy_name(name: &str, scheme: CopyScheme) -> String {
    let (stem, ext) = match name.rfind('.') {
        Some(0) | None => (name, ""),
        Some(dot) => (&name[..dot], &name[dot..]),
    };
    match scheme {
        CopyScheme::Dropbox => format!("{stem} (Test Laptop's conflicted copy 2024-05-01){ext}"),
        CopyScheme::DropboxLocalised => {
            format!("{stem} (C\u{f3}pia em conflito de Test Laptop 2024-05-01){ext}")
        }
        CopyScheme::IsoConflicted => format!("{stem}-conflicted-2024-05-01T10-11-12-0500{ext}"),
        CopyScheme::BracketConflict => format!("{stem}[Conflict]{ext}"),
        CopyScheme::Syncthing => format!("{stem}.sync-conflict-20240501-101112-ABCDEF7{ext}"),
        CopyScheme::Resilio => format!("{stem}.Conflict{ext}"),
        CopyScheme::GitMerge => format!("{name}~HEAD"),
        CopyScheme::Numbered => format!("{stem} (1){ext}"),
        CopyScheme::Underscore => format!("{stem}_2{ext}"),
        CopyScheme::SpaceNumber => format!("{stem} 2{ext}"),
        CopyScheme::OneDrive => format!("{stem}-TESTLAPTOP{ext}"),
    }
}

/// The hidden stub iCloud leaves for an evicted file (macOS 13 and earlier).
pub fn icloud_stub(name: &str) -> String {
    format!(".{name}.icloud")
}

/// The virtual file Nextcloud leaves for a file not yet downloaded.
pub fn nextcloud_stub(name: &str) -> String {
    format!("{name}.nextcloud")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_scheme_decorates_the_name() {
        for scheme in CopyScheme::ALL {
            assert_ne!(
                copy_name("content.rtf", scheme),
                "content.rtf",
                "{scheme:?}"
            );
        }
    }
}
