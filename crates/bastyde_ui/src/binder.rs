// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The binder tree feature — the outline's icons, its placement math, the binder switcher,
//! and the "＋ Create" vocabulary labels.
//!
//! The tree *widget* itself is a dock ([`crate::docks::outline`]); what lives here is
//! everything that dock and the corkboard both reach for.

pub(crate) mod create_labels;
pub(crate) mod icons;
pub(crate) mod placement;
pub(crate) mod switcher_button;
