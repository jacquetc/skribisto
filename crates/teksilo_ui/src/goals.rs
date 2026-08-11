// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Word and character count **targets**: the arithmetic, the measurement, and the one
//! action that writes several at once.
//!
//! `BinderItem` has carried `word_count_goal` and `char_count_goal` since the first commit
//! of the format, and the legacy `.skrib` upgrader has always migrated them; what never
//! existed was a way to *set* one. This module is the shared core the surfaces that now do
//! stand on: the Inspector's editor, the Overview's column, the status bar's bar, the
//! container pages, the milestones, and Distribute.
//!
//! Three rules decide everything here, and each of them is a bug avoided rather than a
//! preference:
//!
//! 1. **A container's target is its own number. Targets are never summed, derived or
//!    reconciled.** A `Folder/Book` with a 90 000 target and three chapters targeting
//!    10 000 / nothing / 5 000 is four independent, simultaneously true numbers. Scrivener
//!    sums them, has done since 2007, and it is the single most complained-about behaviour
//!    in the whole field: a folder's total silently jumping because a scene inside it was
//!    given a target of its own. What *does* roll up is **progress** — the actual words
//!    under a container, measured against that container's own number, which is what
//!    "each level encompasses its children" was ever asking for.
//! 2. **One measurement.** [`measure`] is the only walk. Before it, three different live
//!    "words written" numbers existed in this app with three different admission gates,
//!    and any new surface picking the wrong one would have disagreed with the export it
//!    was meant to describe.
//! 3. **`0` means "no target".** Not `Option<i64>` — the sentinel predates this module,
//!    is load-bearing at every existing read site, and carries exactly the same
//!    information. A target of literally zero words has no meaning in a writing app.
//!
//! The four submodules are all pure except [`measure`], which reads the store:
//! [`progress`] turns counts into ratios and semantic colours, [`apportion`] splits one
//! number into several that sum back to it exactly, [`distribute`] turns that into a plan
//! over a container's children, and [`measure`] answers how much is actually written.

pub mod apportion;
pub mod distribute;
/// The confirm-with-preview modal behind "Distribute…".
pub mod distribute_panel;
pub mod format;
pub mod measure;
pub mod progress;

pub use apportion::apportion;
pub use distribute::{Plan, Proposal, Weighting};
pub use format::{format_count, format_goal};
pub use measure::{Child, Counts, Measured};
pub use progress::{bar_fill, ratio, sprint_role, target_role, words_progress};
