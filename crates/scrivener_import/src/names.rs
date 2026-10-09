// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Names: how a member's name is normalised for lookup, which archive member
//! names are refused, and how the copies sync services leave behind are named.
//!
//! # Lookup keys
//!
//! A member is found exactly first, then by its NFC form (HFS+ hands over
//! decomposed names), then by its NFC form case-folded (Scrivener for Mac writes
//! `docs.checksum` with lower-case UUIDs against upper-case folders, and the Linux
//! beta and Wine write on case-sensitive disks). A case-folded key that two members
//! share resolves to neither: guessing between `Content.rtf` and `content.rtf` would
//! be choosing a document at random.
//!
//! # Sync copies
//!
//! A decorated name is a copy only when its undecorated original is also there; a
//! project whose own name ends in " 2" is not a copy of anything. The schemes are
//! recognised by shape, never by an English word: Dropbox writes "conflicted copy"
//! in the writer's language.

use unicode_normalization::UnicodeNormalization;

use crate::model::ConflictScheme;
use crate::outcome::UnsafeName;

/// The NFC form of a name.
pub(crate) fn nfc(name: &str) -> String {
    name.nfc().collect()
}

/// The NFC form of a name, case-folded.
pub(crate) fn folded(name: &str) -> String {
    nfc(name).to_lowercase()
}

/// Why an archive member's name cannot be taken as a path inside the project,
/// or `None` when it can. `name` has already had `\` turned into `/`.
pub(crate) fn unsafe_member_name(name: &str) -> Option<UnsafeName> {
    if name.is_empty() {
        return Some(UnsafeName::Empty);
    }
    if name
        .chars()
        .any(|c| c == '\0' || (c.is_control() && c != '\r'))
    {
        return Some(UnsafeName::ControlCharacter);
    }
    let bytes = name.as_bytes();
    let drive = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    if name.starts_with('/') || drive {
        return Some(UnsafeName::Absolute);
    }
    if name.split('/').any(|part| part == "..") {
        return Some(UnsafeName::ParentFolder);
    }
    None
}

/// Archive members dropped without a word: what Finder and Windows add to any
/// archive they make.
pub(crate) fn is_archive_litter(path: &str) -> bool {
    let first = path.split('/').next().unwrap_or_default();
    if first == "__MACOSX" {
        return true;
    }
    let base = path.rsplit('/').next().unwrap_or_default();
    base.starts_with("._") || is_os_litter(base)
}

/// Files an operating system leaves in any folder.
pub(crate) fn is_os_litter(base: &str) -> bool {
    matches!(base, ".DS_Store" | "Icon\r")
        || base.eq_ignore_ascii_case("thumbs.db")
        || base.eq_ignore_ascii_case("desktop.ini")
}

/// Litter of any kind: the operating system's, AppleDouble halves, office lock
/// files, version control files.
pub(crate) fn is_litter(base: &str) -> bool {
    is_os_litter(base)
        || base.starts_with("._")
        || (base.starts_with(".~lock.") && base.ends_with('#'))
        || base.starts_with(".git")
        || base == ".svn"
        || base.starts_with(".hg")
}

/// A name-based cloud placeholder, and the name of the file it stands for:
/// `.name.icloud` (iCloud before macOS 14), `name.nextcloud`, `name.owncloud`.
pub(crate) fn placeholder_target(base: &str) -> Option<String> {
    if let Some(inner) = base
        .strip_prefix('.')
        .and_then(|rest| rest.strip_suffix(".icloud"))
        && !inner.is_empty()
    {
        return Some(inner.to_string());
    }
    for suffix in [".nextcloud", ".owncloud"] {
        if let Some(inner) = base.strip_suffix(suffix)
            && !inner.is_empty()
        {
            return Some(inner.to_string());
        }
    }
    None
}

