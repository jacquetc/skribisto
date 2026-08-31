// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Event wiring installed once per `App::build` — the subscriptions that keep the app's
//! view-models in step with the backend.
//!
//! Distinct from [`super::commands`]: those register *verbs the user can fire*, these
//! register *reactions to things that happened*. Both were inline in `App::build`, which is
//! why that function ran to two thousand lines.
//!
//!   * [`focus_sync`] — Image-menu visibility, the Insert-Template submenu refill,
//!     long-operation event routing, Document-menu selection gating, per-pane
//!     focus/flush, and the BinderItem-Updated resync — six blocks that all run
//!     consecutively right after the seed, grouped by *when* rather than *what*.
//!   * [`long_ops`] — routing the `Origin::LongOperation` event stream to whichever
//!     view-model owns the operation in flight.
//!   * [`spellcheck`] — keeping the checker and the open documents honest as dictionaries,
//!     personal words and the theme change.
//!   * [`guards`] — multi-work event filters so every window does not react to a sibling's
//!     Load/New/Close.
//!   * [`window_bind`] — register a window against its Work (ordinal + toast audience).
//!   * [`project_events`] — Load/New/Close/Attach lifecycle subscribers.
//!   * [`save_and_exit`] — deferred close/switch resumption off long-op save events.
//!   * [`autosave`] — the comments-visibility mirror plus the dirty→disk and periodic
//!     backup countdown timers.
//!   * [`punctuation`] — the Work's house-style row (plus the app-level fallback)
//!     pushed into every open editor.
//!   * [`footnotes`] — the per-window footnotes view-model: construction, backend
//!     wiring and the renumber-on-edit hookup.
//!   * [`shared_view_models`] — get-or-create for the search/trash/comments view-models
//!     the docking rail shares across builds.

pub(super) mod autosave;
pub(super) mod exchange;
pub(super) mod focus_sync;
pub(super) mod footnotes;
pub(super) mod guards;
pub(super) mod long_ops;
pub(super) mod project_events;
pub(super) mod prose_repair;
pub(super) mod punctuation;
pub(super) mod save_and_exit;
pub(super) mod shared_view_models;
pub(super) mod spellcheck;
pub(super) mod window_bind;
