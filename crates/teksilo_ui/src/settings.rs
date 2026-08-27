// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Settings modal — Skribisto's full preferences window.
//!
//! Presented as an in-tree modal (see the `app.settings` action in `app.rs`). A
//! two-pane layout mirroring the design (IntelliJ Int UI vocabulary): a left
//! **category [`TreeView`](teksilo::widgets::TreeView)** (with a
//! [`SearchField`](teksilo::widgets::SearchField) on top) and a right
//! **settings pane** that switches with the tree selection, breadcrumb header +
//! action footer. The header/footer chrome matches the Welcome and New Work
//! panels.
//!
//! All state lives on [`SettingsViewModel`] (persisted signals) plus the
//! framework's drop-in [`ThemeSwitcher`] / [`LanguageSwitcher`] / [`TextScaleControl`]
//! (theme / interface language / text scale — each applies live and persists).
//! The panes bind those signals directly; this view is thin.
//!
//! Categories that don't yet carry settings (today only Menus & Toolbars) render
//! an **empty placeholder**. Keymap embeds Teksilo's `ShortcutSettings`;
//! Notifications embeds the toast archive `NotificationLog`.
//!
//! A **parent** — a section, or the nested Typography group — is a page in its own
//! right: its title, what it is for, and a link to each thing under it
//! (`panes::overview`). Selecting one used to switch nothing at all, so the right
//! pane went on showing whichever leaf was open last and the window disagreed with
//! its own tree.
//! **Instant-apply** (the macOS / GNOME convention): every change takes effect and
//! persists immediately, so there is no Apply/Cancel/OK staged model. The footer
//! carries only *Reset to defaults* (left — enabled only while something differs
//! from the factory defaults, and guarded by a confirmation since there is no undo)
//! and *Done* (right — closes the window). The header ✕ closes it too.

use std::rc::Rc;

use teksilo::core::styles::PanelVariant;
use teksilo::i18n::{LocalizedString, current_locale};
use teksilo::prelude::*;
use teksilo::settings::{SettingsExt, TEXT_SCALE_KEY};
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, FontPicker, FormLayout, HStack, IconButton,
    LanguageSwitcher, MessageBox, MessageBoxButton, MessageBoxButtons, Padding, Panel, RadioButton,
    RadioGroup, Slider, Spacer, StandardButton, TextScaleControl, TextWidget, ThemeSwitcher,
    Toggle, VStack,
};

mod content;
mod defaults;
mod fields;
mod nav;
pub(crate) mod panes;
mod settings_vm;
mod text_replacement_rules_vm;
mod tree;
mod tree_expansion_vm;
mod work_settings_vm;

// Re-bound here, not merely `use`d, because every pane module reaches this file
// with `use super::super::*` — a glob that carries a private import along with
// the rest. Naming them explicitly is what keeps that glob honest: the list
// below is the settings window's whole internal vocabulary.
pub(crate) use defaults::build_not_defaults;
pub(crate) use fields::{
    crumb, empty_pane, field_label, group, hint, index_to_method, method_to_index, pane_frame,
    section_title, slider_field, slider_field_tipped,
};
pub(crate) use nav::{Navigator, Pane, Sec, tree_spec};

pub use settings_vm::{
    CorkboardDefaults, EditorTypography, EditorTypographySet, EditorViewMemory, SettingsViewModel,
    TypographyKind, TypographySizeRange,
};
pub use text_replacement_rules_vm::TextReplacementRulesViewModel;
pub use tree_expansion_vm::TreeExpansionViewModel;
pub use work_settings_vm::WorkSettingsViewModel;

use crate::sessions::WorkSession;
use crate::shared::{HighlightScope, TypewriterAnchor};
use skribisto_model::ChapterMode;

/// Card dimensions (a compact two-pane preferences window).
const CARD_W: f32 = 920.0;
const CARD_H: f32 = 620.0;
const HEADER_H: f32 = 44.0;
const FOOTER_H: f32 = 56.0;
const TREE_W: f32 = 262.0;
/// Body height = card − header − the 1 px rule beneath it.
const BODY_H: f32 = CARD_H - HEADER_H - 1.0;
/// The default text-scale factor (framework `TEXT_SCALE_KEY` baseline).
const TEXT_SCALE_DEFAULT: f32 = 1.0;