/// A decorated file name and the name it decorates, when `base` has the shape
/// of a sync copy. Whether the original exists is the caller's question.
pub(crate) fn conflict_original(base: &str) -> Option<(ConflictScheme, String)> {
    let (stem, ext) = split_ext(base);
    // Dropbox: "stem (… conflicted copy 2016-03-22).ext", localised: the
    // parenthesis holds a date whatever the language.
    if let Some(open) = stem.rfind(" (")
        && stem.ends_with(')')
    {
        let inner = &stem[open + 2..stem.len() - 1];
        if contains_iso_date(inner) {
            return Some((ConflictScheme::Dropbox, format!("{}{ext}", &stem[..open])));
        }
    }
    // "stem-conflicted-2018-02-14T09-31-27-0500.ext"
    if let Some(at) = stem.find("-conflicted-") {
        let rest = &stem[at + "-conflicted-".len()..];
        if starts_with_iso_date(rest) && rest.as_bytes().get(10) == Some(&b'T') && at > 0 {
            return Some((
                ConflictScheme::IsoConflicted,
                format!("{}{ext}", &stem[..at]),
            ));
        }
    }
    // "stem[Conflict].ext", "stem[Conflict 1].ext"
    if let Some(open) = stem.rfind("[Conflict")
        && stem.ends_with(']')
        && open > 0
    {
        let inner = &stem[open + "[Conflict".len()..stem.len() - 1];
        if inner.is_empty() || (inner.starts_with(' ') && is_digits(&inner[1..])) {
            return Some((
                ConflictScheme::BracketConflict,
                format!("{}{ext}", &stem[..open]),
            ));
        }
    }
    // Syncthing: "stem.sync-conflict-20240101-101010-ABCDEF7.ext"
    if let Some(at) = stem.find(".sync-conflict-")
        && at > 0
    {
        let rest = &stem[at + ".sync-conflict-".len()..];
        let parts: Vec<&str> = rest.splitn(3, '-').collect();
        if parts.len() == 3
            && parts[0].len() == 8
            && is_digits(parts[0])
            && parts[1].len() == 6
            && is_digits(parts[1])
            && !parts[2].is_empty()
            && parts[2]
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            return Some((ConflictScheme::Syncthing, format!("{}{ext}", &stem[..at])));
        }
    }
    // Resilio: "stem.Conflict.ext"
    if let Some(inner) = stem.strip_suffix(".Conflict")
        && !inner.is_empty()
    {
        return Some((ConflictScheme::Resilio, format!("{inner}{ext}")));
    }
    // git: "name.ext~HEAD", "name.ext~0a9cefd9…"
    if let Some(tilde) = base.rfind('~') {
        let tail = &base[tilde + 1..];
        let hex =
            tail.len() >= 7 && tail.len() <= 40 && tail.chars().all(|c| c.is_ascii_hexdigit());
        if (tail == "HEAD" || hex) && tilde > 0 {
            return Some((ConflictScheme::GitMerge, base[..tilde].to_string()));
        }
    }
    // "stem (1).ext"
    if let Some(open) = stem.rfind(" (")
        && stem.ends_with(')')
        && open > 0
        && is_digits(&stem[open + 2..stem.len() - 1])
    {
        return Some((ConflictScheme::Numbered, format!("{}{ext}", &stem[..open])));
    }
    // "stem_2.ext" and "stem 2.ext": only with an extension, as the prototype
    // measured them; a bare "Chapter_2" is a name, not a copy.
    if !ext.is_empty() {
        if let Some(under) = stem.rfind('_')
            && under > 0
            && is_digits(&stem[under + 1..])
        {
            return Some((
                ConflictScheme::Underscore,
                format!("{}{ext}", &stem[..under]),
            ));
        }
        if let Some(space) = stem.rfind(' ')
            && space > 0
            && is_digits(&stem[space + 1..])
        {
            return Some((
                ConflictScheme::SpaceNumber,
                format!("{}{ext}", &stem[..space]),
            ));
        }
    }
    None
}

