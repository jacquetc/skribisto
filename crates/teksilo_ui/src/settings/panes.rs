// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One module per Settings page body.
//!
//! `settings.rs` keeps the *shell* — the category tree, the search field, the
//! pane-switcher and the footer — and each page's form lives here, one `*_pane` function per
//! page (29 modules today; `backup` carries two pages, the app-level one and the Work one).
//! Each reaches the shell's shared helpers (`field_label`, `hint`, `group`, `slider_field`,
//! `pane_frame`, and the [`Crumbs`](super::Crumbs) handle every page asks for its own
//! breadcrumb) through `use super::super::*` — child modules can see a parent's private items,
//! so none of those had to be widened for this.
//!
//! [`overview`] is the one module that serves more than one page: every parent — each section
//! and the nested Typography group — renders through it, from the same tree spec the tree
//! itself is built from.
//!
//! No page is an `empty_pane` placeholder any more: the one that was (Menus & Toolbars) had
//! nothing to set and no plan to gain anything, so its `Pane` variant was deleted rather than
//! left as a row that answers a click with an apology. `empty_pane` itself is kept — it is
//! still what a Work page renders with no project open.
//! Keymap hosts Teksilo's [`ShortcutSettings`](teksilo::widgets::ShortcutSettings);
//! Notifications hosts the toast archive [`NotificationLog`](teksilo::widgets::NotificationLog).
//!
//! A settings page is a page in the Settings window — its chrome, navigation and lifecycle
//! belong to this shell — so every page sits here rather than being scattered across the
//! feature directories their view-models live in.

pub(super) mod appearance;
pub(super) mod autosave;
pub(super) mod backup;
pub(super) mod corkboard;
pub(super) mod dictionaries;
pub(super) mod distraction_free;
pub(super) mod distraction_free_themes;
pub(super) mod editor_behavior;
pub(super) mod export_styles;
pub(super) mod games;
pub(super) mod goals;
pub(super) mod keymap;
pub(super) mod margin_lane;
pub(super) mod notifications;
pub(super) mod overview;
pub(super) mod paratext;
pub(super) mod punctuation;
pub(super) mod spellcheck;
pub(super) mod text_replacements;
pub(crate) mod typography;
pub(super) mod user;
pub(super) mod user_dictionary;
pub(super) mod work_author;
pub(super) mod work_language;
pub(super) mod work_punctuation;
pub(super) mod work_statuses;
pub(super) mod work_structure;
pub(super) mod work_tags;
pub(super) mod work_templates;