pub struct SettingsPanel {
    /// The active page — a panel field so it survives rebuilds (and seeds the
    /// tree selection + the content `Switcher`). Defaults to Editor ▸ Scene.
    selected_pane: Signal<Pane>,
    root_child: Option<WidgetId>,
    /// The Tier-2 bundle for the Work the OPENING window shows.
    ///
    /// Never resolve the equivalent state via `ctx.app_state::<SingleWork/
    /// SingleWorkInfo/AppIds/TagsViewModel/UserDictionaryViewModel>()` — that
    /// slot is one per process, seeded from the first window's session, so a
    /// second open Work would silently read/write the wrong project's
    /// settings. Same pattern as `SaveAsViewModel`/`BackupRestoreViewModel`.
    session: WorkSession,
}

impl SettingsPanel {
    pub fn new(session: WorkSession) -> Self {
        Self::opening_at(Pane::SceneTypography, session)
    }

    /// Open straight to Spelling ▸ Dictionaries — the target of the "install the
    /// missing dictionaries" toast (`offer_missing_dictionaries`). The tree seeds
    /// its selection to that page and expands the owning section on open.
    pub fn open_to_dictionaries(session: WorkSession) -> Self {
        Self::opening_at(Pane::Dictionaries, session)
    }

    /// Open straight to Editor ▸ Writing games — the target of the games dock's
    /// own "Writing game settings…" button, which promises that page by name.
    pub fn open_to_games(session: WorkSession) -> Self {
        Self::opening_at(Pane::Games, session)
    }

    /// Open straight to Backup & Sync ▸ Backup — the target of the
    /// "no backups configured" nudge toast.
    pub fn open_to_backup(session: WorkSession) -> Self {
        Self::opening_at(Pane::Backup, session)
    }

    /// Open straight to Settings ▸ User — the target of the "your comments are
    /// unsigned" toast (`crate::app::warn_unsigned_comments`), which promises
    /// that page by name.
    pub fn open_to_user(session: WorkSession) -> Self {
        Self::opening_at(Pane::User, session)
    }

    /// Open straight to Keymap — the target of the Help ▸ Keyboard shortcuts sheet's
    /// own "Change shortcuts…" button, which promises that page by name. The sheet
    /// itself is read-only, so this is where a reader who wanted to *change* a chord
    /// rather than look one up ends up.
    pub fn open_to_keymap(session: WorkSession) -> Self {
        Self::opening_at(Pane::Keymap, session)
    }

    fn opening_at(pane: Pane, session: WorkSession) -> Self {
        Self {
            selected_pane: Signal::new(pane),
            root_child: None,
            session,
        }
    }
}

impl std::fmt::Debug for SettingsPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsPanel").finish()
    }
}

impl Widget for SettingsPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = SettingsViewModel::new(ctx.settings());
        // Which surfaces a game covers is an app setting; whether it is being
        // played is this project's session state. Paired here, exactly as
        // `App::build` pairs them for the editors.
        let games = crate::writing_session::WritingGamesViewModel::new(
            self.session.always_forward.clone(),
            crate::writing_session::WritingGameOptions::new(
                vm.games_forward_prose(),
                vm.games_forward_synopsis(),
            ),
        );
        let scale = ctx.settings().signal_for(&TEXT_SCALE_KEY);
        let theme_sig = ctx.theme_signal().clone();
        let locale_sig = current_locale();

        // Instant-apply: settings take effect + persist the moment they change, so
        // there is no Apply/Cancel/OK. The footer's Reset-to-defaults is enabled
        // only while something differs from the factory defaults.
        let not_defaults = build_not_defaults(&theme_sig, &locale_sig, &scale, &vm);

        // The OPENING WINDOW's own Work (never `ctx.app_state`, see this struct's
        // `session` field doc) backs the "Work: `<name>` ▸ Structure" page. When
        // no project is open yet, the page is present in the Switcher but its
        // tree node isn't shown, so it renders an empty placeholder — same
        // `w.id().is_some()` gate as before, just sourced from this window's own
        // session instead of a process-wide `app_state` slot.
        let work = Some(self.session.single_work.clone());
        let stack = self.session.ids.stack_id.clone();
        let work_title = work.as_ref().map(|w| w.title().get()).unwrap_or_default();

        // The shape of the whole window, resolved once: the tree walks it, the
        // parents' pages read their children off it, and the search index takes
        // its parents from it. Both variables it depends on are snapshots taken
        // as this window is built — a Work opened later gets its section when the
        // Settings window is next opened, and the extension registry is a
        // snapshot by design (see `settings_ext`).
        let extension_ids: Vec<&'static str> = crate::settings_ext::registered_pages()
            .iter()
            .map(|p| p.id)
            .collect();
        let spec = tree_spec(self.session.single_work.id().is_some(), &extension_ids);
        // The two Work pages edit the *entity*, not the settings store, so they go through
        // their own view-model rather than calling `SingleWork::set_*` + `save` from a pane.
        // THIS WINDOW's own session (never `ctx.app_state`), same reasoning as `work` above.
        let punctuation = Some(self.session.smart_punctuation.clone());
        let work_vm = work
            .as_ref()
            .zip(punctuation.as_ref())
            .map(|(w, p)| WorkSettingsViewModel::new(w.clone(), p.clone(), stack.clone()));
        let (left, content) = content::build(
            ctx,
            &self.session,
            self.selected_pane.clone(),
            &vm,
            scale.clone(),
            &games,
            spec,
            &work_vm,
            &work,
            work_title,
        );

        let footer = self.footer(vm, scale, not_defaults);
        let right = VStack::new()
            .spacing(0.0)
            .child(Expand::vertical().child(content))
            .child(Expand::horizontal().child(Divider::new()))
            .child(FixedSize::new().height(FOOTER_H).child(footer));

        // ── Header strip (title + close), mirroring the Welcome panel ───────
        let header = FixedSize::new().height(HEADER_H).child(
            Padding::symmetric(8.0, 14.0).child(
                HStack::new()
                    .spacing(8.0)
                    .child(
                        Expand::horizontal().child(
                            TextWidget::new(tr!(settings_title()))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Secondary),
                        ),
                    )
                    .child(
                        IconButton::clear()
                            .tooltip(tr!(settings_close()))
                            .on_activate_fn(|ctx| ctx.dismiss_modal()),
                    ),
            ),
        );

        let root = teksu!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        Expand::horizontal {
                            child: header
                        }
                        Expand::horizontal {
                            Divider
                        }
                        HStack {
                            spacing: 0.0
                            FixedSize {
                                width: TREE_W
                                height: BODY_H
                                child: left
                            }
                            FixedSize {
                                height: BODY_H
                                Divider::vertical()
                            }
                            Expand::horizontal {
                                FixedSize {
                                    height: BODY_H
                                    child: right
                                }
                            }
                        }
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Delegate to the fixed-size root so the modal host sizes/centres the
        // card (delegating to the greedy inner Panel would fill the window).
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