/// A project folder's or archive's name as a title: without its extensions (`.scriv`,
/// `.zip`, `.scrivtemplate`), and without what a backup or a sync service added to it.
///
/// Removed: Scrivener's backup marks (` Backup` on the copy it keeps before upgrading a
/// project, so only when `upgradable` says the project is of a format Scrivener upgrades;
/// `.bak`/`.bak3` on a zipped backup, a `[2025_06_01_05_14_50]` stamp), the conflicted-copy
/// notes of the sync services that write one (Dropbox's names its person and computer), the
/// stamp of a Google Drive download, and a computer's name OneDrive joins to a copy's: one
/// of `hosts`, the computers the project names, or a name Windows gave a computer itself.
/// Decorations that could be the writer's own (`Novel (2)`, `Novel 2`, `The Backup` in a
/// current project, a title ending with a word that is also a computer's name) are kept:
/// without the original beside it, nothing says they are a copy's.
pub(crate) fn title_from_name(name: &str, hosts: &[String], upgradable: bool) -> String {
    let hosts: Vec<String> = hosts
        .iter()
        .filter_map(|h| h.trim().split('.').next().map(str::to_lowercase))
        .filter(|h| !h.is_empty())
        .collect();
    let mut current = name.trim().to_string();
    // Each pass removes one decoration; a name holds a handful at most.
    for _ in 0..16 {
        let next = undecorated(&current, &hosts, upgradable);
        if next == current || next.trim().is_empty() {
            break;
        }
        current = next;
    }
    current.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `name` with its outermost decoration removed, or unchanged.
fn undecorated(name: &str, hosts: &[String], upgradable: bool) -> String {
    // ASCII case only, so every byte offset in `lower` is one in `name`.
    let lower = name.to_ascii_lowercase();
    for extension in [".scrivtemplate", ".scriv", ".zip"] {
        if lower.ends_with(extension) && name.len() > extension.len() {
            return name[..name.len() - extension.len()].trim_end().to_string();
        }
    }
    // `.bak`, `.bak3`: a zipped backup.
    if let Some(at) = lower.rfind(".bak")
        && at > 0
        && lower[at + 4..].bytes().all(|b| b.is_ascii_digit())
    {
        return name[..at].trim_end().to_string();
    }
    // ` Backup`: the copy Scrivener keeps before upgrading a project, which is of the
    // format it upgrades from.
    if upgradable
        && let Some(stem) = lower.strip_suffix(" backup")
        && !stem.trim().is_empty()
    {
        return name[..stem.len()].trim_end().to_string();
    }
    // ` [2025_06_01_05_14_50]`: a timestamped backup.
    if lower.len() >= 21
        && lower.ends_with(']')
        && let Some(open) = lower.rfind('[')
        && lower.len() - open == 21
        && lower[open + 1..lower.len() - 1]
            .bytes()
            .enumerate()
            .all(|(j, b)| match j {
                4 | 7 | 10 | 13 | 16 => b == b'_',
                _ => b.is_ascii_digit(),
            })
    {
        return name[..open].trim_end().to_string();
    }
    // `-20260319T124826Z-1-001`: the archive Google Drive makes of a folder downloaded.
    if let Some(stem) = drive_download_stem(name) {
        return stem.trim_end().to_string();
    }
    // `_conflict-20140331-100100`: ownCloud's conflict copy.
    if let Some(at) = lower.rfind("_conflict-")
        && at > 0
        && is_stamp(&lower[at + "_conflict-".len()..], &[8, 6])
    {
        return name[..at].trim_end().to_string();
    }
    // ` (… 2016-03-22 (1))`: a dated conflict note that counts its copies inside.
    if let Some(open) = last_group_open(name)
        && open > 0
        && name[..open].ends_with(' ')
        && contains_iso_date(&name[open..])
    {
        return name[..open].trim_end().to_string();
    }
    // A sync service's conflict note, read as on any file: the name is given an extension
    // for the reading, since a folder's may already have gone.
    if let Some((scheme, original)) = conflict_original(&format!("{name}.x"))
        && matches!(
            scheme,
            ConflictScheme::Dropbox
                | ConflictScheme::IsoConflicted
                | ConflictScheme::BracketConflict
                | ConflictScheme::Syncthing
                | ConflictScheme::Resilio
                | ConflictScheme::GitMerge
        )
        && let Some(stem) = original.strip_suffix(".x")
    {
        return stem.trim_end().to_string();
    }
    // `Novel-DESKTOP-4F2A9QK`: OneDrive's copy, named after a computer Windows named itself.
    if let Some(stem) = windows_computer_stem(name) {
        return stem.trim_end().to_string();
    }
    // `Novel-MACHINE`: OneDrive's copy, named after a computer the project names. Written
    // after a space instead, a name that reads as a word ("Rise of the Phoenix" on a computer
    // called Phoenix) is the writer's own; one only a computer bears goes all the same.
    let chars: Vec<char> = name.chars().collect();
    for host in hosts {
        let wanted = host.chars().count() + 1;
        if chars.len() <= wanted {
            continue;
        }
        let cut = chars.len() - wanted;
        let tail: String = chars[cut + 1..].iter().collect();
        let joined = match chars[cut] {
            '-' => true,
            ' ' | '_' => !host.chars().all(char::is_alphabetic),
            _ => false,
        };
        if joined && tail.to_lowercase() == *host {
            return chars[..cut]
                .iter()
                .collect::<String>()
                .trim_end()
                .to_string();
        }
    }
    name.to_string()
}

/// `name` without the `-YYYYMMDDTHHMMSSZ-N-NNN` Google Drive gives the archive of a folder
/// downloaded, when it has one.
fn drive_download_stem(name: &str) -> Option<&str> {
    let mut parts = name.rsplitn(4, '-');
    let (part, number, stamp) = (parts.next()?, parts.next()?, parts.next()?);
    let stem = parts.next()?;
    let b = stamp.as_bytes();
    let stamped = b.len() == 16
        && b[8] == b'T'
        && b[15] == b'Z'
        && b[..8].iter().chain(&b[9..15]).all(u8::is_ascii_digit);
    (stamped && part.len() == 3 && is_digits(part) && is_digits(number) && !stem.trim().is_empty())
        .then_some(stem)
}

/// Whether `s` is groups of digits of the lengths given, joined by `-`, and nothing else.
fn is_stamp(s: &str, lengths: &[usize]) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    parts.len() == lengths.len()
        && parts
            .iter()
            .zip(lengths)
            .all(|(part, len)| part.len() == *len && is_digits(part))
}

