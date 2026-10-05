// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The interface's language as a plain tag.
//!
//! Read where the interface language decides something other than a string: the
//! help topic to show, the language a new project starts in, the word a binder row
//! is named by when its project has no language of its own yet. Three private
//! copies of this existed before it lived here.

/// The active interface locale (`"fr-FR"`), or `None` before the i18n manager
/// exists, which is the case in a headless test.
pub(crate) fn current_tag() -> Option<String> {
    teksilo::i18n::current_locale().map(|sig| sig.get().to_string())
}
