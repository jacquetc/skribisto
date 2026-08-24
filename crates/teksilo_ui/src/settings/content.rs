// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The settings window's content: the left rail (search + category tree) and
//! every page behind the selection `Switcher` it drives.
//!
//! One function, on purpose. It builds all ~30 pages — the Work-scoped panes
//! (structure/punctuation/language/author/tags/templates/text replacements/
//! personal dictionary, each falling back to an empty placeholder when no
//! project is open), the app-state singletons (dictionaries, export styles,
//! paratext presets, distraction-free themes, backup), the built-in typography/
//! behaviour/appearance panes, whatever an extension registered, and the two
//! rounds of parents' own "overview" pages — into one `Vec` and then derives
//! the `Switcher`'s index by *looking the selected pane up in that very vec*
//! (see the comment above `order`/`index` below). Splitting page construction
//! from the index derivation would reopen the exact bug that comment describes
//! fixing: the two silently disagreeing about a page's slot.
//!
//! Split out of `settings.rs` because it is the window's *view* half of
//! `SettingsPanel::build`, over state that half already resolved — the Work
//! view-models, the tree spec, the app-state singletons — leaving `build`
//! itself to just wrap the result in the header/footer chrome.

use std::rc::Rc;

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::{Expand, Padding, Switcher, VStack};

use super::nav::{Branch, Navigator, Pane, Root, Sec, children_of};
use super::{crumb, empty_pane, pane_frame, panes, section_title, tree};
use crate::backup::BackupSettingsViewModel;
use crate::sessions::WorkSession;
use crate::settings::{SettingsViewModel, WorkSettingsViewModel};
use crate::singles::SingleWork;
use crate::writing_session::WritingGamesViewModel;

/// The frame a contributed page is mounted in: the same [`pane_frame`] every
/// built-in page sits in, breadcrumbed under the Extensions section.
///
/// A function of its own rather than four lines inside the loop, because it is
/// the only thing standing between an extension and a page with no scroll bar,
/// no insets and no header — and a promise that shape is what
/// `a_contributed_page_is_framed_like_a_built_in_one` mounts and measures.
///
/// `tabs::Boxed` rather than a second adapter of the same three lines:
/// [`pane_frame`] takes an `impl Widget` and a registered page hands over a
/// `Box<dyn Widget>`, which is the one thing that is not one. Its path is `pub`
/// and depended on downstream, so it does not move.
pub(super) fn extension_pane(
    section: LocalizedString,
    label: LocalizedString,
    body: Box<dyn Widget>,
) -> impl Widget {
    pane_frame(crumb(Some(section), label), crate::tabs::Boxed::new(body))
}

