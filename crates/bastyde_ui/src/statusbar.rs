// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status bar's indicators. Each is a thin reactive view over a pure view-model
//! function — the save glyph over `view_models::save_status`, the word count over
//! `view_models::word_count_status`, the writing-session timer over
//! `view_models::writing_session`.

pub(crate) mod notification_bell;
pub(crate) mod save_indicator;
pub(crate) mod session_status_item;
pub(crate) mod word_count_indicator;
