// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `app.identity` slot: who the running application is.
//!
//! Every other slot in the seam adds something *to* Skribisto. This one says what
//! Skribisto **is**: the `(qualifier, organization, application)` triple that
//! decides where settings live, and the display name the writer reads in window
//! titles, About and the Welcome screen.
//!
//! ## Why this exists
//!
//! An edition that does not register an identity of its own shares the community
//! build's config directory — and the config directory is what
//! [`crate::shell::open_registry::namespace`] hashes to decide which instances
//! belong to the same installation. Two editions with one config directory
//! therefore share **one single-instance election**: launch an extension build
//! while the community build is running and it exits in ~half a second, having
//! handed its project to a window with none of the extension in it. Verified
//! live, not inferred — the extension process is gone in 489 ms with an empty
//! log, and nothing anywhere reports that the writer did not get what they
//! double-clicked.
//!
//! Registering an identity moves the config directory, which moves the namespace,
//! which separates the elections.
//!
//! ## ⚠ The earliest-read slot in the seam
//!
//! Every registry here is a snapshot rather than a subscription, and this one is
//! read **earliest of all** — before `AppContext::new()`, before the first
//! window, at the top of [`crate::run`] where the election happens. Registering
//! after that point is worse than being ignored: the process has already elected,
//! bound a socket and resolved its settings under the *previous* identity, so a
//! late registration produces a half-migrated process rather than a no-op.
//!
//! Register on the main thread, before `run()`, like everything else.
//!
//! ## The family, and what it is deliberately for
//!
//! [`family_paths`] always resolves the **community** triple, whatever identity is
//! registered. Exactly one caller wants it —
//! [`crate::shell::open_registry::dir`] — and it wants it for a reason that took
//! a data-loss trace to find:
//!
//! `BackupRestoreViewModel::check_open_elsewhere` refuses to restore a backup over
//! a project another instance holds open, by reading the lock files in that
//! directory. Key the directory to the *edition* and the two builds stop seeing
//! each other, so: community holds `Novel.skrib` with unsaved edits → the
//! extension restores a backup over it → sees no peer → proceeds → community
//! autosaves its stale in-memory state back over the restored file. Silently.
//!
//! So the lock **directory** is family-shared and the primary **socket** is
//! per-edition: the editions can see one another's claims, and still elect
//! separately. See `open_registry`'s module docs for the naming that implements
//! it.
//!
//! This is also why `identity.rs` is the only module in the crate allowed to name
//! the literal triple — `tests::no_module_resolves_its_own_app_paths` walks the
//! source and fails on any other site. A module that resolves its own `AppPaths`
//! silently reads and writes the *community's* file while everything around it
//! uses the edition's, and half-migrated settings are worse than none.

use std::sync::{LazyLock, RwLock};

use teksilo::settings::AppPaths;

/// The community edition's `directories` triple.
///
/// Also the default: a build that registers nothing behaves exactly as it did
/// before this module existed, byte for byte, which is what keeps the community
/// application unaffected by the seam.
pub const COMMUNITY_QUALIFIER: &str = "eu";
pub const COMMUNITY_ORGANIZATION: &str = "skribisto";
pub const COMMUNITY_APPLICATION: &str = "Skribisto";

/// Who the running application is.
///
/// The triple follows the `directories` / `etcetera` convention and picks the
/// platform-native locations: on Linux the config directory is the *application*
/// name lowercased with spaces turned to hyphens (`"Skribisto Pro"` →
/// `~/.config/skribisto-pro`), on macOS it is `{qualifier}.{organization}.{application}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppIdentity {
    pub qualifier: String,
    pub organization: String,
    pub application: String,
    /// What the writer sees. Defaults to `application`; override when the path
    /// name and the product name should differ.
    pub display_name: String,
}

impl AppIdentity {
    /// A new identity whose display name is its application name.
    pub fn new(
        qualifier: impl Into<String>,
        organization: impl Into<String>,
        application: impl Into<String>,
    ) -> Self {
        let application = application.into();
        Self {
            qualifier: qualifier.into(),
            organization: organization.into(),
            display_name: application.clone(),
            application,
        }
    }

