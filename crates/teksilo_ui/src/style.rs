// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The design language the app is built in, chosen once per run from `--style`.
//!
//! Teksilo ships four complete presets — its own IntUI baseline plus the
//! [`Fluent`](AppStyle::Fluent) (WinUI 3), [`Material3`](AppStyle::Material3)
//! and [`MacOs`](AppStyle::MacOs) sibling crates. Each is a whole design
//! language: colour, shape, typography and motion tokens **plus** Tier-3 widget
//! chrome (Fluent's elevation edge and list-selection pill, Material 3's pill
//! buttons, and so on).
//!
//! ## Why the CLI and not the Settings window
//!
//! Because a runtime switch cannot deliver what the name promises. Colour tokens
//! resolve at paint time and retint instantly, but **widget chrome is resolved
//! when a widget is built** — so switching family in a live window changes the
//! palette and keeps the previous shapes. The result looks like a half-applied
//! theme, and every widget already on screen is evidence against the setting
//! that claims to have changed it.
//!
//! A launch flag has no such gap: the style is installed before the first
//! `AppContext` exists, so every widget in the process is built in it. That is
//! also why this is not a settings key — a key implies the Settings window can
//! change it, and `--dump-config` / `--config` would then advertise a pin the
//! app cannot honour without a restart.
//!
//! Light ↔ dark is a different question and stays exactly where it was: the
//! Settings window's Light / Dark / System picker, persisted in
//! [`crate::settings_keys::DARK_KEY`]. Those three entries are built from the
//! **active style's** own light and dark themes ([`light`] / [`dark`]), so
//! picking Light under `--style fluent` gives Fluent Light rather than dropping
//! the run back to IntUI.
//!
//! ⚠ The one entry that does leave the style is **System**, and deliberately so:
//! `EventContext::follow_system_theme` adopts the desktop's own colours (or
//! falls back to the IntUI presets off a colour-reporting desktop). It is the
//! framework's contract, it is what the entry has always meant, and a writer who
//! asks to follow the OS is asking for exactly that.
//!
//! ## Reading it
//!
//! [`active`] answers everywhere, so a call site never has to be handed the
//! style. It is process-wide (Tier 1 — the style cannot differ between two
//! windows of one process, since both are built from the same slot), installed
//! once by [`install`] at the top of [`crate::run`], and defaults to
//! [`AppStyle::IntUi`] when no flag was given — which is byte for byte what the
//! app did before this module existed.

use std::sync::{LazyLock, RwLock};

use teksilo::core::styles::Theme;

/// A complete design language the app can be built in.
///
/// The default is [`IntUi`](Self::IntUi) — Teksilo's own baseline preset, and
/// the only one that needs no Cargo feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppStyle {
    /// Teksilo's built-in Int UI baseline. The app's default.
    #[default]
    IntUi,
    /// Fluent — Windows 11 / WinUI 3.
    Fluent,
    /// Material 3 — Google's design language.
    Material3,
    /// macOS — Apple's Human Interface look.
    MacOs,
}

impl AppStyle {
    /// Every style, in the order `--help` and the error message list them.
    ///
    /// The canonical name is first for each; [`from_name`](Self::from_name)
    /// additionally accepts the aliases named there.
    pub const ALL: &'static [AppStyle] = &[
        AppStyle::IntUi,
        AppStyle::Fluent,
        AppStyle::Material3,
        AppStyle::MacOs,
    ];

    /// The canonical `--style` name.
    pub fn name(self) -> &'static str {
        match self {
            AppStyle::IntUi => "intui",
            AppStyle::Fluent => "fluent",
            AppStyle::Material3 => "material3",
            AppStyle::MacOs => "macos",
        }
    }

    /// Resolve a `--style NAME` value, case-insensitively.
    ///
    /// `m3` is accepted for `material3` (the catalog's alias) and `int-ui` /
    /// `intui` both name the baseline. Anything else is `None`, which the
    /// caller turns into a hard startup error rather than a silent fallback —
    /// a run that was asked for a style and quietly gave the default is the
    /// same silence [`crate::settings_keys`] exists to remove.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "intui" | "int-ui" | "default" => Some(AppStyle::IntUi),
            "fluent" | "winui" => Some(AppStyle::Fluent),
            "material3" | "m3" | "material" => Some(AppStyle::Material3),
            "macos" | "mac" => Some(AppStyle::MacOs),
            _ => None,
        }
    }

    /// This style's light theme.
    pub fn light(self) -> Theme {
        use teksilo::prelude::{fluent, macos, material3};
        use teksilo::presets::intui;
        match self {
            AppStyle::IntUi => intui::light(),
            AppStyle::Fluent => fluent::light(),
            AppStyle::Material3 => material3::light(),
            AppStyle::MacOs => macos::light(),
        }
    }

    /// This style's dark theme.
    pub fn dark(self) -> Theme {
        use teksilo::prelude::{fluent, macos, material3};
        use teksilo::presets::intui;
        match self {
            AppStyle::IntUi => intui::dark(),
            AppStyle::Fluent => fluent::dark(),
            AppStyle::Material3 => material3::dark(),
            AppStyle::MacOs => macos::dark(),
        }
    }

    /// This style's theme for the requested appearance — the one-liner every
    /// call site that already holds a `dark` boolean wants.
    pub fn theme(self, dark: bool) -> Theme {
        if dark { self.dark() } else { self.light() }
    }
}

