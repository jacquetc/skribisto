// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project lifecycle event wiring — Load / New / Close / Attach.
//!
//! Extracted from `App::build` so that god-function no longer owns the
//! multi-window binding policy. The heavy LoadWork backup-sniff subscriber and
//! attach seed still live inline in `App::build` where they share local
//! captures with the rest of first-build setup; this module owns the shared
//! helpers those sites use ([`super::window_bind`], [`super::guards`]).

pub(in crate::app) use super::guards::{on_own_close, on_own_load_or_new};
pub(in crate::app) use super::window_bind::bind_window_to_work;