    /// Override the user-visible name, leaving the paths alone.
    #[must_use]
    pub fn with_display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = name.into();
        self
    }

    /// The community edition's identity — the default when nothing is registered.
    pub fn community() -> Self {
        Self::new(
            COMMUNITY_QUALIFIER,
            COMMUNITY_ORGANIZATION,
            COMMUNITY_APPLICATION,
        )
    }

    /// Whether this *is* the community identity (all three path fields), which is
    /// what decides whether a first-run import is offered and whether the primary
    /// socket carries an edition suffix.
    ///
    /// Compares the path triple only: a build that renamed itself in the title bar
    /// but kept the community's directories is still the community installation as
    /// far as settings and the election are concerned, and treating it otherwise
    /// would offer it an import from itself.
    pub fn is_community(&self) -> bool {
        self.qualifier == COMMUNITY_QUALIFIER
            && self.organization == COMMUNITY_ORGANIZATION
            && self.application == COMMUNITY_APPLICATION
    }

    /// This identity's OS directories, or `None` where no home directory is
    /// detectable (the same degraded case `AppPaths::new` reports).
    pub fn app_paths(&self) -> Option<AppPaths> {
        AppPaths::new(&self.qualifier, &self.organization, &self.application)
    }

    /// A short, filesystem-safe discriminator for this identity.
    ///
    /// **Six hex characters, and the length is load-bearing.** This lands in the
    /// primary socket's file name, which on macOS lives under
    /// `~/Library/Application Support/eu.skribisto.Skribisto/run/` — and a Unix
    /// domain socket's `sun_path` holds only 104 bytes on Darwin. A readable
    /// suffix (`primary-skribisto-pro`) pushes a realistic path to 108 and
    /// `bind()` fails with `ENAMETOOLONG`, which does not crash: it degrades
    /// silently to `Standalone`, so single instance simply never engages and
    /// nobody finds out. `open_registry::tests::the_macos_socket_path_fits_in_sun_path`
    /// pins the budget for both leaf kinds.
    pub fn slug(&self) -> String {
        let key = format!(
            "{}.{}.{}",
            self.qualifier, self.organization, self.application
        );
        blake3::hash(key.as_bytes()).to_hex()[..6].to_string()
    }
}

static IDENTITY: LazyLock<RwLock<Option<AppIdentity>>> = LazyLock::new(|| RwLock::new(None));

/// Declare who this application is. Call **before** [`crate::run`], on the main
/// thread.
///
/// Registering twice replaces the earlier identity. The returned handle restores
/// the previous one on drop, which matters in tests and is inert in an
/// application, where the identity outlives the process.
pub fn register(identity: AppIdentity) -> IdentityHandle {
    let mut slot = IDENTITY.write().unwrap_or_else(|e| e.into_inner());
    let previous = slot.replace(identity);
    IdentityHandle { previous }
}

/// Restores the previous identity when dropped.
#[derive(Debug)]
pub struct IdentityHandle {
    previous: Option<AppIdentity>,
}

impl Drop for IdentityHandle {
    fn drop(&mut self) {
        let mut slot = IDENTITY.write().unwrap_or_else(|e| e.into_inner());
        *slot = self.previous.take();
    }
}

/// The running identity — the community one when nothing has registered.
pub fn current() -> AppIdentity {
    IDENTITY
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
        .unwrap_or_else(AppIdentity::community)
}

/// This edition's config and data directories. **The only way any module in this
/// crate should resolve them.**
pub fn app_paths() -> Option<AppPaths> {
    current().app_paths()
}

/// The **community** directories, whatever edition is running.
///
/// One caller, one reason: the cross-instance open registry, so that every
/// edition installed on a machine shares one lock directory and can see what the
/// others hold open. See the module docs for the data-loss trace that requires
/// it, and `open_registry`'s for how the elections stay separate anyway.
pub fn family_paths() -> Option<AppPaths> {
    AppIdentity::community().app_paths()
}

/// What the writer sees. Product names are **data**, never translated — the same
/// rule that keeps entity titles on `lit!`.
pub fn display_name() -> String {
    current().display_name
}

/// Whether the running build is the community edition.
pub fn is_community() -> bool {
    current().is_community()
}