/// The canonical names, for `--help` and for the unknown-value error.
pub fn names() -> Vec<&'static str> {
    AppStyle::ALL.iter().map(|s| s.name()).collect()
}

static ACTIVE: LazyLock<RwLock<AppStyle>> = LazyLock::new(|| RwLock::new(AppStyle::IntUi));

/// Install the style for this process. Called once, from [`crate::run`], before
/// `AppContext::new()` and before any window exists.
///
/// Installing later is not an error here but is useless in practice: the tree
/// that is already built keeps the chrome it was built with, which is the very
/// thing [the module docs](self) explain the flag exists to avoid.
pub fn install(style: AppStyle) {
    if let Ok(mut slot) = ACTIVE.write() {
        *slot = style;
    }
}

/// The style this process is built in. [`AppStyle::IntUi`] unless `--style`
/// said otherwise.
pub fn active() -> AppStyle {
    ACTIVE.read().map(|s| *s).unwrap_or_default()
}

/// The active style's light theme.
pub fn light() -> Theme {
    active().light()
}

/// The active style's dark theme.
pub fn dark() -> Theme {
    active().dark()
}

/// The active style's theme for `dark`.
pub fn theme(dark: bool) -> Theme {
    active().theme(dark)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `install` writes a process-wide slot, so the tests that touch it must not
    /// interleave. Mirrors `identity`'s own serialisation for the same reason.
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn the_default_is_the_baseline_preset() {
        assert_eq!(AppStyle::default(), AppStyle::IntUi);
        assert_eq!(AppStyle::default().light().id.as_str(), "intui.light");
    }

    /// Every variant must resolve to a real theme in both appearances, and the
    /// two must differ — a preset whose Cargo feature was dropped would
    /// otherwise fail to compile here rather than at some distant call site.
    #[test]
    fn every_style_resolves_a_distinct_light_and_dark() {
        for style in AppStyle::ALL {
            let (l, d) = (style.light(), style.dark());
            assert!(!l.is_dark(), "{} light reports dark", style.name());
            assert!(d.is_dark(), "{} dark reports light", style.name());
            assert_ne!(
                l.colors.surface_main,
                d.colors.surface_main,
                "{} light and dark share a surface",
                style.name()
            );
        }
    }

    /// The theme ids are what `--style` is really selecting; a preset silently
    /// resolving to another family's theme is the failure this pins down.
    #[test]
    fn each_style_resolves_its_own_family() {
        for (style, prefix) in [
            (AppStyle::IntUi, "intui"),
            (AppStyle::Fluent, "fluent"),
            (AppStyle::Material3, "material3"),
            (AppStyle::MacOs, "macos"),
        ] {
            for theme in [style.light(), style.dark()] {
                assert!(
                    theme.id.as_str().starts_with(prefix),
                    "{} resolved to theme id {}",
                    style.name(),
                    theme.id.as_str()
                );
            }
        }
    }

    #[test]
    fn canonical_names_round_trip() {
        for style in AppStyle::ALL {
            assert_eq!(AppStyle::from_name(style.name()), Some(*style));
        }
    }

    #[test]
    fn names_are_case_insensitive_and_aliased() {
        assert_eq!(AppStyle::from_name("FLUENT"), Some(AppStyle::Fluent));
        assert_eq!(AppStyle::from_name("  Fluent "), Some(AppStyle::Fluent));
        assert_eq!(AppStyle::from_name("m3"), Some(AppStyle::Material3));
        assert_eq!(AppStyle::from_name("mac"), Some(AppStyle::MacOs));
        assert_eq!(AppStyle::from_name("int-ui"), Some(AppStyle::IntUi));
    }

    /// An unknown name resolves to nothing at all, so the caller is forced to
    /// decide what to do rather than being handed a plausible default.
    #[test]
    fn an_unknown_name_does_not_resolve() {
        assert_eq!(AppStyle::from_name("fluid"), None);
        assert_eq!(AppStyle::from_name(""), None);
    }

    #[test]
    fn theme_picks_by_appearance() {
        assert!(AppStyle::Fluent.theme(true).is_dark());
        assert!(!AppStyle::Fluent.theme(false).is_dark());
    }

    #[test]
    fn install_moves_what_active_answers() {
        let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let before = active();
        install(AppStyle::Material3);
        assert_eq!(active(), AppStyle::Material3);
        assert!(light().id.as_str().starts_with("material3"));
        install(before);
    }

    /// `names()` is what the unknown-value error prints; it must stay in step
    /// with the variants rather than being a hand-copied list beside them.
    #[test]
    fn names_covers_every_variant() {
        assert_eq!(names().len(), AppStyle::ALL.len());
        for style in AppStyle::ALL {
            assert!(names().contains(&style.name()));
        }
    }
}
