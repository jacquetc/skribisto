// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `locale.bundle` slot: how an extension's own translated strings reach the
//! running app.
//!
//! `tr!` resolves its keys against the **calling crate's** `locales/en-US/`, so an
//! extension already writes its strings exactly the way this crate does — its own
//! `.ftl` files, its own compile-time key validation. What it cannot do on its own
//! is get those files into the [`I18nConfig`](teksilo::i18n::I18nConfig) that
//! [`crate::run`] builds, which is what this registry is for.
//!
//! The enabling change was in teksilo: `I18nConfig::compile_in` **extends** rather
//! than replaces, and the manager merges every resource for a locale into one
//! bundle. Before that, a second `compile_in` silently discarded the app's own
//! strings and the window came up in message keys.
//!
//! ## The app always wins a key collision
//!
//! Fluent's `add_resource` keeps the **first** definition of a key, and the app's
//! own resources are pushed before any registered here. So an extension cannot
//! redefine `work-save` out from under the File menu — its own namespaced keys are
//! all that reach the screen. That is a guarantee, not an accident of ordering:
//! the alternative is an installed extension quietly rewriting the app's menus.
//!
//! ## Only locales the app itself supports
//!
//! A bundle for a locale outside [`crate::run`]'s supported set is **dropped, with
//! a message on stderr**. Adding, say, `de-DE` here would make the language
//! selectable while every string outside the extension stayed English, which is a
//! worse outcome than not offering it — and dropping it silently is the failure
//! mode this whole seam keeps guarding against.

use std::sync::{LazyLock, RwLock};

/// One locale's worth of an extension's Fluent resources.
///
/// `resources` are `&'static str` because they are `include_str!`-ed at the
/// extension's compile time, exactly as this crate's own are — an extension does
/// not ship loose `.ftl` files to be discovered at runtime.
#[derive(Clone, Debug)]
pub struct LocaleBundle {
    /// A BCP-47 tag: `"en-US"`, `"fr-FR"`.
    pub locale: String,
    pub resources: Vec<&'static str>,
}

impl LocaleBundle {
    pub fn new(locale: impl Into<String>, resources: Vec<&'static str>) -> Self {
        Self {
            locale: locale.into(),
            resources,
        }
    }
}

struct Registered {
    namespace: String,
    bundles: Vec<LocaleBundle>,
}

// Plain data (`String` + `&'static str`), so unlike the dock, segment and analysis
// registries this one is genuinely `Send` and can be a normal static.
static EXTENSION_LOCALES: LazyLock<RwLock<Vec<Registered>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

/// Add an extension's Fluent resources to the app's i18n configuration.
///
/// ⚠ Read **once**, by [`crate::run`], while it builds the `I18nConfig`. Register
/// before calling `run` — this is the one slot in the seam where "register at
/// startup" is not merely the advice but the only thing that can work, because
/// the config is consumed and never rebuilt.
///
/// Registering the same namespace twice replaces the earlier entry. The returned
/// handle unregisters on drop, which is useful in tests and inert in an
/// application, where registration outlives the process.
pub fn register_locales(namespace: impl Into<String>, bundles: Vec<LocaleBundle>) -> LocaleHandle {
    let namespace = namespace.into();
    let mut reg = EXTENSION_LOCALES.write().unwrap_or_else(|e| e.into_inner());
    reg.retain(|r| r.namespace != namespace);
    reg.push(Registered {
        namespace: namespace.clone(),
        bundles,
    });
    LocaleHandle { namespace }
}

/// Unregisters its bundles when dropped.
#[derive(Debug)]
pub struct LocaleHandle {
    namespace: String,
}

impl Drop for LocaleHandle {
    fn drop(&mut self) {
        let mut reg = EXTENSION_LOCALES.write().unwrap_or_else(|e| e.into_inner());
        reg.retain(|r| r.namespace != self.namespace);
    }
}

/// Every registered bundle whose locale the application actually supports, in
/// registration order.
///
/// `supported` is what [`crate::run`] passes to `I18nConfig::supported_locales`.
/// A bundle for anything else is reported and skipped — see the module docs.
pub fn registered_locales(supported: &[&str]) -> Vec<LocaleBundle> {
    let reg = EXTENSION_LOCALES.read().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    for r in reg.iter() {
        for b in &r.bundles {
            if supported.iter().any(|s| *s == b.locale) {
                out.push(b.clone());
            } else {
                eprintln!(
                    "skribisto: extension '{}' supplied strings for '{}', which this build does \
                     not offer — skipped",
                    r.namespace, b.locale
                );
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUPPORTED: &[&str] = &["en-US", "fr-FR"];

    #[test]
    fn a_registered_bundle_is_returned_for_a_supported_locale() {
        let _h = register_locales(
            "test.supported",
            vec![LocaleBundle::new("fr-FR", vec!["demo-x = Bonjour\n"])],
        );
        let got = registered_locales(SUPPORTED);
        assert!(
            got.iter()
                .any(|b| b.locale == "fr-FR" && b.resources == vec!["demo-x = Bonjour\n"]),
            "a bundle for a supported locale must reach the config"
        );
    }

    /// A locale the app has no strings of its own for is refused rather than
    /// making a half-translated language selectable.
    #[test]
    fn a_bundle_for_an_unsupported_locale_is_dropped() {
        let _h = register_locales(
            "test.unsupported",
            vec![LocaleBundle::new("de-DE", vec!["demo-y = Hallo\n"])],
        );
        assert!(
            !registered_locales(SUPPORTED)
                .iter()
                .any(|b| b.locale == "de-DE"),
            "an unsupported locale must not reach the config"
        );
    }

    #[test]
    fn re_registration_replaces_and_drop_unregisters() {
        {
            let _a = register_locales(
                "test.same-ns",
                vec![LocaleBundle::new("en-US", vec!["demo-a = A\n"])],
            );
            let _b = register_locales(
                "test.same-ns",
                vec![LocaleBundle::new("en-US", vec!["demo-b = B\n"])],
            );
            let got = registered_locales(SUPPORTED);
            assert!(got.iter().any(|b| b.resources == vec!["demo-b = B\n"]));
            assert!(
                !got.iter().any(|b| b.resources == vec!["demo-a = A\n"]),
                "re-registering a namespace must replace, not stack"
            );
        }
        assert!(
            !registered_locales(SUPPORTED)
                .iter()
                .any(|b| b.resources == vec!["demo-b = B\n"]),
            "a dropped handle must leave no bundle behind"
        );
    }
}
