// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Save / deferred-exit helpers shared by `App::build` and the close guard.
//!
//! The full long-op Completed/Failed subscribers still install from `App::build`
//! (they capture many locals minted only there). Free lifecycle functions that
//! used to sit only in the widget module are re-exported here as the canonical
//! home for "leave this project" orchestration.

// Reserved for further extraction of the pending_exit effect + long-op resume
// blocks out of `App::build`. The free functions in `crate::app` already form
// the public surface (`guard_unsaved_exit`, `close_work_and_return_to_launcher`,
// `abandon_deferred`); call sites should prefer those over inlining.
