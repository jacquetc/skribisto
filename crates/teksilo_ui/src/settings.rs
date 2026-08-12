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
use teksilo::res;
use teksilo::settings::{SettingsExt, TEXT_SCALE_KEY};
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, FontPicker, FormLayout, HStack, IconButton,
    LanguageSwitcher, MessageBox, MessageBoxButton, MessageBoxButtons, Padding, Panel, RadioButton,
    RadioGroup, Slider, Spacer, StandardButton, Switcher, TextScaleControl, TextWidget,
    ThemeSwitcher, Toggle, VStack,
};

mod defaults;
mod fields;
mod nav;
pub(crate) mod panes;
mod tree;

// Re-bound here, not merely `use`d, because every pane module reaches this file
// with `use super::super::*` — a glob that carries a private import along with
// the rest. Naming them explicitly is what keeps that glob honest: the list
// below is the settings window's whole internal vocabulary.
pub(crate) use defaults::build_not_defaults;
pub(crate) use fields::{
    crumb, empty_pane, field_label, group, hint, index_to_method, method_to_index, pane_frame,
    section_title, slider_field, slider_field_tipped,
};
pub(crate) use nav::{Branch, Navigator, Pane, Root, Sec, children_of, tree_spec};

use crate::sessions::WorkSession;
use crate::view_models::{
    BackupSettingsViewModel, EditorTypography, HighlightScope, SettingsViewModel, TypewriterAnchor,
    WorkSettingsViewModel,
};
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
        let games = crate::view_models::WritingGamesViewModel::new(
            self.session.always_forward.clone(),
            crate::view_models::WritingGameOptions::new(
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
        let structure_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_structure::work_structure_pane(
                ctx,
                vm,
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_structure()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let punctuation_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_punctuation::work_punctuation_pane(
                ctx,
                vm,
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_punctuation()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let language_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_language::work_language_pane(
                ctx,
                vm,
                work_title.clone(),
                self.session.open_docs.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_language()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        let author_pane: Box<dyn Widget> = match &work_vm {
            Some(vm) => Box::new(panes::work_author::work_author_pane(
                ctx,
                vm,
                work_title.clone(),
            )),
            None => Box::new(empty_pane(
                None,
                tr!(settings_page_author()),
                res!("assets/icons/binder/book.svg"),
            )),
        };
        // Spelling ▸ Dictionaries — the management pane (Installed / Get more), wrapped in the
        // shared `pane_frame` like every other pane. Always available (dictionaries are a
        // machine-wide resource, independent of any open project).
        let dictionaries_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::DictionariesViewModel>()
            .cloned()
        {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_spelling())),
                    tr!(settings_page_dictionaries()),
                ),
                crate::settings::panes::dictionaries::dictionaries_pane(ctx, &vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_spelling())),
                tr!(settings_page_dictionaries()),
                Sec::Spelling.icon_svg(),
            )),
        };
        // Compile & Export ▸ Export Formats — the export-style manager (built-in + user styles,
        // duplicate-to-edit, JSON import/export), wrapped in `pane_frame` like every other pane.
        let export_styles_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::ExportStylesViewModel>()
            .cloned()
        {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_compile())),
                    tr!(settings_page_export()),
                ),
                crate::settings::panes::export_styles::export_styles_pane(ctx, &vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_compile())),
                tr!(settings_page_export()),
                Sec::CompileExport.icon_svg(),
            )),
        };

        // Compile & Export ▸ Paratext structures — the front/back-matter catalogue New
        // Work starts a project from. App-level like export styles, and resolved the same
        // way, so one instance backs both this pane and the New Work picker.
        let paratext_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::ParatextPresetsViewModel>()
            .cloned()
        {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_compile())),
                    tr!(settings_page_paratext()),
                ),
                crate::settings::panes::paratext::paratext_pane(ctx, &vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_compile())),
                tr!(settings_page_paratext()),
                Sec::CompileExport.icon_svg(),
            )),
        };

        // Editor ▸ Distraction-free themes — the theme library, same shape and
        // same app_state resolution as Export Formats just above.
        let df_themes_pane: Box<dyn Widget> = match ctx
            .app_state::<crate::view_models::DistractionFreeThemesViewModel>()
            .cloned()
        {
            Some(themes_vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_editor())),
                    tr!(settings_page_distraction_free_themes()),
                ),
                crate::settings::panes::distraction_free_themes::distraction_free_themes_pane(
                    ctx,
                    &themes_vm,
                    vm.distraction_free_theme(),
                ),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_editor())),
                tr!(settings_page_distraction_free_themes()),
                Sec::Editor.icon_svg(),
            )),
        };

        // ── Backup ("Copies de secours") panes ──
        // Wrapped in the shared `pane_frame` (breadcrumb · rule · scrollable,
        // padded body) exactly like every built-in pane, so they match the rest
        // and their tall forms scroll instead of overflowing.
        let backup_vm = ctx.app_state::<BackupSettingsViewModel>().cloned();
        let backup_pane: Box<dyn Widget> = match &backup_vm {
            Some(vm) => Box::new(pane_frame(
                crumb(
                    Some(tr!(settings_sec_backup())),
                    tr!(settings_page_backup()),
                ),
                crate::settings::panes::backup::general_pane(ctx, vm),
            )),
            None => Box::new(empty_pane(
                Some(tr!(settings_sec_backup())),
                tr!(settings_page_backup()),
                Sec::BackupSync.icon_svg(),
            )),
        };
        let work_backup_pane: Box<dyn Widget> = match (&backup_vm, &work) {
            (Some(vm), Some(w)) if w.id().is_some() => {
                let uid = w.unique_id().get();
                let path = self
                    .session
                    .single_work_info
                    .file_name()
                    .get()
                    .unwrap_or_default();
                let title = w.title().get();
                Box::new(pane_frame(
                    crumb(
                        Some(lit!(format!(
                            "{}: {}",
                            tr!(settings_sec_work()).resolve_now(),
                            title
                        ))),
                        tr!(settings_page_work_backup()),
                    ),
                    crate::settings::panes::backup::work_backup_pane(
                        ctx,
                        vm,
                        uid,
                        path,
                        title,
                        // F3 — `ids.work_id` is the authoritative "current
                        // Work" id, never `single_work.id()` (`w` here):
                        // this feeds `BackupsListPanel`/`BackupsListViewModel`
                        // for the identical delete-failure toast routing as
                        // `app.rs`'s `backups.show` handler (F2), so it must
                        // read the same source.
                        self.session.ids.work_id.get(),
                    ),
                ))
            }
            _ => Box::new(empty_pane(
                None,
                tr!(settings_page_work_backup()),
                res!("assets/icons/binder/book.svg"),
            )),
        };

        // Work ▸ Tags — the per-project palette manager, over THIS WINDOW's own
        // `WorkSession::tags` (never `ctx.app_state::<TagsViewModel>()`, which
        // would resolve to whichever Work's session registered it first).
        // Present in the Switcher regardless, an empty placeholder when no
        // project is open (same as the other Work panes).
        let tags_pane: Box<dyn Widget> = match (Some(self.session.tags.clone()), &work) {
            (Some(tvm), Some(w)) if w.id().is_some() => {
                let title = w.title().get();
                Box::new(pane_frame(
                    crumb(
                        Some(lit!(format!(
                            "{}: {}",
                            tr!(settings_sec_work()).resolve_now(),
                            title
                        ))),
                        tr!(settings_page_tags()),
                    ),
                    crate::settings::panes::work_tags::work_tags_pane(ctx, &tvm),
                ))
            }
            _ => Box::new(empty_pane(
                None,
                tr!(settings_page_tags()),
                res!("assets/icons/binder/book.svg"),
            )),
        };

        // Work ▸ Templates — the per-project note-template catalogue, over THIS WINDOW's
        // own `WorkSession::note_templates`, same reasoning as `tags_pane` above.
        let templates_pane: Box<dyn Widget> =
            match (Some(self.session.note_templates.clone()), &work) {
                (Some(nvm), Some(w)) if w.id().is_some() => {
                    let title = w.title().get();
                    Box::new(pane_frame(
                        crumb(
                            Some(lit!(format!(
                                "{}: {}",
                                tr!(settings_sec_work()).resolve_now(),
                                title
                            ))),
                            tr!(settings_page_templates()),
                        ),
                        crate::settings::panes::work_templates::work_templates_pane(ctx, &nvm),
                    ))
                }
                _ => Box::new(empty_pane(
                    None,
                    tr!(settings_page_templates()),
                    res!("assets/icons/binder/book.svg"),
                )),
            };

        // Work ▸ Text replacements — the per-project custom lexicon, over THIS
        // WINDOW's own `WorkSession::text_replacements` (never `ctx.app_state`),
        // same reasoning as `tags_pane`/`dictionary_pane`/`punctuation` above.
        let text_replacements_pane: Box<dyn Widget> =
            match (Some(self.session.text_replacements.clone()), &work) {
                (Some(rvm), Some(w)) if w.id().is_some() => {
                    let title = w.title().get();
                    Box::new(pane_frame(
                        crumb(
                            Some(lit!(format!(
                                "{}: {}",
                                tr!(settings_sec_work()).resolve_now(),
                                title
                            ))),
                            tr!(settings_page_text_replacements()),
                        ),
                        crate::settings::panes::text_replacements::text_replacements_pane(
                            ctx, &rvm,
                        ),
                    ))
                }
                _ => Box::new(empty_pane(
                    None,
                    tr!(settings_page_text_replacements()),
                    res!("assets/icons/binder/book.svg"),
                )),
            };

        // Work ▸ Personal dictionary — the per-project word-list manager, over
        // THIS WINDOW's own `WorkSession::user_dictionary` (never
        // `ctx.app_state::<UserDictionaryViewModel>()`, same reasoning as
        // `tags_pane` above). Present in the Switcher regardless, an empty
        // placeholder when no project is open (same as the other Work panes).
        let dictionary_pane: Box<dyn Widget> =
            match (Some(self.session.user_dictionary.clone()), &work) {
                (Some(vm), Some(w)) if w.id().is_some() => {
                    let title = w.title().get();
                    Box::new(pane_frame(
                        crumb(
                            Some(lit!(format!(
                                "{}: {}",
                                tr!(settings_sec_work()).resolve_now(),
                                title
                            ))),
                            tr!(settings_page_personal_dictionary()),
                        ),
                        crate::settings::panes::user_dictionary::user_dictionary_pane(ctx, &vm),
                    ))
                }
                _ => Box::new(empty_pane(
                    None,
                    tr!(settings_page_personal_dictionary()),
                    res!("assets/icons/binder/book.svg"),
                )),
            };

        // ── Left rail: search + category tree ───────────────────────────────
        let (tree, selection, nodes) =
            tree::build_tree(ctx, &self.selected_pane, &spec, work_title.clone());
        // Every way of reaching a page that isn't clicking its own row goes
        // through this: the search suggestions, and the links on each parent's
        // page. It must be built from the tree's own selection model and node
        // map, or a jump would switch the pane while leaving the highlight behind.
        let nav = Navigator {
            selection,
            nodes: Rc::new(nodes),
            selected_pane: self.selected_pane.clone(),
        };
        let search = tree::search_field(&spec, &work_title, nav.clone());
        let left = VStack::new()
            .spacing(0.0)
            .child(Padding::symmetric(10.0, 10.0).child(search))
            .child(Expand::vertical().child(Padding::symmetric(2.0, 6.0).child(tree)));

        // ── Right pane: the per-page content behind the selection Switcher ──
        // The Switcher index is looked up in this very list (see below), so a page
        // may be added anywhere in it; each child is tagged with the `Pane` it
        // serves rather than trusted from a hand-kept `.child()` chain.
        let typo = vm.editor_typography();
        let panes: Vec<(Pane, Box<dyn Widget>)> = vec![
            (Pane::User, Box::new(panes::user::user_pane(&vm))),
            (
                Pane::Appearance,
                Box::new(panes::appearance::appearance_pane(&vm, scale.clone())),
            ),
            (
                Pane::MenusToolbars,
                Box::new(empty_pane(
                    Some(tr!(settings_sec_appearance_behaviour())),
                    tr!(settings_page_menus()),
                    Sec::AppearanceBehaviour.icon_svg(),
                )),
            ),
            (
                Pane::Notifications,
                panes::notifications::notifications_pane(ctx),
            ),
            (
                Pane::SceneTypography,
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_scene()),
                    &typo.scene,
                )),
            ),
            (
                Pane::SynopsisTypography,
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_synopsis()),
                    &typo.synopsis,
                )),
            ),
            (
                Pane::NotesTypography,
                Box::new(panes::typography::typography_pane(
                    ctx,
                    tr!(settings_page_notes()),
                    &typo.notes,
                )),
            ),
            (
                Pane::EditorBehavior,
                Box::new(panes::editor_behavior::editor_behavior_pane(ctx, &vm)),
            ),
            (Pane::Goals, Box::new(panes::goals::goals_pane(ctx, &vm))),
            (
                Pane::Games,
                // The activation comes from THIS panel's own `WorkSession` — the
                // project the opening window shows — never `ctx.app_state`, which
                // is one slot per process and would let the pane start (or stop) a
                // game in whichever project happened to open first.
                Box::new(panes::games::games_pane(ctx, &games)),
            ),
            (
                Pane::Corkboard,
                Box::new(panes::corkboard::corkboard_pane(ctx, &vm)),
            ),
            (Pane::Dictionaries, dictionaries_pane),
            (
                Pane::Autosave,
                Box::new(panes::autosave::autosave_pane(&vm)),
            ),
            (Pane::ExportFormats, export_styles_pane),
            (Pane::Paratext, paratext_pane),
            (Pane::Keymap, Box::new(panes::keymap::keymap_pane(ctx))),
            (Pane::WorkStructure, structure_pane),
            (Pane::WorkPunctuation, punctuation_pane),
            (
                Pane::Punctuation,
                Box::new(panes::punctuation::punctuation_pane(ctx, &vm)),
            ),
            (Pane::Backup, backup_pane),
            (Pane::WorkBackup, work_backup_pane),
            (Pane::WorkLanguage, language_pane),
            (Pane::WorkDictionary, dictionary_pane),
            (
                Pane::Spellcheck,
                Box::new(panes::spellcheck::spellcheck_pane(&vm)),
            ),
            (Pane::WorkTags, tags_pane),
            (Pane::WorkAuthor, author_pane),
            (Pane::WorkTextReplacements, text_replacements_pane),
            (
                Pane::DistractionFree,
                Box::new(panes::distraction_free::distraction_free_pane(
                    ctx,
                    &vm,
                    &typo.distraction_free,
                )),
            ),
            (Pane::DistractionFreeThemes, df_themes_pane),
            (Pane::WorkTemplates, templates_pane),
        ];
        // Anything an extension registered, appended in the same order
        // `build_tree` inserted its nodes — the `Switcher` index is derived from
        // this vec below, so the two cannot disagree however the list grows.
        //
        // Built here, with the real `BuildContext`, so a contributed page reaches
        // `ctx.settings()` and binds its own keys exactly as a built-in pane does.
        let mut panes = panes;
        for page in crate::settings_ext::registered_pages() {
            let body = crate::settings_ext::build_page(page.id, ctx)
                .unwrap_or_else(|| Box::new(teksilo::widgets::Spacer::new()));
            panes.push((Pane::Extension(page.id), body));
        }
        // Every parent's own page, off the same spec the tree was built from — so
        // a section added there arrives with its page already written, and a page
        // moved between sections is listed by its new parent without a second
        // edit. A parent the spec left out (the Work section with nothing open)
        // has no page here either, and nothing can select it.
        for root in &spec {
            let Root::Section(sec, branches) = root else {
                continue;
            };
            let parent = Pane::Section(*sec);
            panes.push((
                parent,
                Box::new(panes::overview::overview_pane(
                    parent,
                    section_title(*sec, &work_title),
                    None,
                    children_of(&spec, parent),
                    nav.clone(),
                )),
            ));
            for branch in branches {
                let Branch::Group(group, _) = branch else {
                    continue;
                };
                let parent = Pane::Group(*group);
                panes.push((
                    parent,
                    Box::new(panes::overview::overview_pane(
                        parent,
                        group.label(),
                        // One level in, so its trail names the section it sits in.
                        Some(section_title(*sec, &work_title)),
                        children_of(&spec, parent),
                        nav.clone(),
                    )),
                ));
            }
        }
        // The list IS the order. Deriving the `Switcher` index by looking the selected
        // pane up in this very vec is what makes the pairing true rather than merely
        // asserted: previously the index was `pane as usize` and a page added at the
        // wrong slot shifted every later page's content by one — silently, and it
        // shipped that way once. A `debug_assert!` caught it in debug builds only.
        // Nothing to catch now; the two cannot disagree.
        //
        // An unknown pane resolves to slot 0, matching `Switcher`'s own out-of-range
        // behaviour: showing the first page beats showing a blank panel.
        let order: Vec<Pane> = panes.iter().map(|(p, _)| *p).collect();
        let index = self
            .selected_pane
            .map(move |sel| order.iter().position(|p| p == sel).unwrap_or(0));
        let content = panes
            .into_iter()
            .fold(Switcher::new(index), |sw, (_, body)| sw.child_boxed(body));

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
                            ctx.set_theme(intui::light());
                            ctx.set_locale("en-US");
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
