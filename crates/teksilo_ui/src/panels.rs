// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One-off modal panels with no feature directory of their own.
//!
//! Each is a self-contained dialog over its own view-model: the Launcher window body, the
//! New Work form, the Plume import form, and the licence viewer. Panels that *do* belong to
//! a feature live with it instead — the backup chooser under [`crate::backup`], the trash
//! destination picker under [`crate::trash`], and so on.

pub(crate) mod about;
pub(crate) mod first_run;
pub(crate) mod import_document;
pub(crate) mod import_plume;
pub(crate) mod license;
pub(crate) mod new_work;
pub(crate) mod welcome;
