// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Persistent dictionary bookkeeping — **which licences the user has accepted**.
//!
//! A `SettingsFile<DictionarySettingsFile>` at `<config_dir>/dictionaries.toml`, the exact
//! shape and cross-process story as [`crate::models::backup_settings_file`]: one locked
//! read-modify-write per mutation, registered into the app's `SettingsRegistry` so a peer
//! process's write reloads in place. App configuration, orthogonal to the backend, so a
//! single implementation — no real/mock seam (the acceptance record is identical in both
//! builds; only the *download* differs, and that lives in the view-model).
//!
//! It stores **only** accepted licences. The installed-dictionary *list* is never persisted
//! here — it is always re-derived by scanning the data dir + system dirs
//! ([`crate::models::installed_dictionaries_model`]), a directory listing, not a parse, so it
//! cannot drift from what is actually on disk.
//!
//! ## What an acceptance records, and why the hash
//!
//! `(dictionary_id, accepted_at, license_text_hash)`. The hash is a BLAKE3 of the **exact
//! bundled licence text shown at accept time** (`blake3` is already a workspace dependency).
//! Storing the hash rather than the full text keeps the canonical copy in the bundled asset
//! while making the record verifiable: if a registry entry's licence text is ever corrected,
//! a stale hash on file is a legitimate signal to re-prompt. Acceptance is keyed **per
//! dictionary id**, not per licence text — the clearer mental model, and it never reuses a
//! "close enough" acceptance for a dictionary whose bundled text actually differs.

use std::rc::Rc;

use serde::{Deserialize, Serialize};
use teksilo::settings::{
    AppPaths, Migrator, Reloadable, SettingsFile, SettingsFileError, Versioned,
};

/// One accepted-licence record.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AcceptedLicense {
    /// The registry id of the dictionary this acceptance is for.
    pub dictionary_id: String,
    /// RFC3339 timestamp of acceptance.
    pub accepted_at: String,
    /// Lowercase hex BLAKE3 of the exact bundled licence text shown at accept time.
    pub license_text_hash: String,
}

/// A dictionary the user added by hand from local `.aff`/`.dic` files (Settings ▸ Dictionaries ▸
/// *Add dictionary*). The **files** are copied into the download dir as `{code}.aff`/`.dic` — so
/// the loader and the install scan find them exactly like a downloaded one, with no engine change
/// — and this record supplies the human **name** they lack (a `.aff`/`.dic` carries no title) and
/// makes the custom `code` offerable in the language picker. Removing the dictionary deletes both
/// the files and this record.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserDictionary {
    /// The `dict_language` value this dictionary satisfies — the on-disk basename of its copied
    /// files (`{code}.aff`/`.dic`). Not a registry id (those are added through the catalogue).
    pub code: String,
    /// The display name the user gave it.
    pub name: String,
}

/// The persisted dictionary settings — accepted licences and user-added dictionaries.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DictionarySettingsFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub accepted: Vec<AcceptedLicense>,
    /// Hand-added dictionaries (name + code); the files live in the download dir. `#[serde(default)]`
    /// so a pre-existing `dictionaries.toml` (which never had this key) still loads.
    #[serde(default)]
    pub user_dictionaries: Vec<UserDictionary>,
}

fn default_version() -> u32 {
    DictionarySettingsFile::CURRENT_VERSION
}

impl Default for DictionarySettingsFile {
    fn default() -> Self {
        DictionarySettingsFile {
            version: DictionarySettingsFile::CURRENT_VERSION,
            accepted: Vec::new(),
            user_dictionaries: Vec::new(),
        }
    }
}

