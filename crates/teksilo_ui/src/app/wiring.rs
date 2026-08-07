// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Event wiring installed once per `App::build` — the subscriptions that keep the app's
//! view-models in step with the backend.
//!
//! Distinct from [`super::commands`]: those register *verbs the user can fire*, these
//! register *reactions to things that happened*. Both were inline in `App::build`, which is
//! why that function ran to two thousand lines.
//!
//!   * [`long_ops`] — routing the `Origin::LongOperation` event stream to whichever
//!     view-model owns the operation in flight.
//!   * [`spellcheck`] — keeping the checker and the open documents honest as dictionaries,
//!     personal words and the theme change.
//!   * [`guards`] — multi-work event filters so every window does not react to a sibling's
//!     Load/New/Close.
//!   * [`window_bind`] — register a window against its Work (ordinal + toast audience).
//!   * [`project_events`] — Load/New/Close/Attach lifecycle subscribers.
//!   * [`save_and_exit`] — deferred close/switch resumption off long-op save events.

pub(super) mod guards;
pub(super) mod long_ops;
pub(super) mod project_events;
pub(super) mod save_and_exit;
pub(super) mod spellcheck;
pub(super) mod window_bind;
