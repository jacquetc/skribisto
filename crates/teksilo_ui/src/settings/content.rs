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

use teksilo::canvas::svg::SvgIcon;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::{Expand, Padding, Switcher, VStack};

use super::nav::{Branch, Navigator, Pane, Root, Sec, children_of};
use super::{Crumbs, empty_pane, pane_frame, panes, section_title, tree};
use crate::backup::BackupSettingsViewModel;
use crate::sessions::WorkSession;
use crate::settings::{SettingsViewModel, WorkSettingsViewModel};
use crate::singles::SingleWork;
use crate::writing_session::WritingGamesViewModel;

/// The placeholder icon every Work-scoped page falls back to with no project
/// open — one book, named once, rather than the same `res!` spelled out at each
/// of the nine pages that shows it.
fn book_icon() -> &'static SvgIcon {
    res!("assets/icons/binder/book.svg")
}

/// The frame a contributed page is mounted in: the same [`pane_frame`] every
/// built-in page sits in, breadcrumbed under the Extensions section.
///
/// A function of its own rather than four lines inside the loop, because it is
/// the only thing standing between an extension and a page with no scroll bar,
/// no insets and no header — and a promise that shape is what
/// `a_contributed_page_is_framed_like_a_built_in_one` mounts and measures.
///
/// The trail is derived like every built-in page's, off the same spec — an
/// extension's page therefore gets a *working* link back to the Extensions
/// section rather than the inert word "Extensions" it used to print. Its own
/// crumb has to be passed in: the label lives in the registry, not the enum.
///
/// `tabs::Boxed` rather than a second adapter of the same three lines:
/// [`pane_frame`] takes an `impl Widget` and a registered page hands over a
/// `Box<dyn Widget>`, which is the one thing that is not one. Its path is `pub`
/// and depended on downstream, so it does not move.
pub(super) fn extension_pane(
    crumbs: &Crumbs,
    id: &'static str,
    label: LocalizedString,
    body: Box<dyn Widget>,
) -> impl Widget {
    pane_frame(
        crumbs.titled(Pane::Extension(id), label),
        crate::tabs::Boxed::new(body),
    )
}

