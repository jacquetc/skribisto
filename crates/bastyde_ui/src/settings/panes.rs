// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One module per Settings page body.
//!
//! `settings_panel.rs` keeps the *shell* — the category tree, the search field, the
//! pane-switcher and the footer — and each page's form lives here. Five pages were already
//! their own top-level files (`settings_backup`, `settings_dictionaries`,
//! `settings_export_styles`, `settings_user_dictionary`); these are the nine that were still
//! inline, finishing a split that had been started and left half-done.
//!
//! Each module exposes one `*_pane` function returning the page's widget, and they all reach
//! the shell's shared helpers (`field_label`, `hint`, `group`, `slider_field`, `pane_frame`,
//! `crumb`) through `use super::super::*` — child modules can see a parent's private items,
//! so none of those had to be widened to make this move.
//!
//! Three pages are still `empty_pane` placeholders (Menus & Toolbars, Notifications, Keymap)
//! and have no module until they have a body.
//!
//! The four that were already split live here too, having been top-level `settings_*.rs`
//! files: [`backup`], [`dictionaries`], [`export_styles`] and [`user_dictionary`]. A
//! settings page is a page in the Settings window — its chrome, navigation and lifecycle
//! belong to this shell — so all thirteen sit together rather than being scattered across
//! the feature directories their view-models live in.

pub(super) mod appearance;
pub(super) mod backup;
pub(super) mod autosave;
pub(super) mod corkboard;
pub(super) mod dictionaries;
pub(super) mod editor_behavior;
pub(super) mod export_styles;
pub(super) mod goals;
pub(super) mod spellcheck;
pub(super) mod typography;
pub(super) mod user_dictionary;
pub(super) mod work_author;
pub(super) mod work_language;
pub(super) mod work_structure;
pub(super) mod work_tags;