impl Versioned for DictionarySettingsFile {
    // v2 added `user_dictionaries`. The bump matters for *downgrades*: an older build (v1) reading
    // a v2 file is refused by the Migrator (on-disk > current) and falls back rather than
    // rewriting it — so it can't silently drop the `user_dictionaries` it doesn't know about.
    const CURRENT_VERSION: u32 = 2;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// The migrator for `dictionaries.toml`. v1→v2 only added `user_dictionaries` (a `#[serde(default)]`
/// field), so the step is the identity — the Migrator stamps the new version and serde fills the
/// empty list. Shared by every load site so a v1 file on disk upgrades consistently.
fn migrator() -> Migrator<DictionarySettingsFile> {
    Migrator::new().step(1, Ok)
}

/// The lowercase-hex BLAKE3 of a licence text — the canonical hashing used both when
/// recording an acceptance and when checking one, so the two can never disagree.
pub fn license_hash(text: &str) -> String {
    blake3::hash(text.as_bytes()).to_hex().to_string()
}

/// Persistent accepted-licence store. `SettingsFile` is `Clone` (shares the live in-memory
/// state), so every clone is a view over the same configuration.
#[derive(Clone)]
pub struct DictionarySettingsService {
    file: SettingsFile<DictionarySettingsFile>,
}

impl DictionarySettingsService {
    /// Open `dictionaries.toml` under `paths` (cross-process safe by default).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("dictionaries"), migrator())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, migrator())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process temp
    /// file, so the app still runs (acceptances just won't persist across restarts). Shares
    /// its retry/uniqueness logic with every sibling via
    /// [`in_memory_settings_file`](super::backup_settings_file::in_memory_settings_file).
    pub fn in_memory_default() -> Self {
        let file = super::backup_settings_file::in_memory_settings_file("dictionaries", migrator());
        Self { file }
    }

    /// The `Reloadable` hook for the app's shared `SettingsRegistry` (keep the returned handle
    /// alive) so a peer process's acceptance write refreshes this handle in place.
    pub fn as_reloadable(&self) -> Rc<dyn Reloadable> {
        Rc::new(self.file.clone())
    }

    /// Whether the user has accepted the licence for `dictionary_id` whose text hashes to
    /// `expected_hash` (the currently-bundled text). A stale hash — a licence text corrected
    /// since acceptance — reads as *not accepted*, which re-prompts, as intended.
    pub fn has_accepted(&self, dictionary_id: &str, expected_hash: &str) -> bool {
        self.file
            .borrow()
            .accepted
            .iter()
            .any(|a| a.dictionary_id == dictionary_id && a.license_text_hash == expected_hash)
    }

    /// Record acceptance of `dictionary_id` for the exact licence text hashing to `hash` at
    /// `accepted_at` (RFC3339). Replaces any prior record for the same id (a re-acceptance
    /// after a text correction), so the file never accumulates stale duplicates.
    pub fn accept(
        &self,
        dictionary_id: &str,
        hash: &str,
        accepted_at: &str,
    ) -> Result<(), SettingsFileError> {
        let record = AcceptedLicense {
            dictionary_id: dictionary_id.to_string(),
            accepted_at: accepted_at.to_string(),
            license_text_hash: hash.to_string(),
        };
        self.file.mutate(|f| {
            f.accepted.retain(|a| a.dictionary_id != dictionary_id);
            f.accepted.push(record);
        })
    }

    // ── user-added dictionaries ──

    /// The hand-added dictionaries recorded here (name + code). The files themselves are in the
    /// download dir; this is only the metadata a `.aff`/`.dic` cannot carry.
    pub fn user_dictionaries(&self) -> Vec<UserDictionary> {
        self.file.borrow().user_dictionaries.clone()
    }

    /// Record a user-added dictionary, replacing any prior record for the same `code` (a re-add
    /// after replacing the files), so the file never accumulates duplicates.
    pub fn add_user_dictionary(&self, code: &str, name: &str) -> Result<(), SettingsFileError> {
        let record = UserDictionary {
            code: code.to_string(),
            name: name.to_string(),
        };
        self.file.mutate(|f| {
            f.user_dictionaries.retain(|d| d.code != code);
            f.user_dictionaries.push(record);
        })
    }

