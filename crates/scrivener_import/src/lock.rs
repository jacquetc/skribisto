// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Files/user.lock`: Scrivener's sign that a project is open somewhere.
//!
//! It is only ever read. Scrivener writes `key=value` lines (Windows and the
//! Linux beta as a Qt INI file with a `[General]` header, in CRLF): `platform`,
//! `host`, `user`, `uuid`, `app`, `project_path`, `app_path`. It carries no time
//! and no process id, so a lock left by a crash years ago looks exactly like a
//! live one; every lock in the measured corpus was committed long after Scrivener
//! closed. So a lock never blocks: the writer is told, more urgently when it names
//! this computer, and the consistent read protects the import either way.
//!
//! **Personal data.** The host, the user and both paths are the writer's (or
//! someone else's) personal data: the host may be shown to the writer on the
//! project card, and nothing of the lock ever goes into the imported project, a
//! notice's payload or a log line.

/// What a lock file says, as far as the import cares.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Lock {
    /// The computer that holds the project, as the lock names it.
    pub(crate) host: Option<String>,
}

/// Read a lock file's `key=value` lines.
pub(crate) fn parse(bytes: &[u8]) -> Lock {
    let text = String::from_utf8_lossy(bytes);
    let mut lock = Lock::default();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=')
            && key.trim() == "host"
        {
            let value = value.trim();
            if !value.is_empty() {
                lock.host = Some(value.to_string());
            }
        }
    }
    lock
}

/// Whether a lock's host names this computer.
///
/// The two sides are written differently: a Mac lock holds a bare label where
/// `gethostname` often says `Name.local`, and a Windows lock holds the NetBIOS
/// name, upper-case and cut at 15 characters, where the system reports the full
/// DNS name. So both are compared on their first DNS label, case-folded, and a
/// 15-character upper-case lock host matches a local name it is the start of.
pub(crate) fn names_this_machine(lock_host: &str, this_host: &str) -> bool {
    let first = |name: &str| {
        name.split('.')
            .next()
            .unwrap_or_default()
            .trim()
            .to_lowercase()
    };
    let lock_label = first(lock_host);
    let local_label = first(this_host);
    if lock_label.is_empty() || local_label.is_empty() {
        return false;
    }
    if lock_label == local_label {
        return true;
    }
    let netbios = lock_host.len() == 15 && lock_host.chars().all(|c| !c.is_lowercase());
    netbios && local_label.starts_with(&lock_label)
}

/// This computer's host name, when the system says.
pub fn this_host() -> Option<String> {
    let name = gethostname::gethostname();
    let name = name.to_string_lossy().trim().to_string();
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mac_lock_and_a_windows_lock_both_read() {
        let mac = b"platform=mac\nhost=Quill-Harbour\nuser=Someone\nuuid=E69E\n";
        assert_eq!(parse(mac).host.as_deref(), Some("Quill-Harbour"));
        let win = b"[General]\r\nplatform=win\r\nhost=DESKTOP-ABCDEFG\r\nuser=Someone\r\n";
        assert_eq!(parse(win).host.as_deref(), Some("DESKTOP-ABCDEFG"));
    }

    #[test]
    fn this_machine_is_recognised_however_each_side_spells_it() {
        assert!(names_this_machine("Quill-Harbour", "Quill-Harbour.local"));
        assert!(names_this_machine("quill-harbour", "QUILL-HARBOUR.home"));
        // NetBIOS: upper-case, cut at 15 characters.
        assert!(names_this_machine(
            "WRITERS-LAPTOP-",
            "writers-laptop-2024.corp.example"
        ));
        assert!(!names_this_machine("Another-Mac", "Quill-Harbour.local"));
        assert!(!names_this_machine("WRITERS", "writers-laptop.local"));
        assert!(!names_this_machine("", "anything"));
    }
}
