// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The debug-build startup flags, and the prefs read before the app is built.
//!
//! `--dump-config` prints every settable key with its effective value, in
//! exactly the syntax a pins file wants; `--config <file>` pins a set of them
//! for one run. Both run **ahead of the election** — pins must reach the process
//! that will actually hold them, and a dump must not depend on winning.
//!
//! The whole point of the pair is that an unknown or mistyped key is a hard
//! startup error naming the offender and its nearest legal neighbour. See
//! [`crate::settings_keys`] for the schema that makes that possible, and for the
//! wrong diagnosis that silence once cost.

use teksilo::prelude::*;

// `self` is only reached from the `debug_assertions` halves below (`dump`,
// `load_pins`, `merge_into`), which a release build compiles out — so in release it
// is genuinely unused and the lint is right. Said here rather than dropped, because
// the module IS used in every build a person runs these flags from.
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::settings_keys::{
    self, AUTOSAVE_KEY, DARK_KEY, LOCALE_KEY, SHOW_WELCOME_KEY, SPELLCHECK_ENABLED_DEFAULT,
    SPELLCHECK_ENABLED_KEY,
};

/// The settings store's own file, resolved exactly as `read_prefs` and the app
/// builder's `SettingsBundle` resolve it (`AppPaths` → `config_file("general")`,
/// which appends `.toml`). `--config` merges into this file and `--dump-config`
/// reads it, so all four agree on one path by construction.
/// Dead in a release build for the same reason the import above is: both callers
/// live in `debug_assertions` halves. Kept rather than gated, because gating the
/// function would mean gating its doc comment and its one honest definition of where
/// the file is.
#[cfg_attr(not(debug_assertions), allow(dead_code))]
pub(crate) fn general_settings_path() -> Option<std::path::PathBuf> {
    crate::identity::app_paths().map(|paths| paths.config_file("general"))
}

/// `--dump-config`: print every settable key with its effective value, then exit.
///
/// Exits rather than launching. It is a question about configuration, asked most
/// often before any app is running, and answering it *and* opening a window would
/// make it useless in a shell pipeline.
pub(crate) fn run_dump_config() {
    #[cfg(not(debug_assertions))]
    {
        eprintln!(
            "skribisto: {} is available in debug builds only",
            crate::shell::instance::DUMP_CONFIG_FLAG
        );
        std::process::exit(2);
    }
    #[cfg(debug_assertions)]
    {
        let Some(path) = general_settings_path() else {
            eprintln!("skribisto: cannot resolve the configuration directory");
            std::process::exit(2);
        };
        print!("{}", settings_keys::dump(&path));
    }
}

/// `--config <file>`: validate a pins file and merge it into the settings store.
///
/// Debug-only, and deliberately fatal on any problem. The whole point of the flag
/// is that a probe knows what state it is in; a run that was asked to pin settings
/// and silently pinned none would assert against a state it never reached, which
/// is the failure mode the schema exists to remove — see `settings_keys`' module
/// docs for the one that cost a wrong diagnosis of teksilo's i18n layer.
///
/// Prints the file it writes to. That path is the flag's one sharp edge: pins land
/// in whatever configuration directory this process resolves, so a run against the
/// operator's own `XDG_CONFIG_HOME` changes their real settings. Isolation is the
/// caller's to arrange (`automation_fixture.isolated_config` does it), so the least
/// this can do is say out loud which directory it just wrote into.
#[cfg_attr(not(debug_assertions), allow(unused_variables))]
pub(crate) fn apply_config_pins(path: &str) {
    #[cfg(not(debug_assertions))]
    {
        eprintln!(
            "skribisto: {} is available in debug builds only",
            crate::shell::instance::CONFIG_FLAG
        );
        std::process::exit(2);
    }
    #[cfg(debug_assertions)]
    {
        let Some(general) = general_settings_path() else {
            eprintln!("skribisto: cannot resolve the configuration directory");
            std::process::exit(2);
        };

        let pins = match settings_keys::load_pins(std::path::Path::new(path)) {
            Ok(pins) => pins,
            Err(e) => {
                eprintln!("skribisto: {e}");
                std::process::exit(2);
            }
        };
        if let Err(e) = settings_keys::merge_into(&general, &pins) {
            eprintln!("skribisto: {e}");
            std::process::exit(2);
        }

        eprintln!(
            "skribisto: pinned {} setting{} from {path} into {}",
            pins.len(),
            if pins.len() == 1 { "" } else { "s" },
            general.display()
        );
    }
}

/// Best-effort read of persisted theme/locale/autosave/show-welcome; defaults
/// if anything is missing. `show_welcome` is read here (not just via
/// `ctx.settings()` inside `App::build`) because it decides whether a bare
/// launch's *initial window* is the Launcher or a project — a decision made
/// in `main`, before any widget tree (hence any `BuildContext`) exists.
pub(crate) fn read_prefs() -> (bool, String, bool, bool, bool) {
    let Some(paths) = crate::identity::app_paths() else {
        return (
            false,
            "en-US".to_string(),
            false,
            SPELLCHECK_ENABLED_DEFAULT,
            true,
        );
    };
    // `config_file` appends `.toml`, and the settings bundle opens its K/V
    // store under the name "general" (-> general.toml). Pass the bare name
    // here too, otherwise this reads `general.toml.toml` and never sees the
    // values the settings panel wrote, so prefs don't restore on restart.
    match SettingsStore::open(paths.config_file("general")) {
        Ok(store) => (
            store.signal(DARK_KEY, false).get(),
            store.signal(LOCALE_KEY, "en-US".to_string()).get(),
            store.signal(AUTOSAVE_KEY, false).get(),
            store
                .signal(SPELLCHECK_ENABLED_KEY, SPELLCHECK_ENABLED_DEFAULT)
                .get(),
            store.signal(SHOW_WELCOME_KEY, true).get(),
        ),
        Err(_) => (
            false,
            "en-US".to_string(),
            false,
            SPELLCHECK_ENABLED_DEFAULT,
            true,
        ),
    }
}