/// Builds the left rail and the content `Switcher`, for [`super::SettingsPanel::build`]
/// to wrap in its header/footer chrome.
///
/// `session` is the OPENING WINDOW's own Tier-2 bundle (never `ctx.app_state`, see
/// `SettingsPanel::session`'s own doc) — every Work-scoped pane below reads it
/// directly rather than a process-wide slot that would resolve to whichever
/// project's session registered first.
#[allow(clippy::too_many_arguments)]
pub(super) fn build(
    ctx: &mut BuildContext,
    session: &WorkSession,
    selected_pane: Signal<Pane>,
    vm: &SettingsViewModel,
    scale: Signal<f32>,
    games: &WritingGamesViewModel,
    spec: Vec<Root>,
    work_vm: &Option<WorkSettingsViewModel>,
    work: &Option<SingleWork>,
    work_title: String,
) -> (VStack, Switcher) {
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
            session.open_docs.clone(),
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
        .app_state::<crate::spellcheck::DictionariesViewModel>()
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
        .app_state::<crate::export::ExportStylesViewModel>()
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
        .app_state::<crate::export::ParatextPresetsViewModel>()
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
        .app_state::<crate::distraction_free::DistractionFreeThemesViewModel>()
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
            let path = session
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
                    session.ids.work_id.get(),
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
    let tags_pane: Box<dyn Widget> = match (Some(session.tags.clone()), &work) {
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
    let templates_pane: Box<dyn Widget> = match (Some(session.note_templates.clone()), &work) {
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
        match (Some(session.text_replacements.clone()), &work) {
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
                    crate::settings::panes::text_replacements::text_replacements_pane(ctx, &rvm),
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
    let dictionary_pane: Box<dyn Widget> = match (Some(session.user_dictionary.clone()), &work) {
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
    let (tree, selection, nodes) = tree::build_tree(ctx, &selected_pane, &spec, work_title.clone());
    // Every way of reaching a page that isn't clicking its own row goes
    // through this: the search suggestions, and the links on each parent's
    // page. It must be built from the tree's own selection model and node
    // map, or a jump would switch the pane while leaving the highlight behind.
    let nav = Navigator {
        selection,
        nodes: Rc::new(nodes),
        selected_pane: selected_pane.clone(),
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
        (Pane::User, Box::new(panes::user::user_pane(vm))),
        (
            Pane::Appearance,
            Box::new(panes::appearance::appearance_pane(vm, scale.clone())),
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
            Box::new(panes::editor_behavior::editor_behavior_pane(ctx, vm)),
        ),
        (Pane::Goals, Box::new(panes::goals::goals_pane(ctx, vm))),
        (
            Pane::Games,
            // The activation comes from THIS panel's own `WorkSession` — the
            // project the opening window shows — never `ctx.app_state`, which
            // is one slot per process and would let the pane start (or stop) a
            // game in whichever project happened to open first.
            Box::new(panes::games::games_pane(ctx, games)),
        ),
        (
            Pane::MarginLane,
            Box::new(panes::margin_lane::margin_lane_pane(ctx)),
        ),
        (
            Pane::Corkboard,
            Box::new(panes::corkboard::corkboard_pane(ctx, vm)),
        ),
        (Pane::Dictionaries, dictionaries_pane),
        (Pane::Autosave, Box::new(panes::autosave::autosave_pane(vm))),
        (Pane::ExportFormats, export_styles_pane),
        (Pane::Paratext, paratext_pane),
        (Pane::Keymap, Box::new(panes::keymap::keymap_pane(ctx))),
        (Pane::WorkStructure, structure_pane),
        (Pane::WorkPunctuation, punctuation_pane),
        (
            Pane::Punctuation,
            Box::new(panes::punctuation::punctuation_pane(ctx, vm)),
        ),
        (Pane::Backup, backup_pane),
        (Pane::WorkBackup, work_backup_pane),
        (Pane::WorkLanguage, language_pane),
        (Pane::WorkDictionary, dictionary_pane),
        (
            Pane::Spellcheck,
            Box::new(panes::spellcheck::spellcheck_pane(vm)),
        ),
        (Pane::WorkTags, tags_pane),
        (Pane::WorkAuthor, author_pane),
        (Pane::WorkTextReplacements, text_replacements_pane),
        (
            Pane::DistractionFree,
            Box::new(panes::distraction_free::distraction_free_pane(
                ctx,
                vm,
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
    //
    // ⚠ **And wrapped in `pane_frame`, exactly like a built-in one.** It was
    // pushed in raw until a contributed page shipped that was cut off at the
    // bottom of the window with no scroll bar to say there was more — because
    // `pane_frame` is where the `ScrollArea`, the 20/24 insets, the breadcrumb
    // and the rule under it all live, and nothing else supplies them. An
    // extension could build its own, and one did; what it cannot do is build one
    // that stays in step with this file, and every extension rebuilding the
    // window's chrome is the shape a seam exists to prevent.
    //
    // There is a second, quieter reason it belongs here rather than there: the
    // content pane is measured with **no width offered** (`settings.rs` places
    // it in a height-only `FixedSize`), and `pane_frame`'s `ScrollArea` is what
    // re-proposes the real viewport width to the content when it is placed. A
    // page without one is measured unbounded and placed unbounded, so wrap-mode
    // text lays out on a single line and runs off the right-hand edge.
    let mut panes = panes;
    for page in crate::settings_ext::registered_pages() {
        let body = crate::settings_ext::build_page(page.id, ctx)
            .unwrap_or_else(|| Box::new(teksilo::widgets::Spacer::new()));
        panes.push((
            Pane::Extension(page.id),
            Box::new(extension_pane(
                section_title(Sec::Extensions, &work_title),
                (page.label)(),
                body,
            )),
        ));
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
    let index = selected_pane.map(move |sel| order.iter().position(|p| p == sel).unwrap_or(0));
    let content = panes
        .into_iter()
        .fold(Switcher::new(index), |sw, (_, body)| sw.child_boxed(body));

    (left, content)
}
