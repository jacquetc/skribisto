// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `res!()` SVG factories, grouped by where the icon appears. Pure asset lookups — no
//! state, no logic. The binder's own icons live with that feature, in
//! [`crate::binder::icons`], because they are chosen from a `(role, sub_role)` rather than
//! being a fixed set.

pub(crate) mod activity;
pub(crate) mod comments;
pub(crate) mod editor;
pub(crate) mod find;
pub(crate) mod format;
pub(crate) mod go;
pub(crate) mod session;
