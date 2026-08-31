// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The project's own addresses, in one place.
//!
//! Two features offer them now (the Launcher's sidebar and the Help menu), which is
//! what moved them out of `welcome_vm`. Keeping one copy matters more than it looks:
//! these are the addresses a stuck reader is sent to, and a stale one in a released
//! build cannot be fixed from the outside.
//!
//! Every one is `https`, which is not incidental. `open_external_link` allows exactly
//! `http`, `https` and `mailto`, so a link added here in another scheme is refused at
//! the door and toasts at the reader instead of opening.

/// The source repository. Also the Launcher sidebar's GitHub button.
pub const GITHUB_URL: &str = "https://github.com/jacquetc/skribisto";

/// The community chat. Also the Launcher sidebar's Discord button.
pub const DISCORD_URL: &str = "https://discord.gg/5BSkvQmyVH";

/// Where Help ▸ Report a Problem goes.
///
/// The issue tracker rather than an email address, deliberately: a bug report nobody
/// else can read is one the next person to hit it files again.
pub const ISSUES_URL: &str = "https://github.com/jacquetc/skribisto/issues";
