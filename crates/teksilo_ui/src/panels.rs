// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One-off modal panels with no feature directory of their own.
//!
//! Each is a self-contained dialog: the About box, first-run's one-time-copy offer, the
//! licence viewer, and the Import documents wizard's epigraph-cell helper widget. Panels
//! that *do* belong to a feature live with it instead — the backup chooser under
//! [`crate::backup`], the trash destination picker under [`crate::trash`], the Launcher
//! window body under [`crate::welcome`], New Work under [`crate::new_work`], Import Plume
//! Creator under [`crate::import_plume`], and the Import documents wizard under
//! [`crate::import_document`].

pub(crate) mod about;
pub(crate) mod first_run;
pub(crate) mod import_epigraph_cell;
pub(crate) mod license;