/// Builds the left rail and the content `Switcher`, for [`super::SettingsPanel::build`]
/// to wrap in its header/footer chrome.
///
/// The third element is the rail's own `SearchField`, by id: the panel returns it
/// as the window's initial-focus hint. Without one, focus lands on the first
/// focusable widget in the tree — the header's ✕, the one control in the window
/// nobody opened Settings to reach — and a writer who opened it to change one
/// setting had to take their hands off the keyboard to say which.
///
/// `session` is the OPENING WINDOW's own Tier-2 bundle (never `ctx.app_state`, see
/// `SettingsPanel::session`'s own doc) — every Work-scoped pane below reads it
/// directly rather than a process-wide slot that would resolve to whichever
/// project's session registered first. It is `None` when the window that opened
/// this one holds no project (the Launcher): every Work-scoped pane below then
/// takes the same empty-placeholder branch it already takes for a session whose
/// Work has not loaded, and Writing games hands its unbindable activation switch
/// over disabled rather than live.
#[allow(clippy::too_many_arguments)]
pub(super) fn build(
    ctx: &mut BuildContext,
    session: Option<&WorkSession>,
    selected_pane: Signal<Pane>,
    vm: &SettingsViewModel,
    scale: Signal<f32>,
    games: &WritingGamesViewModel,
    spec: Vec<Root>,
    work_vm: &Option<WorkSettingsViewModel>,
    work: &Option<SingleWork>,
    work_title: String,
) -> (VStack, Switcher, WidgetId) {
    // ── Left rail: search + category tree ───────────────────────────────
    //
    // Built *before* the pages, not after: every page's breadcrumb is derived
    // from this tree's own spec and navigates with this tree's own selection
    // model, so the rail has to exist before the first page can be framed.
    let spec = Rc::new(spec);
    let (tree, reveal, selection, nodes) =
        tree::build_tree(ctx, &selected_pane, &spec, work_title.clone());
    // Every way of reaching a page that isn't clicking its own row goes
    // through this: the search suggestions, the links on each parent's page,
    // and every ancestor crumb above every page. It must be built from the
    // tree's own tree model, selection model and node map, or a jump would
    // switch the pane while leaving the highlight behind — or leave it on a row
    // still folded inside a collapsed section, which is the same disagreement
    // wearing a different hat. See `Navigator::go`.
    let nav = Navigator {
        selection,
        nodes: Rc::new(nodes),
        selected_pane: selected_pane.clone(),
        reveal,
        spec: spec.clone(),
    };
    // Added to the tree here rather than built inline, because the panel needs its
    // id: it is what the window focuses on open.
    let search = tree::search_field(ctx, &spec, &work_title, nav.clone());
    let left = VStack::new()
        .spacing(0.0)
        .child(Padding::symmetric(10.0, 10.0).child_id(search))
        .child(Expand::vertical().child(Padding::symmetric(2.0, 6.0).child(tree)));

    // Every breadcrumb in the window, derived from that same spec rather than
    // written out per page. A page names only *itself*; where it sits, and what
    // its ancestors are called, is the tree's answer to give — which is what
    // makes "Editor › Typography › Scene" right without any page knowing it is
    // two levels down.
    let crumbs = Crumbs::new(spec.clone(), &work_title, Some(nav.clone()));

    let structure_pane: Box<dyn Widget> = match &work_vm {
        Some(vm) => Box::new(panes::work_structure::work_structure_pane(ctx, vm, &crumbs)),
        None => Box::new(empty_pane(&crumbs, Pane::WorkStructure, book_icon())),
    };
    let punctuation_pane: Box<dyn Widget> = match &work_vm {
        Some(vm) => Box::new(panes::work_punctuation::work_punctuation_pane(
            ctx, vm, &crumbs,
        )),
        None => Box::new(empty_pane(&crumbs, Pane::WorkPunctuation, book_icon())),
    };
    let language_pane: Box<dyn Widget> = match (&work_vm, session) {
        (Some(vm), Some(session)) => Box::new(panes::work_language::work_language_pane(
            ctx,
            vm,
            &crumbs,
            session.open_docs.clone(),
        )),
        _ => Box::new(empty_pane(&crumbs, Pane::WorkLanguage, book_icon())),
    };
    let author_pane: Box<dyn Widget> = match &work_vm {
        Some(vm) => Box::new(panes::work_author::work_author_pane(ctx, vm, &crumbs)),
        None => Box::new(empty_pane(&crumbs, Pane::WorkAuthor, book_icon())),
    };
    // Spelling ▸ Dictionaries — the management pane (Installed / Get more), wrapped in the
    // shared `pane_frame` like every other pane. Always available (dictionaries are a
    // machine-wide resource, independent of any open project).
    let dictionaries_pane: Box<dyn Widget> = match ctx
        .app_state::<crate::spellcheck::DictionariesViewModel>()
        .cloned()
    {
        Some(vm) => Box::new(pane_frame(
            crumbs.of(Pane::Dictionaries),
            crate::settings::panes::dictionaries::dictionaries_pane(ctx, &vm),
        )),
        None => Box::new(empty_pane(
            &crumbs,
            Pane::Dictionaries,
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
            crumbs.of(Pane::ExportFormats),
            crate::settings::panes::export_styles::export_styles_pane(ctx, &vm),
        )),
        None => Box::new(empty_pane(
            &crumbs,
            Pane::ExportFormats,
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
            crumbs.of(Pane::Paratext),
            crate::settings::panes::paratext::paratext_pane(ctx, &vm),
        )),
        None => Box::new(empty_pane(
            &crumbs,
            Pane::Paratext,
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
            crumbs.of(Pane::DistractionFreeThemes),
            crate::settings::panes::distraction_free_themes::distraction_free_themes_pane(
                ctx,
                &themes_vm,
                vm.distraction_free_theme(),
            ),
        )),
        None => Box::new(empty_pane(
            &crumbs,
            Pane::DistractionFreeThemes,
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
            crumbs.of(Pane::Backup),
            crate::settings::panes::backup::general_pane(ctx, vm),
        )),
        None => Box::new(empty_pane(
            &crumbs,
            Pane::Backup,
            Sec::BackupSync.icon_svg(),
        )),
    };
    let work_backup_pane: Box<dyn Widget> = match (&backup_vm, &work, session) {
        (Some(vm), Some(w), Some(session)) if w.id().is_some() => {
            let uid = w.unique_id().get();
            let path = session
                .single_work_info
                .file_name()
                .get()
                .unwrap_or_default();
            let title = w.title().get();
            Box::new(pane_frame(
                crumbs.of(Pane::WorkBackup),
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
        _ => Box::new(empty_pane(&crumbs, Pane::WorkBackup, book_icon())),
    };

    // Work ▸ Tags — the per-project palette manager, over THIS WINDOW's own
    // `WorkSession::tags` (never `ctx.app_state::<TagsViewModel>()`, which
    // would resolve to whichever Work's session registered it first).
    // Present in the Switcher regardless, an empty placeholder when no
    // project is open (same as the other Work panes).
    let tags_pane: Box<dyn Widget> = match (session, &work) {
        (Some(session), Some(w)) if w.id().is_some() => {
            let tvm = session.tags.clone();
            Box::new(pane_frame(
                crumbs.of(Pane::WorkTags),
                crate::settings::panes::work_tags::work_tags_pane(
                    ctx,
                    &tvm,
                    &session.note_templates,
                ),
            ))
        }
        _ => Box::new(empty_pane(&crumbs, Pane::WorkTags, book_icon())),
    };

    // Work ▸ Statuses — the workflow ladder editor, over THIS WINDOW's own
    // `WorkSession::statuses`, same reasoning as `tags_pane` above.
    let statuses_pane: Box<dyn Widget> = match (session.map(|s| s.statuses.clone()), &work) {
        (Some(svm), Some(w)) if w.id().is_some() => Box::new(pane_frame(
            crumbs.of(Pane::WorkStatuses),
            crate::settings::panes::work_statuses::work_statuses_pane(ctx, &svm),
        )),
        _ => Box::new(empty_pane(&crumbs, Pane::WorkStatuses, book_icon())),
    };

    // Work ▸ Templates — the per-project note-template catalogue, over THIS WINDOW's
    // own `WorkSession::note_templates`, same reasoning as `tags_pane` above.
    let templates_pane: Box<dyn Widget> = match (session.map(|s| s.note_templates.clone()), &work) {
        (Some(nvm), Some(w)) if w.id().is_some() => Box::new(pane_frame(
            crumbs.of(Pane::WorkTemplates),
            crate::settings::panes::work_templates::work_templates_pane(ctx, &nvm),
        )),
        _ => Box::new(empty_pane(&crumbs, Pane::WorkTemplates, book_icon())),
    };

    // Work ▸ Text replacements — the per-project custom lexicon, over THIS
    // WINDOW's own `WorkSession::text_replacements` (never `ctx.app_state`),
    // same reasoning as `tags_pane`/`dictionary_pane`/`punctuation` above.
    let text_replacements_pane: Box<dyn Widget> =
        match (session.map(|s| s.text_replacements.clone()), &work) {
            (Some(rvm), Some(w)) if w.id().is_some() => Box::new(pane_frame(
                crumbs.of(Pane::WorkTextReplacements),
                crate::settings::panes::text_replacements::text_replacements_pane(ctx, &rvm),
            )),
            _ => Box::new(empty_pane(&crumbs, Pane::WorkTextReplacements, book_icon())),
        };

    // Work ▸ Personal dictionary — the per-project word-list manager, over
    // THIS WINDOW's own `WorkSession::user_dictionary` (never
    // `ctx.app_state::<UserDictionaryViewModel>()`, same reasoning as
    // `tags_pane` above). Present in the Switcher regardless, an empty
    // placeholder when no project is open (same as the other Work panes).
    let dictionary_pane: Box<dyn Widget> = match (session.map(|s| s.user_dictionary.clone()), &work)
    {
        (Some(vm), Some(w)) if w.id().is_some() => Box::new(pane_frame(
            crumbs.of(Pane::WorkDictionary),
            crate::settings::panes::user_dictionary::user_dictionary_pane(ctx, &vm),
        )),
        _ => Box::new(empty_pane(&crumbs, Pane::WorkDictionary, book_icon())),
    };

    // ── Right pane: the per-page content behind the selection Switcher ──
    // The Switcher index is looked up in this very list (see below), so a page
    // may be added anywhere in it; each child is tagged with the `Pane` it
    // serves rather than trusted from a hand-kept `.child()` chain.
    let typo = vm.editor_typography();
    let panes: Vec<(Pane, Box<dyn Widget>)> = vec![
        (Pane::User, Box::new(panes::user::user_pane(&crumbs, vm))),
        (
            Pane::Appearance,
            Box::new(panes::appearance::appearance_pane(
                &crumbs,
                vm,
                scale.clone(),
            )),
        ),
        (
            Pane::Notifications,
            panes::notifications::notifications_pane(ctx, &crumbs),
        ),
        (
            Pane::SceneTypography,
            Box::new(panes::typography::typography_pane(
                ctx,
                &crumbs,
                Pane::SceneTypography,
                &typo.scene,
            )),
        ),
        (
            Pane::SynopsisTypography,
            Box::new(panes::typography::typography_pane(
                ctx,
                &crumbs,
                Pane::SynopsisTypography,
                &typo.synopsis,
            )),
        ),
        (
            Pane::NotesTypography,
            Box::new(panes::typography::typography_pane(
                ctx,
                &crumbs,
                Pane::NotesTypography,
                &typo.notes,
            )),
        ),
        (
            Pane::EditorBehavior,
            Box::new(panes::editor_behavior::editor_behavior_pane(
                ctx, &crumbs, vm,
            )),
        ),
        (
            Pane::Goals,
            Box::new(panes::goals::goals_pane(ctx, &crumbs, vm)),
        ),
        (
            Pane::Games,
            // The activation comes from THIS panel's own `WorkSession` — the
            // project the opening window shows — never `ctx.app_state`, which
            // is one slot per process and would let the pane start (or stop) a
            // game in whichever project happened to open first. With no session
            // at all (the Launcher) there is no manuscript to play in, so the
            // switch is handed over disabled; the two "where it applies" boxes
            // under it are ordinary app settings and stay live.
            Box::new(panes::games::games_pane(
                ctx,
                &crumbs,
                games,
                session.is_some(),
            )),
        ),
        (
            Pane::MarginLane,
            Box::new(panes::margin_lane::margin_lane_pane(ctx, &crumbs)),
        ),
        (
            Pane::Corkboard,
            Box::new(panes::corkboard::corkboard_pane(ctx, &crumbs, vm)),
        ),
        (Pane::Dictionaries, dictionaries_pane),
        (
            Pane::Autosave,
            Box::new(panes::autosave::autosave_pane(&crumbs, vm)),
        ),
        (Pane::ExportFormats, export_styles_pane),
        (Pane::Paratext, paratext_pane),
        (
            Pane::Keymap,
            Box::new(panes::keymap::keymap_pane(ctx, &crumbs)),
        ),
        (Pane::WorkStructure, structure_pane),
        (Pane::WorkPunctuation, punctuation_pane),
        (
            Pane::Punctuation,
            Box::new(panes::punctuation::punctuation_pane(ctx, &crumbs, vm)),
        ),
        (Pane::Backup, backup_pane),
        (Pane::WorkBackup, work_backup_pane),
        (Pane::WorkLanguage, language_pane),
        (Pane::WorkDictionary, dictionary_pane),
        (
            Pane::Spellcheck,
            Box::new(panes::spellcheck::spellcheck_pane(&crumbs, vm)),
        ),
        (Pane::WorkTags, tags_pane),
        (Pane::WorkStatuses, statuses_pane),
        (Pane::WorkAuthor, author_pane),
        (Pane::WorkTextReplacements, text_replacements_pane),
        (
            Pane::DistractionFree,
            Box::new(panes::distraction_free::distraction_free_pane(
                ctx,
                &crumbs,
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
            Box::new(extension_pane(&crumbs, page.id, (page.label)(), body)),
        ));
    }
    // Every parent's own page, off the same spec the tree was built from — so
    // a section added there arrives with its page already written, and a page
    // moved between sections is listed by its new parent without a second
    // edit. A parent the spec left out (the Work section with nothing open)
    // has no page here either, and nothing can select it.
    for root in spec.iter() {
        let Root::Section(sec, branches) = root else {
            continue;
        };
        let parent = Pane::Section(*sec);
        panes.push((
            parent,
            Box::new(panes::overview::overview_pane(
                parent,
                section_title(*sec, &work_title),
                &crumbs,
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
                    &crumbs,
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

    (left, content, search)
}