/// Where the parenthesised group closing `name` opens, counting the groups inside it, or
/// `None` when `name` does not end with a whole group.
fn last_group_open(name: &str) -> Option<usize> {
    if !name.ends_with(')') {
        return None;
    }
    let mut depth = 0usize;
    for (at, c) in name.char_indices().rev() {
        match c {
            ')' => depth += 1,
            '(' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// `name` without the `-DESKTOP-XXXXXXX` or `-LAPTOP-XXXXXXXX` OneDrive adds after a copy's
/// name, when the computer kept the name Windows gave it: those words, then seven or eight
/// capitals and digits.
fn windows_computer_stem(name: &str) -> Option<&str> {
    let (rest, code) = name.rsplit_once('-')?;
    let (stem, word) = rest.rsplit_once('-')?;
    let generated = matches!(word, "DESKTOP" | "LAPTOP")
        && (7..=8).contains(&code.len())
        && code
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit());
    (generated && !stem.trim().is_empty()).then_some(stem)
}

/// `name` split before its last `.`, the dot kept with the extension. A leading
/// dot is part of the stem.
pub(crate) fn split_ext(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(0) | None => (name, ""),
        Some(dot) => (&name[..dot], &name[dot..]),
    }
}

fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// `YYYY-MM-DD` at the start of `s`.
fn starts_with_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 10
        && b[..4].iter().all(u8::is_ascii_digit)
        && b[4] == b'-'
        && b[5..7].iter().all(u8::is_ascii_digit)
        && b[7] == b'-'
        && b[8..10].iter().all(u8::is_ascii_digit)
}