impl SettingsPanel {
    /// The action bar: Reset to defaults · (spacer) · Done.
    ///
    /// Instant-apply — every setting already took effect and persisted as it was
    /// changed, so there is no Apply/Cancel/OK. *Reset to defaults* live-applies
    /// factory values (enabled only while something differs from them) behind a
    /// confirmation, since there is no undo; *Done* closes the window.
    fn footer(
        &self,
        vm: SettingsViewModel,
        scale: Signal<f32>,
        not_defaults: Signal<bool>,
    ) -> impl Widget {
        // Reset to defaults — confirm first (no undo), then live-apply the
        // factory values. Disabled while already at defaults.
        let reset_vm = vm.clone();
        let reset_scale = scale.clone();
        let reset = Button::new(tr!(settings_reset()))
            .variant(ButtonVariant::Plain)
            .enabled(not_defaults)
            .on_activate_fn(move |ctx| {
                let reset_vm = reset_vm.clone();
                let reset_scale = reset_scale.clone();
                MessageBox::warning(tr!(settings_reset_confirm_title()))
                    .text(tr!(settings_reset_confirm_body()))
                    .buttons(MessageBoxButtons::Custom(vec![
                        MessageBoxButton::standard(StandardButton::RestoreDefaults),
                        MessageBoxButton::standard(StandardButton::Cancel),
                    ]))
                    .default_button(StandardButton::Cancel)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |res, ctx| {
                        if res.button == StandardButton::RestoreDefaults {
                            // Light within the run's own design language:
                            // the style is a launch decision, not a setting,
                            // so resetting the settings must not leave it.
                            ctx.set_theme(crate::style::light());
                            // The factory language is "whatever a fresh install
                            // on this machine would have picked", not a flat
                            // en-US — resetting a French account to English
                            // would be restoring somebody else's default.
                            ctx.set_locale(crate::startup::os_default_locale());
                            reset_scale.set(TEXT_SCALE_DEFAULT);
                            reset_vm.reset_editor_defaults();
                        }
                    })
                    .present(ctx);
            });

        // Done — everything already applied + persisted; just close the window.
        let done = Button::new(tr!(settings_done()))
            .variant(ButtonVariant::Filled)
            .on_activate_fn(|ctx| ctx.dismiss_modal());

        Padding::symmetric(10.0, 22.0).child(
            HStack::new()
                .spacing(9.0)
                .child(reset)
                .child(Spacer::new())
                .child(done),
        )
    }
}

#[cfg(test)]
mod tests;
