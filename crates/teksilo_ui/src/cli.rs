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
///
/// The locale is the one member returned as an `Option`, and it has to be:
/// `None` means *nobody has ever chosen a language*, which is the only state in
/// which [`crate::startup::build_ui_config`] lets teksilo consult the OS. Every
/// other member has a defensible flat default; a language does not, because
/// "the writer wants English" and "the writer has not said" are different
/// facts, and collapsing them is what made a French Windows account launch in
/// English.
pub(crate) fn read_prefs() -> (bool, Option<String>, bool, bool, bool) {
    let Some(paths) = crate::identity::app_paths() else {
        return (false, None, false, SPELLCHECK_ENABLED_DEFAULT, true);
    };
    let general = paths.config_file("general");
    // Read before the store below opens: see `persisted_locale`.
    let locale = persisted_locale(&general);
    // `config_file` appends `.toml`, and the settings bundle opens its K/V
    // store under the name "general" (-> general.toml). Pass the bare name
    // here too, otherwise this reads `general.toml.toml` and never sees the
    // values the settings panel wrote, so prefs don't restore on restart.
    match SettingsStore::open(general) {
        Ok(store) => (
            store.signal(DARK_KEY, false).get(),
            locale,
            store.signal(AUTOSAVE_KEY, false).get(),
            store
                .signal(SPELLCHECK_ENABLED_KEY, SPELLCHECK_ENABLED_DEFAULT)
                .get(),
            store.signal(SHOW_WELCOME_KEY, true).get(),
        ),
        Err(_) => (false, locale, false, SPELLCHECK_ENABLED_DEFAULT, true),
    }
}

/// The interface language the writer has actually chosen, or `None` when the key
/// is absent — the only state in which the OS gets a say.
///
/// Read straight out of the TOML rather than through the [`SettingsStore`] its
/// four neighbours use, and that is not a stylistic preference. `store.signal(key,
/// default)` **seeds** a missing key, and the store writes its whole table back to
/// disk — so asking the store "has the writer chosen a language?" is itself the
/// act of choosing one. That is how the original bug outlived its own fix: the
/// first launch of every install wrote `ui.locale = "en-US"`, turning "nobody has
/// said" into "the writer chose English" before a single window had appeared, and
/// no amount of OS detection can reach a key that is already set. Nothing here
/// opens, seeds or writes anything.
///
/// An empty string is treated as absent: it is not a locale tag, only ever the
/// residue of a seed, and the alternative is a `--dump-config` that reports
/// `ui.locale = ""` as *set*.
fn persisted_locale(general: &std::path::Path) -> Option<String> {
    let text = std::fs::read_to_string(general).ok()?;
    let root = toml::from_str::<toml::Value>(&text).ok()?;
    settings_keys::lookup(&root, LOCALE_KEY)?
        .as_str()
        .map(str::to_owned)
        .filter(|l| !l.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &std::path::Path, body: &str) -> std::path::PathBuf {
        let p = dir.join("general.toml");
        std::fs::write(&p, body).unwrap();
        p
    }

    /// The distinction the whole `Option` exists for, and the one the original
    /// bug erased: a file with no `ui.locale` line has to read back as "nobody
    /// has chosen", not as "English".
    #[test]
    fn an_absent_key_reads_back_as_no_choice() {
        let dir = tempfile::tempdir().unwrap();
        let general = write(dir.path(), "ui.dark = false\n");
        assert_eq!(persisted_locale(&general), None);
    }

    /// …as does a file that does not exist at all — the true first launch.
    #[test]
    fn a_missing_file_reads_back_as_no_choice() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(persisted_locale(&dir.path().join("general.toml")), None);
    }

    /// A choice the writer made wins, and must keep winning on a machine whose
    /// OS says otherwise — the regression that turning OS detection on risks.
    #[test]
    fn a_chosen_language_reads_back_verbatim() {
        let dir = tempfile::tempdir().unwrap();
        let general = write(dir.path(), "[ui]\nlocale = \"en-US\"\n");
        assert_eq!(persisted_locale(&general), Some("en-US".to_string()));
        let general = write(dir.path(), "[ui]\nlocale = \"fr-FR\"\n");
        assert_eq!(persisted_locale(&general), Some("fr-FR".to_string()));
    }

    /// Dotted and nested spellings are the same key — `--config` writes the
    /// nested form, a hand-written pins file the dotted one.
    #[test]
    fn the_dotted_spelling_is_the_same_key() {
        let dir = tempfile::tempdir().unwrap();
        let general = write(dir.path(), "ui.locale = \"fr-FR\"\n");
        assert_eq!(persisted_locale(&general), Some("fr-FR".to_string()));
    }

    /// An empty string is residue, not a choice. No locale tag is empty, and
    /// reporting one as *set* would be a `--dump-config` that lies twice over.
    #[test]
    fn an_empty_value_is_not_a_choice() {
        let dir = tempfile::tempdir().unwrap();
        let general = write(dir.path(), "[ui]\nlocale = \"\"\n");
        assert_eq!(persisted_locale(&general), None);
    }

    /// Nothing here may create or touch the file: reading the setting must not
    /// be the act of writing it, which is the defect this function replaced.
    #[test]
    fn reading_never_creates_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let general = dir.path().join("general.toml");
        assert_eq!(persisted_locale(&general), None);
        assert!(
            !general.exists(),
            "reading a missing file must not create it"
        );

        std::fs::write(&general, "ui.dark = false\n").unwrap();
        let before = std::fs::read_to_string(&general).unwrap();
        assert_eq!(persisted_locale(&general), None);
        assert_eq!(
            std::fs::read_to_string(&general).unwrap(),
            before,
            "reading must leave the file byte-identical"
        );
    }

    /// A corrupt file is not a reason to refuse to launch; the writer simply has
    /// no recorded choice, and the OS gets its say.
    #[test]
    fn an_unparseable_file_reads_back_as_no_choice() {
        let dir = tempfile::tempdir().unwrap();
        let general = write(dir.path(), "this is not = = toml\n");
        assert_eq!(persisted_locale(&general), None);
    }

    /// A non-string sitting at the key (an older schema, a hand-edit) is not a
    /// language either.
    #[test]
    fn a_non_string_value_is_not_a_choice() {
        let dir = tempfile::tempdir().unwrap();
        let general = write(dir.path(), "[ui]\nlocale = 42\n");
        assert_eq!(persisted_locale(&general), None);
    }
}
