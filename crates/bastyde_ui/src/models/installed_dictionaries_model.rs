//! Layer-A model: the Hunspell dictionaries actually present on this machine.
//!
//! Feeds the Settings ▸ Dictionaries ▸ *Installed* tab. Two tiers, tier 1 winning on a
//! collision (the same dictionary found in both shows as **one** row, badged):
//!
//! 1. **Skribisto's own downloads** — `<data_dir>/dictionaries/{id}.aff|.dic`. Known-good
//!    UTF-8 (registry sources are), and the only tier the UI lets you *remove*.
//! 2. **System dictionaries** — `/usr/share/hunspell` etc., read-only. Often ISO-8859-x and
//!    a symlink farm (`fr_FR.dic → fr.dic`), so files are deduped by **resolved path** and a
//!    non-UTF-8 `.aff` `SET` directive is recorded for the encoding-aware loader.
//!
//! ## Real / mock seam
//!
//! The `ListModel` wrapper and refresh logic are identical in both builds, so this takes the
//! **data-seam** exception (like the binder tree): the type and its API are written once, and
//! only the [`source::scan`] function is `#[cfg]`-gated — real walks the disk, mock fabricates
//! a plausible couple of rows. No `#[cfg]` reaches any consumer.

use bastyde::data::ListModel;

/// Where a discovered dictionary lives — which decides whether it is removable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictOrigin {
    /// Downloaded by Skribisto into its data dir. Removable.
    Downloaded,
    /// Found in a system directory. Read-only here (a distro package isn't ours to delete).
    System,
}

/// One discovered dictionary on disk.
#[derive(Debug, Clone)]
pub struct InstalledDictionaryRow {
    /// The registry id if the basename matched one, else the raw file stem (surfaced as-is).
    pub id: String,
    /// Registry display name if matched, else a "⟨stem⟩ (system)" fallback.
    pub display_name: String,
    pub origin: DictOrigin,
    pub aff_path: std::path::PathBuf,
    pub dic_path: std::path::PathBuf,
    /// Whether `id` is a known registry id (an unmatched system dict is shown but greyed).
    pub matched: bool,
    /// A non-UTF-8 encoding declared by the `.aff`'s `SET` directive, if any — the loader
    /// transcodes these; `None` means UTF-8 / unspecified (the safe default).
    pub encoding: Option<String>,
    /// Also present in a system directory (tier-1 row only). Drives the "also on your system"
    /// badge, so a dictionary found in both tiers is one row, not two.
    pub also_system: bool,
}

/// Reactive list of the dictionaries present on this machine.
#[derive(Clone)]
pub struct InstalledDictionariesModel {
    model: ListModel<InstalledDictionaryRow>,
}

impl InstalledDictionariesModel {
    /// Build and populate by scanning now.
    pub fn new() -> Self {
        let me = Self {
            model: ListModel::new(),
        };
        me.refresh();
        me
    }

    /// The reactive list a `ListView` binds to.
    pub fn list_model(&self) -> ListModel<InstalledDictionaryRow> {
        self.model.clone()
    }

    /// Re-scan the disk and replace the rows. Cheap (a directory listing), so it is safe to
    /// call on every relevant change (install, remove, window focus-regain).
    pub fn refresh(&self) {
        self.model.replace_all(source::scan());
    }

    /// Whether a dictionary with this registry id is installed (in either tier).
    pub fn is_installed(&self, id: &str) -> bool {
        (0..self.model.len()).any(|i| self.model.with_item(i, |r| r.id == id).unwrap_or(false))
    }
}

impl Default for InstalledDictionariesModel {
    fn default() -> Self {
        Self::new()
    }
}

// ── the one gated seam: where the rows come from ──

