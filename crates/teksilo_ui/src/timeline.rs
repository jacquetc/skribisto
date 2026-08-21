// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Timeline feature: the bottom band showing the whole project's past.
//!
//! [`TimelineViewModel`] compares every recorded moment against the live
//! manuscript, matched by durable `uid` — the one surface that reaches a row
//! which was written, recorded, and later deleted, because it does not depend on
//! a live `BinderItem` the way the Versions dock does. Both sides digest their
//! prose through [`crate::models::digest_of`], so the two can only ever disagree
//! about the text itself. [`timeline_axis`] turns however many recorded moments
//! there are into a band-sized set of bars, bucketing into calendar periods above
//! [`timeline_axis::MAX_BARS`]. [`dock`] is the band itself.
//!
//! Sibling of [`crate::versions`], which this shares [`crate::versions::ProjectHandle`]
//! with (where a project's history is kept) and [`crate::models::digest_of`]
//! with (how a version's prose is compared): a version lands on the focused row's
//! own past, this on the project's.

pub mod dock;
pub mod timeline_axis;
mod timeline_vm;

pub use timeline_axis::{Axis, axis_for};
pub use timeline_vm::{
    ChangeKind, LiveManuscriptFn, LiveProseFn, Moment, PastProse, RowChange, TimelineViewModel,
};