/// Serialize a test that registers an identity.
///
/// The identity is process-wide and `cargo test` runs in parallel, so a test that
/// registers one races every test that reads it — including tests in *other*
/// modules (`open_registry`'s socket naming, the migration's offer conditions).
/// One lock for the whole crate, not one per test module, or those modules race
/// each other instead.
///
/// Learned from the lifecycle and settings registries, whose first drafts failed
/// intermittently for exactly this reason and read as flakes rather than as test
/// cross-talk.
#[cfg(test)]
pub(crate) fn lock_for_test() -> std::sync::MutexGuard<'static, ()> {
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole compatibility promise of this module: a build that registers
    /// nothing is byte-for-byte the application that existed before it.
    #[test]
    fn the_default_identity_is_the_community_one() {
        let _serial = lock_for_test();
        let id = current();
        assert_eq!(id.qualifier, "eu");
        assert_eq!(id.organization, "skribisto");
        assert_eq!(id.application, "Skribisto");
        assert_eq!(id.display_name, "Skribisto");
        assert!(id.is_community());
    }

    #[test]
    fn a_registered_identity_moves_the_paths_and_the_name() {
        let _serial = lock_for_test();
        let community = app_paths().expect("a home directory is detectable in tests");
        let _h = register(AppIdentity::new("eu", "skribisto-pro", "Skribisto Pro"));

        assert_eq!(display_name(), "Skribisto Pro");
        assert!(!is_community());
        let edition = app_paths().expect("the edition resolves paths too");
        assert_ne!(
            edition.config_dir(),
            community.config_dir(),
            "an edition that registered its own identity must not share the community config dir \
             — that shared directory is what makes the two builds share one election"
        );
    }

    /// `family_paths` is the exception that makes the cross-edition overwrite
    /// guard work; if it ever started following the registered identity, two
    /// editions would stop seeing each other's open projects.
    #[test]
    fn family_paths_ignore_the_registered_identity() {
        let _serial = lock_for_test();
        let community = family_paths().expect("a home directory is detectable in tests");
        let _h = register(AppIdentity::new("eu", "skribisto-pro", "Skribisto Pro"));
        assert_eq!(
            family_paths().map(|p| p.config_dir().to_path_buf()),
            Some(community.config_dir().to_path_buf()),
            "the family directories must stay put, or the editions stop sharing a lock directory"
        );
    }

    #[test]
    fn dropping_the_handle_restores_the_previous_identity() {
        let _serial = lock_for_test();
        {
            let _h = register(AppIdentity::new("eu", "skribisto-pro", "Skribisto Pro"));
            assert_eq!(display_name(), "Skribisto Pro");
        }
        assert_eq!(display_name(), "Skribisto");
        assert!(is_community());
    }

    /// The display name is allowed to differ from the path name — a product can
    /// be "Skribisto Pro" on screen and `skribisto-pro` on disk.
    #[test]
    fn a_display_name_can_differ_from_the_application_name() {
        let id = AppIdentity::new("eu", "acme", "acme-writer").with_display_name("Acme Writer");
        assert_eq!(id.application, "acme-writer");
        assert_eq!(id.display_name, "Acme Writer");
    }

    /// Renaming only the title bar leaves the installation alone: same
    /// directories, same election, and no first-run import offered from itself.
    #[test]
    fn a_renamed_community_build_is_still_the_community_installation() {
        let id = AppIdentity::community().with_display_name("My Build");
        assert!(id.is_community());
    }

    #[test]
    fn distinct_identities_get_distinct_slugs() {
        let a = AppIdentity::community().slug();
        let b = AppIdentity::new("eu", "skribisto-pro", "Skribisto Pro").slug();
        assert_ne!(a, b);
        assert_eq!(
            a.len(),
            6,
            "the macOS sun_path budget depends on this length"
        );
        assert_eq!(b.len(), 6);
        assert!(b.chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// A slug is derived from the path triple alone, so two builds that differ
    /// only in what they call themselves on screen still elect together — which
    /// is the same claim `a_renamed_community_build_is_still_the_community_installation`
    /// makes about the directories.
    #[test]
    fn the_slug_ignores_the_display_name() {
        assert_eq!(
            AppIdentity::community().slug(),
            AppIdentity::community()
                .with_display_name("Something Else")
                .slug()
        );
    }

    /// **The drift guard.** Every module must resolve its directories through
    /// [`app_paths`] (or, for the one caller that needs it, [`family_paths`]);
    /// none may call `AppPaths::new` itself.
    ///
    /// A site that keeps its own literal does not fail loudly — it silently reads
    /// and writes the *community's* file while every module around it uses the
    /// edition's, which is a half-migrated install and worse than no migration at
    /// all. This is not hypothetical: collapsing the original sites, a 23rd in
    /// `view_models/new_work.rs` was missed by a hand-built file list and only
    /// this walk found it.
    ///
    /// A directory walk rather than a fixed list, for the same reason
    /// `settings_keys`' key-drift test uses one: the case worth catching is a call
    /// added in a file nobody thought to check.
    #[test]
    fn no_module_resolves_its_own_app_paths() {
        use std::path::{Path, PathBuf};

        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }

        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        walk(&src, &mut files);

        let mut offenders = Vec::new();
        for file in files {
            // This module *defines* the identity, so it is the one place allowed to
            // name a triple — and it necessarily contains the scanner's own needle.
            if file.file_name().is_some_and(|n| n == "identity.rs") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            for (index, _) in text.match_indices("AppPaths::new(") {
                // `for_testing` / `from_dirs` are fine; only the OS-resolving
                // constructor decides which installation's files we touch.
                let line = text[..index].matches('\n').count() + 1;
                offenders.push(format!("{}:{line}", file.display()));
            }
        }

        assert!(
            offenders.is_empty(),
            "these sites resolve their own AppPaths instead of going through \
             `identity::app_paths()`, so they would keep reading the community \
             installation's files under any other edition:\n  {}\n\n\
             If a site genuinely needs the community directories whatever edition is \
             running — today only `open_registry::dir`, so that the editions share one \
             lock directory and can see each other's open projects — use \
             `identity::family_paths()` and say why at the call site.",
            offenders.join("\n  ")
        );
    }
}
