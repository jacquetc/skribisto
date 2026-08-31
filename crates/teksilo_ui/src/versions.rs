// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Versions feature: the trailing-rail dock showing one row's own past.
//!
//! [`VersionsViewModel`] scans both `skrib_format::versions` sources — the
//! in-project history log and any reachable backup — for one focused row's
//! timeline, keyed by its durable `uid` rather than its live store id.
//! [`version_diff`] is the pure block-then-word comparison the dock renders a
//! selected version against its predecessor with, testable with no document or
//! store. [`version_restore`] resolves *where* a past version's text belongs in
//! the row as it stands today — a promote can retype the row since the version
//! was taken — but not the guarded sequence that actually confirms, backs up and
//! writes: that lives in `app::restore_version`, since it needs peers (the
//! open-documents store, the backup scheduler, this Work's undo stack) a
//! view-model may not import. [`dock`] is the panel itself.

pub mod dock;
pub mod version_diff;
pub mod version_restore;
mod versions_vm;

pub use version_diff::VersionDiff;
pub use version_restore::RestoreRequest;
pub use versions_vm::{Pins, ProjectHandle, TimelineView, VersionScope, VersionsViewModel};
