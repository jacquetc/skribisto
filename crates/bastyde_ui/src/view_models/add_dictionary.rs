// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `AddDictionaryViewModel` — the "Add dictionary" form (Settings ▸ Spelling ▸ Dictionaries).
//!
//! A thin form view-model over [`DictionariesViewModel`], the shape of [`ImportPlumeViewModel`]:
//! it owns the four field signals (name, code, `.aff`, `.dic`) and their inline validation, and
//! `add` delegates the actual install to [`DictionariesViewModel::install_user_dictionary`]
//! (validate the pair parses → copy into the download dir as `{code}.aff`/`.dic` → record the
//! name). On success the modal closes and a toast confirms; on refusal the toast names the reason
//! and the form stays open.
//!
//! [`ImportPlumeViewModel`]: crate::view_models::ImportPlumeViewModel

use std::path::Path;
use std::time::Duration;

use bastyde::prelude::*;
use bastyde::widgets::{Toast, ValidationState};

use crate::spellcheck::dictionary_registry;
use crate::view_models::{DictionariesViewModel, InstallDictError};

#[derive(Clone)]
pub struct AddDictionaryViewModel {
    /// The display name the user gives the dictionary.
    name: Signal<String>,
    /// The `dict_language` code its files are stored under (`{code}.aff`/`.dic`).
    code: Signal<String>,
    /// The chosen `.aff` source path.
    aff: Signal<String>,
    /// The chosen `.dic` source path.
    dic: Signal<String>,
    /// The install target — owns the config record + the copy.
    dicts: DictionariesViewModel,
}

impl AddDictionaryViewModel {
    pub fn new(dicts: DictionariesViewModel) -> Self {
        Self {
            name: Signal::new(String::new()),
            code: Signal::new(String::new()),
            aff: Signal::new(String::new()),
            dic: Signal::new(String::new()),
            dicts,
        }
    }

    /// Clear the fields — called when the panel is (re)opened so a previous session doesn't linger.
    pub fn reset_form(&self) {
        self.name.set(String::new());
        self.code.set(String::new());
        self.aff.set(String::new());
        self.dic.set(String::new());
    }

    // ── signal accessors (bound by the view) ──
    pub fn name(&self) -> Signal<String> {
        self.name.clone()
    }
    pub fn code(&self) -> Signal<String> {
        self.code.clone()
    }
    pub fn aff(&self) -> Signal<String> {
        self.aff.clone()
    }
    pub fn dic(&self) -> Signal<String> {
        self.dic.clone()
    }

    /// React to the `.aff` picker (the field itself has already set the `aff` signal): helpfully
    /// default the empty fields — the sibling `.dic` if it exists, the name from the file stem,
    /// and the code from the stem *only* when it isn't a reserved catalogue code (so a stray
    /// `en_US.aff` doesn't pre-fill an invalid code).
    pub fn apply_aff_pick(&self, res: &FileDialogResult) {
        let FileDialogResult::File(Some(path)) = res else {
            return;
        };
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        if self.dic.get().trim().is_empty() {
            let sibling = path.with_extension("dic");
            if sibling.is_file() {
                self.dic.set(sibling.to_string_lossy().into_owned());
            }
        }
        if self.name.get().trim().is_empty() && !stem.is_empty() {
            self.name.set(stem.clone());
        }
        if self.code.get().trim().is_empty()
            && !stem.is_empty()
            && dictionary_registry::resolve_token(&stem).is_none()
        {
            self.code.set(stem);
        }
    }