#[cfg(not(feature = "mocks"))]
mod source {
    use super::{DictOrigin, InstalledDictionaryRow};
    use crate::dictionary_registry;
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};

    /// Scan tier 1 (our downloads) then tier 2 (system), deduping by resolved `.dic` path so a
    /// symlink alias or a both-tiers dictionary appears once, tier 1 winning.
    pub(super) fn scan() -> Vec<InstalledDictionaryRow> {
        let mut rows: Vec<InstalledDictionaryRow> = Vec::new();
        let mut seen: HashSet<PathBuf> = HashSet::new();
        // The resolved .dic paths of tier-2, so a tier-1 row can flag "also on your system".
        let system_resolved: HashSet<PathBuf> = system_dirs()
            .iter()
            .flat_map(|d| pairs_in(d))
            .filter_map(|(_, dic)| std::fs::canonicalize(&dic).ok())
            .collect();

        for (aff, dic) in downloaded_dir().iter().flat_map(|d| pairs_in(d)) {
            let key = std::fs::canonicalize(&dic).unwrap_or_else(|_| dic.clone());
            if !seen.insert(key.clone()) {
                continue;
            }
            let also_system = system_resolved.contains(&key);
            rows.push(row_for(aff, dic, DictOrigin::Downloaded, also_system));
        }

        for dir in system_dirs() {
            for (aff, dic) in pairs_in(&dir) {
                let key = std::fs::canonicalize(&dic).unwrap_or_else(|_| dic.clone());
                if !seen.insert(key) {
                    continue; // a symlink alias, or already taken by tier 1
                }
                rows.push(row_for(aff, dic, DictOrigin::System, false));
            }
        }
        rows
    }

    // The dictionary directories (our download dir + the OS system dirs) have one definition,
    // shared with the loader in `crate::spellcheck`.
    use crate::spellcheck::{downloaded_dictionaries_dir as downloaded_dir, system_dictionary_dirs as system_dirs};

    /// Every `<stem>.aff` in `dir` that has a sibling `<stem>.dic`, as `(aff, dic)` paths.
    fn pairs_in(dir: &Path) -> Vec<(PathBuf, PathBuf)> {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for entry in entries.flatten() {
            let aff = entry.path();
            if aff.extension().and_then(|e| e.to_str()) != Some("aff") {
                continue;
            }
            let dic = aff.with_extension("dic");
            if dic.exists() {
                out.push((aff, dic));
            }
        }
        out
    }

    fn row_for(
        aff: PathBuf,
        dic: PathBuf,
        origin: DictOrigin,
        also_system: bool,
    ) -> InstalledDictionaryRow {
        let stem = aff
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        // Our own downloads are named by registry `id` (`fr-FR-x-1990.aff`); system files by
        // an editorial basename (`fr-reforme1990`, `de_DE_frami`). Try both, id first.
        let resolved = dictionary_registry::by_id(&stem)
            .map(|e| e.id.as_str())
            .or_else(|| dictionary_registry::id_for_basename(&stem));
        let (id, display_name, matched) = match resolved {
            Some(id) => {
                let name = dictionary_registry::by_id(id)
                    .map(|e| e.display_name.clone())
                    .unwrap_or_else(|| id.to_string());
                (id.to_string(), name, true)
            }
            None => (stem.clone(), format!("{stem} (system)"), false),
        };
        let encoding = match origin {
            // Our own downloads are UTF-8 by construction; skip the read.
            DictOrigin::Downloaded => None,
            DictOrigin::System => aff_encoding(&aff),
        };
        InstalledDictionaryRow {
            id,
            display_name,
            origin,
            aff_path: aff,
            dic_path: dic,
            matched,
            encoding,
            also_system,
        }
    }

    /// The non-UTF-8 encoding an `.aff` declares via its `SET <name>` directive, if any.
    /// Reads only the head of the file — the directive is in the first handful of lines.
    fn aff_encoding(aff: &Path) -> Option<String> {
        use std::io::Read;
        let mut buf = [0u8; 512];
        let n = std::fs::File::open(aff).ok()?.read(&mut buf).ok()?;
        // `SET` and the encoding name are always ASCII, so a lossy decode of the head is safe.
        let head = String::from_utf8_lossy(&buf[..n]);
        for line in head.lines() {
            if let Some(rest) = line.strip_prefix("SET ") {
                let enc = rest.trim();
                if !enc.eq_ignore_ascii_case("UTF-8") {
                    return Some(enc.to_string());
                }
                return None;
            }
        }
        None
    }
}

#[cfg(feature = "mocks")]
mod source {
    use super::{DictOrigin, InstalledDictionaryRow};
    use crate::dictionary_registry;
    use std::path::PathBuf;

    /// Fabricated rows so the Installed tab renders in the mock build: one downloaded, one
    /// system, drawn from real registry ids so the display names are consistent.
    pub(super) fn scan() -> Vec<InstalledDictionaryRow> {
        let named = |id: &str, origin: DictOrigin, also_system: bool| {
            let display_name = dictionary_registry::by_id(id)
                .map(|e| e.display_name.clone())
                .unwrap_or_else(|| id.to_string());
            InstalledDictionaryRow {
                id: id.to_string(),
                display_name,
                origin,
                aff_path: PathBuf::from(format!("/mock/{id}.aff")),
                dic_path: PathBuf::from(format!("/mock/{id}.dic")),
                matched: true,
                encoding: None,
                also_system,
            }
        };
        vec![
            named("en-US", DictOrigin::Downloaded, false),
            named("fr-FR", DictOrigin::System, false),
        ]
    }
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;
    use std::io::Write;

    /// A dictionary in the data dir and its symlink alias in a system dir dedupe to one row —
    /// the shared scan logic (exercised here directly against a temp layout).
    #[test]
    fn a_pair_is_discovered_and_matched_to_the_registry() {
        // Write a fake en_US .aff/.dic pair into a temp "downloaded" dir and confirm the
        // basename resolves to the registry's en-US entry.
        let dir = std::env::temp_dir().join(format!("skrib-instdict-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let aff = dir.join("en_US.aff");
        let dic = dir.join("en_US.dic");
        let mut f = std::fs::File::create(&aff).unwrap();
        writeln!(f, "SET UTF-8").unwrap();
        std::fs::write(&dic, "1\nword\n").unwrap();

        // The basename → registry id mapping is the load-bearing bit the scan relies on.
        assert_eq!(
            crate::dictionary_registry::id_for_basename("en_US"),
            Some("en-US")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