    /// Drop the record for `code` (called when the dictionary is removed). A no-op if `code` was
    /// never a user-added dictionary, so it is safe to call from the generic remove path.
    pub fn remove_user_dictionary(&self, code: &str) -> Result<(), SettingsFileError> {
        self.file
            .mutate(|f| f.user_dictionaries.retain(|d| d.code != code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_service() -> DictionarySettingsService {
        // A unique path per test invocation — index by nanos is unavailable in this harness,
        // so use the pid plus a monotonic counter baked into the name.
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto-dicttest-{}-{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        DictionarySettingsService::open_at(path).expect("open temp dictionary settings")
    }

    #[test]
    fn acceptance_round_trips_and_is_hash_scoped() {
        let svc = temp_service();
        let hash = license_hash("Mozilla Public License Version 2.0 …");

        assert!(
            !svc.has_accepted("fr-FR-x-1990", &hash),
            "nothing accepted yet"
        );
        svc.accept("fr-FR-x-1990", &hash, "2026-07-15T00:00:00Z")
            .unwrap();
        assert!(svc.has_accepted("fr-FR-x-1990", &hash));

        // A different (corrected) text hash reads as not-accepted → re-prompt.
        let other = license_hash("Mozilla Public License Version 2.0 (corrected) …");
        assert!(
            !svc.has_accepted("fr-FR-x-1990", &other),
            "a changed licence text must not count as accepted"
        );
    }

    #[test]
    fn re_accepting_replaces_rather_than_duplicates() {
        let svc = temp_service();
        let h1 = license_hash("v1");
        let h2 = license_hash("v2");
        svc.accept("en-US", &h1, "2026-07-15T00:00:00Z").unwrap();
        svc.accept("en-US", &h2, "2026-07-15T01:00:00Z").unwrap();
        assert_eq!(svc.file.borrow().accepted.len(), 1, "one record per id");
        assert!(svc.has_accepted("en-US", &h2));
        assert!(!svc.has_accepted("en-US", &h1), "the old hash is gone");
    }

    #[test]
    fn user_dictionaries_add_replace_remove() {
        let svc = temp_service();
        assert!(svc.user_dictionaries().is_empty());

        svc.add_user_dictionary("fr-FR-x-mine", "My French")
            .unwrap();
        svc.add_user_dictionary("cy-GB", "Cymraeg").unwrap();
        assert_eq!(svc.user_dictionaries().len(), 2);

        // Re-adding the same code replaces (a renamed re-import), never duplicates.
        svc.add_user_dictionary("fr-FR-x-mine", "My French (v2)")
            .unwrap();
        let list = svc.user_dictionaries();
        assert_eq!(list.len(), 2, "one record per code");
        assert_eq!(
            list.iter().find(|d| d.code == "fr-FR-x-mine").unwrap().name,
            "My French (v2)"
        );

        svc.remove_user_dictionary("fr-FR-x-mine").unwrap();
        assert_eq!(svc.user_dictionaries().len(), 1);
        // Removing a code that was never added is a harmless no-op (the generic remove path).
        svc.remove_user_dictionary("not-there").unwrap();
        assert_eq!(svc.user_dictionaries().len(), 1);
    }

    /// A v1 file on disk upgrades to v2 through the migrator on open — its `accepted` list is
    /// preserved, `user_dictionaries` defaults empty, and the version is stamped forward.
    #[test]
    fn v1_file_upgrades_to_v2_on_open() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("skribisto-dictmig-{}-{n}.toml", std::process::id()));
        std::fs::write(
            &path,
            "version = 1\n[[accepted]]\ndictionary_id = \"en-US\"\naccepted_at = \"t\"\nlicense_text_hash = \"h\"\n",
        )
        .unwrap();

        let svc = DictionarySettingsService::open_at(path.clone()).expect("v1 upgrades to v2");
        assert_eq!(svc.file.borrow().version, 2, "version stamped forward");
        assert_eq!(svc.file.borrow().accepted.len(), 1, "acceptances preserved");
        assert!(
            svc.user_dictionaries().is_empty(),
            "user list defaults empty"
        );
        let _ = std::fs::remove_file(&path);
    }
}
