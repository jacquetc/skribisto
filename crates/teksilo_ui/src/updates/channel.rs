// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How this binary was distributed, and whether that means it should speak.
//!
//! Stamped at build time into `SKRIBISTO_CHANNEL` by `build.rs`; see the
//! reasoning there for why this cannot be worked out at runtime.
//!
//! ## The policy, and why silence is the majority answer
//!
//! Telling a reader to update when they cannot act on it, or when something else
//! is already doing it for them, is the way this feature ships broken. Two
//! channels are therefore silent by design:
//!
//! - **Flathub** updates itself, and GNOME Software and KDE Discover both notify
//!   about pending updates on their own. A second notice is duplication, and it
//!   would arrive before Flathub had even built the release.
//! - **A distribution package** is on the distributor's schedule, not this
//!   project's. Its reader cannot install an upstream build without leaving the
//!   package manager, and the version they are told about may never be packaged.
//!
//! Every remaining channel has no updater of any kind, which makes the
//! application the only signal its reader will ever get.
//!
//! `Source` is the local-build case. It does not check on its own, so a
//! `cargo run`, a test and a CI job all make no network request at all. The Help
//! menu row is still offered there, which is what leaves a way to exercise the
//! feature by hand without any channel pretending to be a shipped artifact.

/// How this copy of the application was installed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Installed from Flathub, which keeps it up to date.
    Flathub,
    /// The `.flatpak` bundle attached to a GitHub release. Installs like a
    /// Flatpak but is not connected to any repository, so nothing updates it.
    FlatpakBundle,
    /// The standalone Linux tarball.
    Tarball,
    /// The macOS disk image.
    Dmg,
    /// Windows, either the installer or the portable zip.
    ///
    /// One value for both because one `cargo build` produces both artifacts, so
    /// they physically cannot carry different stamps. Nothing is lost: neither
    /// updates itself, so the policy and the wording are the same for both.
    Windows,
    /// Packaged by a distribution, on the distributor's schedule.
    Distro,
    /// Built from source, including every `cargo build` in this workspace.
    Source,
    /// `SKRIBISTO_CHANNEL` held something this build does not recognise.
    ///
    /// Reached when a packaging recipe is newer than the application it is
    /// packaging. Treated exactly like [`Channel::Source`]: silent.
    Unknown,
}

impl Channel {
    /// The channel this binary was built for.
    pub fn current() -> Self {
        Self::parse(env!("SKRIBISTO_CHANNEL"))
    }

    /// Read a stamp. Separated from [`Self::current`] so the table is testable
    /// without rebuilding the crate under nine different environments.
    pub fn parse(stamp: &str) -> Self {
        match stamp.trim() {
            "flathub" => Self::Flathub,
            "flatpak-bundle" => Self::FlatpakBundle,
            "tarball" => Self::Tarball,
            "dmg" => Self::Dmg,
            "windows" => Self::Windows,
            "distro" => Self::Distro,
            "source" | "" => Self::Source,
            _ => Self::Unknown,
        }
    }

    /// Whether this channel may check on its own, without being asked.
    ///
    /// Governs the unprompted daily check only. Asking by hand, through the Help
    /// menu, is answered on any channel that [shows update state at
    /// all](Self::shows_update_state).
    pub fn checks_automatically(self) -> bool {
        matches!(
            self,
            Self::FlatpakBundle | Self::Tarball | Self::Dmg | Self::Windows
        )
    }

    /// Whether the application offers any update surface at all here.
    ///
    /// False on the two channels somebody else is responsible for. There, the
    /// Help menu row is absent and the version line stays a version line, so
    /// nothing offers a reader an action their install cannot take.
    pub fn shows_update_state(self) -> bool {
        !matches!(self, Self::Flathub | Self::Distro)
    }

    /// Who keeps this copy current, when it is not the reader. `None` when it is.
    ///
    /// Drives the one explanatory line the About box shows on a managed channel,
    /// so a Flathub reader is told why there is no check rather than left to
    /// wonder whether it is broken.
    pub fn managed_by(self) -> Option<ManagedBy> {
        match self {
            Self::Flathub => Some(ManagedBy::Flathub),
            Self::Distro => Some(ManagedBy::Distribution),
            _ => None,
        }
    }
}

/// Who is responsible for updating a managed install.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedBy {
    Flathub,
    Distribution,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_stamp_the_packaging_writes_is_recognised() {
        // Exactly the values the release workflows set. A typo on either side
        // lands on Unknown, which is silent, so this table is what keeps a
        // shipped artifact from quietly losing its channel.
        for (stamp, expected) in [
            ("flathub", Channel::Flathub),
            ("flatpak-bundle", Channel::FlatpakBundle),
            ("tarball", Channel::Tarball),
            ("dmg", Channel::Dmg),
            ("windows", Channel::Windows),
            ("distro", Channel::Distro),
            ("source", Channel::Source),
        ] {
            assert_eq!(Channel::parse(stamp), expected, "stamp {stamp}");
        }
    }

    #[test]
    fn an_unstamped_build_is_source_and_an_unknown_one_is_silent() {
        assert_eq!(Channel::parse(""), Channel::Source);
        assert_eq!(Channel::parse("   "), Channel::Source);
        assert_eq!(Channel::parse("snap"), Channel::Unknown);
        assert!(!Channel::parse("snap").checks_automatically());
        assert!(!Channel::parse("source").checks_automatically());
    }

    #[test]
    fn the_two_managed_channels_never_check_and_never_show_a_surface() {
        for managed in [Channel::Flathub, Channel::Distro] {
            assert!(!managed.checks_automatically(), "{managed:?}");
            assert!(!managed.shows_update_state(), "{managed:?}");
            assert!(managed.managed_by().is_some(), "{managed:?}");
        }
    }

    #[test]
    fn every_self_managed_channel_checks() {
        // These four have no updater of any kind; the application is the only
        // signal their reader gets, so none of them may be silent.
        for channel in [
            Channel::FlatpakBundle,
            Channel::Tarball,
            Channel::Dmg,
            Channel::Windows,
        ] {
            assert!(channel.checks_automatically(), "{channel:?}");
            assert!(channel.shows_update_state(), "{channel:?}");
            assert!(channel.managed_by().is_none(), "{channel:?}");
        }
    }

    #[test]
    fn a_source_build_offers_the_manual_check_but_never_runs_one() {
        // The manual path has to stay reachable in a development build, or the
        // feature can only be exercised by cutting a release.
        assert!(!Channel::Source.checks_automatically());
        assert!(Channel::Source.shows_update_state());
    }

    #[test]
    fn a_test_run_makes_no_network_request() {
        // `cargo test` compiles with no SKRIBISTO_CHANNEL set. If this ever
        // fails, the suite has started reaching the network.
        assert!(!Channel::current().checks_automatically());
    }
}