/// `YYYY-MM-DD` anywhere in `s`.
fn contains_iso_date(s: &str) -> bool {
    s.char_indices()
        .any(|(i, _)| s.is_char_boundary(i) && starts_with_iso_date(&s[i..]))
}

/// A UUID written the way Scrivener writes one: 8-4-4-4-12 hexadecimal digits.
/// Anything else is refused, even forms `uuid` would accept (braces, no dashes),
/// because an identifier becomes part of a path.
pub(crate) fn is_uuid(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 36
        && b.iter().enumerate().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => *c == b'-',
            _ => c.is_ascii_hexdigit(),
        })
}

/// A numeric binder ID: one to nine digits.
pub(crate) fn is_item_number(s: &str) -> bool {
    !s.is_empty() && s.len() <= 9 && s.bytes().all(|b| b.is_ascii_digit())
}

/// A file extension a binder item may name: one to sixteen ASCII letters and
/// digits (the longest real one is `webarchive`).
pub(crate) fn is_extension(s: &str) -> bool {
    !s.is_empty() && s.len() <= 16 && s.bytes().all(|b| b.is_ascii_alphanumeric())
}

/// A folder name Scrivener uses inside a project, so that a pick of it means
/// the project around it.
pub(crate) fn is_internal_folder(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "files"
            | "data"
            | "docs"
            | "settings"
            | "snapshots"
            | "mobile"
            | "projectnotes"
            | "icons"
            | "quicklook"
    ) || is_uuid(name)
        || lower.ends_with(".snapshots")
        || lower.ends_with(".rtfd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_real_conflict_scheme_names_its_original() {
        let cases = [
            (
                "Coursebook (NOAKE's MacBook Air's conflicted copy 2016-03-22).scrivx",
                ConflictScheme::Dropbox,
                "Coursebook.scrivx",
            ),
            (
                "user (Cópia em conflito de MacBook-Pro 2019-12-29).lock",
                ConflictScheme::Dropbox,
                "user.lock",
            ),
            (
                "Hollows and Kilns-conflicted-2018-02-14T09-31-27-0500.scrivx",
                ConflictScheme::IsoConflicted,
                "Hollows and Kilns.scrivx",
            ),
            (
                "binder[Conflict].autosave",
                ConflictScheme::BracketConflict,
                "binder.autosave",
            ),
            (
                "284[Conflict 1].rtf",
                ConflictScheme::BracketConflict,
                "284.rtf",
            ),
            (
                "content.sync-conflict-20240101-101010-ABCDEF7.rtf",
                ConflictScheme::Syncthing,
                "content.rtf",
            ),
            (
                "content.Conflict.rtf",
                ConflictScheme::Resilio,
                "content.rtf",
            ),
            (
                "binder.backup~HEAD",
                ConflictScheme::GitMerge,
                "binder.backup",
            ),
            (
                "binder.backup~0a9cefd9",
                ConflictScheme::GitMerge,
                "binder.backup",
            ),
            ("201 (1).rtf", ConflictScheme::Numbered, "201.rtf"),
            ("12_2.rtf", ConflictScheme::Underscore, "12.rtf"),
            ("styles 2.xml", ConflictScheme::SpaceNumber, "styles.xml"),
        ];
        for (name, scheme, original) in cases {
            assert_eq!(
                conflict_original(name),
                Some((scheme, original.to_string())),
                "{name}"
            );
        }
    }

    proptest::proptest! {
        /// Whatever a document is called, every sync service's copy of it names it
        /// back, so a copy is never mistaken for a stray file.
        #[test]
        fn every_scheme_s_copy_names_its_original(
            stem in "[a-z][a-z0-9 _()\\[\\]]{0,10}[a-z]",
            ext in "[a-z]{1,5}",
        ) {
            let name = format!("{stem}.{ext}");
            for scheme in scrivener_fixtures::traps::CopyScheme::ALL {
                let copy = scrivener_fixtures::traps::copy_name(&name, scheme);
                if scheme == scrivener_fixtures::traps::CopyScheme::OneDrive {
                    // OneDrive's copies are told apart by the stem rule, inside a
                    // document folder, not by their name alone.
                    continue;
                }
                let original = conflict_original(&copy).map(|(_, original)| original);
                proptest::prop_assert_eq!(original.as_deref(), Some(name.as_str()), "{:?} {}", scheme, copy);
            }
        }
    }

    /// A title made from a folder's name never keeps what a backup or a sync service
    /// wrote into it, and never the name of a person or a computer.
    #[test]
    fn a_folder_name_becomes_a_title_without_its_decorations() {
        let hosts = vec!["Tavs-MacBook.local".to_string(), "DESKTOP-4F2A".to_string()];
        for (name, title) in [
            ("Novel.scriv", "Novel"),
            ("Novel.SCRIV", "Novel"),
            ("Novel Backup.scriv", "Novel"),
            ("Novel.bak3.zip", "Novel"),
            ("Novel.bak.zip", "Novel"),
            ("Novel [2025_06_01_05_14_50].zip", "Novel"),
            ("Novel.scriv.zip", "Novel"),
            ("Starter.scrivtemplate", "Starter"),
            (
                "Novel (Tav's MacBook Air's conflicted copy 2016-03-22).scriv",
                "Novel",
            ),
            (
                "Novel (Cópia em conflito de MacBook-Pro 2019-12-29).scriv",
                "Novel",
            ),
            ("Novel-conflicted-2018-02-14T09-31-27-0500.scriv", "Novel"),
            ("Novel.sync-conflict-20240101-101010-ABCDEF7.scriv", "Novel"),
            ("Novel[Conflict 1].scriv", "Novel"),
            ("Novel-DESKTOP-4F2A.scriv", "Novel"),
            ("Novel tavs-macbook.scriv", "Novel"),
            ("Novel (2).scriv", "Novel (2)"),
            ("Novel 2.scriv", "Novel 2"),
            ("Pride-Prejudice.scriv", "Pride-Prejudice"),
            ("  Two  Words .scriv", "Two Words"),
            (".scriv", ".scriv"),
            ("Backup", "Backup"),
        ] {
            assert_eq!(title_from_name(name, &hosts, true), title, "{name}");
        }
        assert_eq!(
            title_from_name("Ünïcode İstanbul.scriv", &[], true),
            "Ünïcode İstanbul"
        );
    }

    /// The copies whose computer or person no list of this computer's names holds lose
    /// their decoration too: Windows' own computer names, a dated conflict note that counts
    /// its copies, a Google Drive download, an ownCloud conflict.
    #[test]
    fn a_decoration_naming_an_unknown_computer_or_person_goes_too() {
        for (name, title) in [
            ("Novel-DESKTOP-4F2A9QK.scriv", "Novel"),
            ("Novel-LAPTOP-8H3JQ2LM.scriv", "Novel"),
            (
                "Novel (Tav Ormby's conflicted copy 2016-03-22 (1)).scriv",
                "Novel",
            ),
            ("Novel-20260319T124826Z-1-001.zip", "Novel"),
            ("Novel.scriv-20260319T124826Z-1-001.zip", "Novel"),
            ("Novel_conflict-20140331-100100.scriv", "Novel"),
            // A title of its own is kept, whatever it ends with.
            ("Novel (the 2nd draft).scriv", "Novel (the 2nd draft)"),
            ("Novel-DESKTOP.scriv", "Novel-DESKTOP"),
            ("Novel-Desktop-4f2a9qk.scriv", "Novel-Desktop-4f2a9qk"),
        ] {
            assert_eq!(title_from_name(name, &[], true), title, "{name}");
        }
    }

    /// Scrivener keeps ` Backup` copies of the formats it upgrades from: a current project
    /// called that is called that.
    #[test]
    fn a_current_project_s_backup_in_its_name_is_its_own() {
        assert_eq!(
            title_from_name("The Backup.scriv", &[], false),
            "The Backup"
        );
        assert_eq!(title_from_name("Novel Backup.scriv", &[], true), "Novel");
    }

    /// A title that ends with a word that happens to be this computer's name, written as a
    /// word rather than joined on as a copy's note, is the writer's.
    #[test]
    fn a_title_ending_with_a_computers_name_as_a_word_is_kept() {
        let hosts = vec!["phoenix.local".to_string(), "Studio".to_string()];
        for name in ["Rise of the Phoenix.scriv", "Recording Studio.scriv"] {
            let expected = name.trim_end_matches(".scriv");
            assert_eq!(title_from_name(name, &hosts, true), expected, "{name}");
        }
        assert_eq!(
            title_from_name("Novel-Phoenix.scriv", &hosts, true),
            "Novel"
        );
    }

    #[test]
    fn plain_names_are_not_copies() {
        for name in [
            "content.rtf",
            "12.rtf",
            "Chapter_2",
            "notes.rtf",
            "search.indexes",
            "a (b).rtf",
        ] {
            assert_eq!(conflict_original(name), None, "{name}");
        }
    }

    #[test]
    fn unsafe_names_are_refused() {
        assert_eq!(
            unsafe_member_name("/etc/passwd"),
            Some(UnsafeName::Absolute)
        );
        assert_eq!(unsafe_member_name("C:/x"), Some(UnsafeName::Absolute));
        assert_eq!(
            unsafe_member_name("a/../../b"),
            Some(UnsafeName::ParentFolder)
        );
        assert_eq!(
            unsafe_member_name("a/b\u{1}"),
            Some(UnsafeName::ControlCharacter)
        );
        assert_eq!(unsafe_member_name(""), Some(UnsafeName::Empty));
        assert_eq!(unsafe_member_name("Novel.scriv/Files/Data/x"), None);
        // A name may hold two dots in a row, just not as a whole component.
        assert_eq!(unsafe_member_name("a/b..c"), None);
    }

    #[test]
    fn placeholders_name_the_file_they_stand_for() {
        assert_eq!(
            placeholder_target(".content.rtf.icloud"),
            Some("content.rtf".to_string())
        );
        assert_eq!(
            placeholder_target("12.rtf.nextcloud"),
            Some("12.rtf".to_string())
        );
        assert_eq!(placeholder_target("content.rtf"), None);
        assert_eq!(placeholder_target(".icloud"), None);
    }

    #[test]
    fn identifiers_follow_scrivener_s_grammar() {
        assert!(is_uuid("4A1D6B18-0952-42EB-A487-D17915DD3BA1"));
        assert!(is_uuid("4a1d6b18-0952-42eb-a487-d17915dd3ba1"));
        assert!(!is_uuid("{4A1D6B18-0952-42EB-A487-D17915DD3BA1}"));
        assert!(!is_uuid("../../../../etc/passwd-aaaa-aaaa-aaaa"));
        assert!(is_item_number("0") && is_item_number("123456789"));
        assert!(!is_item_number("1234567890") && !is_item_number("-1") && !is_item_number(""));
        assert!(is_extension("webarchive") && !is_extension("tar.gz") && !is_extension(""));
    }
}