    // ── inline validation ──
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.name.map(|n| name_state(n))
    }
    /// The code field's validity, including whether a dictionary with that code is **already
    /// installed** — that check needs the runtime installed set (a pure fn can't see it), and it
    /// rebuilds when the set changes so removing a dictionary clears a stale "already installed".
    pub fn code_validation(&self) -> Signal<ValidationState> {
        let dicts = self.dicts.clone();
        self.code.zip(&self.dicts.changed_signal()).map(move |(c, _)| {
            let base = code_state(c);
            if matches!(base, ValidationState::None) && dicts.is_installed_ci(c.trim()) {
                ValidationState::Error(tr!(dict_add_code_taken()))
            } else {
                base
            }
        })
    }
    pub fn aff_validation(&self) -> Signal<ValidationState> {
        self.aff.map(|p| file_state(p))
    }
    pub fn dic_validation(&self) -> Signal<ValidationState> {
        self.dic.map(|p| file_state(p))
    }

    /// Whether "Add" may fire — all four fields valid and the code not already installed.
    pub fn can_add(&self) -> Signal<bool> {
        let name_ok = self.name.map(|n| matches!(name_state(n), ValidationState::None));
        let dicts = self.dicts.clone();
        let code_ok = self.code.zip(&self.dicts.changed_signal()).map(move |(c, _)| {
            matches!(code_state(c), ValidationState::None) && !dicts.is_installed_ci(c.trim())
        });
        let aff_ok = self.aff.map(|p| matches!(file_state(p), ValidationState::None));
        let dic_ok = self.dic.map(|p| matches!(file_state(p), ValidationState::None));
        name_ok.and(&code_ok).and(&aff_ok).and(&dic_ok)
    }

    /// Install the dictionary. On success the modal closes and a toast confirms; on refusal the
    /// toast names the reason and the form stays open so the user can correct it.
    pub fn add(&self, ctx: &mut EventContext) {
        let name = self.name.get();
        let code = self.code.get();
        let aff = self.aff.get();
        let dic = self.dic.get();
        match self.dicts.install_user_dictionary(
            &name,
            &code,
            Path::new(aff.trim()),
            Path::new(dic.trim()),
        ) {
            Ok(()) => {
                ctx.dismiss_modal();
                ctx.show_toast(
                    Toast::success(tr!(dict_add_done(name = name.trim().to_string())))
                        .auto_dismiss_after(Duration::from_secs(4)),
                );
            }
            // Map the typed error to a localized message — an English string in the toast would
            // be half-translated. The form stays open so the user can correct the input.
            Err(e) => {
                let msg = match e {
                    InstallDictError::NameRequired => tr!(dict_add_name_required()),
                    InstallDictError::InvalidCode => tr!(dict_add_code_invalid()),
                    InstallDictError::Reserved => tr!(dict_add_code_reserved()),
                    InstallDictError::AlreadyInstalled => tr!(dict_add_code_taken()),
                    InstallDictError::Unusable(detail) => tr!(dict_add_unusable(error = detail)),
                    InstallDictError::Io(detail) => tr!(dict_add_failed(error = detail)),
                };
                ctx.show_toast(Toast::error(msg));
            }
        }
    }
}

// ── pure field validation (headless-testable) ──

fn name_state(name: &str) -> ValidationState {
    if name.trim().is_empty() {
        ValidationState::Error(tr!(dict_add_name_required()))
    } else {
        ValidationState::None
    }
}

fn code_state(code: &str) -> ValidationState {
    let c = code.trim();
    if c.is_empty() {
        return ValidationState::Error(tr!(dict_add_code_required()));
    }
    let filename_safe = c.chars().any(|ch| ch.is_ascii_alphanumeric())
        && c.chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if !filename_safe {
        return ValidationState::Error(tr!(dict_add_code_invalid()));
    }
    // A custom code must not shadow a catalogue dictionary — those are added via "Get more".
    // Case-insensitive, matching the install guard (a case-variant collides on Windows/macOS).
    if dictionary_registry::collides_with_catalogue(c) {
        return ValidationState::Error(tr!(dict_add_code_reserved()));
    }
    ValidationState::None
}

fn file_state(path: &str) -> ValidationState {
    let p = path.trim();
    if p.is_empty() {
        return ValidationState::Error(tr!(dict_add_file_required()));
    }
    if !Path::new(p).is_file() {
        return ValidationState::Error(tr!(dict_add_file_missing()));
    }
    ValidationState::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_requires_content() {
        assert!(matches!(name_state("  "), ValidationState::Error(_)));
        assert!(matches!(name_state("My French"), ValidationState::None));
    }

    #[test]
    fn code_rejects_empty_unsafe_and_reserved() {
        assert!(matches!(code_state(""), ValidationState::Error(_)), "empty");
        assert!(matches!(code_state("a/b"), ValidationState::Error(_)), "separator");
        assert!(matches!(code_state("--"), ValidationState::Error(_)), "no alphanumeric");
        // `fr-FR` is a real catalogue code → reserved; so is a case variant (Win/macOS collide).
        assert!(matches!(code_state("fr-FR"), ValidationState::Error(_)), "reserved");
        assert!(matches!(code_state("FR-FR"), ValidationState::Error(_)), "reserved case-variant");
        // A genuinely custom code is accepted.
        assert!(matches!(code_state("fr-FR-x-mine"), ValidationState::None));
    }

    #[test]
    fn file_state_wants_an_existing_file() {
        assert!(matches!(file_state("  "), ValidationState::Error(_)), "blank");
        assert!(matches!(file_state("/no/such/file.aff"), ValidationState::Error(_)));
        let f = std::env::temp_dir().join(format!("skrib-fs-{}.aff", std::process::id()));
        std::fs::write(&f, b"SET UTF-8\n").unwrap();
        assert!(matches!(file_state(&f.to_string_lossy()), ValidationState::None));
        let _ = std::fs::remove_file(&f);
    }
}
