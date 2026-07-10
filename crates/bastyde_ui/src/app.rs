//! The application body: a `DockingLayout` whose leading dock is the binder
//! tree and whose center is a `TabWidget` of editor tabs (Phase 3), with a thin
//! status bar underneath. The window chrome (custom `TitleBar` + hamburger menu)
//! lives at the window root in `main.rs`.
//!
//! Clicking a binder item opens (or focuses) its editor tab via the tree's
//! `KeyedSelectionModel<NodeId>` selection signal.
//!
//! Plain builder calls rather than `bati!`: the docking/tab/editor widgets are
//! generic over closures, which the DSL doesn't express cleanly. See
//! `settings_panel.rs` for the `bati!` style.

use std::rc::Rc;

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::settings::SettingsExt;
use bastyde::tokens::SurfaceRole::Hover;
use bastyde::widgets::{
    Divider, DockRail, DockSide, DockingLayout, EventContextMessageBoxExt, Expand, HStack,
    IconButtonSize, MessageBox, MessageBoxButtons, NotificationArchiveModel,
    NotificationCenterButton, Spacer, StandardButton, StatusBar, TabBarVisibility, TabWidget,
    Toast, VStack,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::event::{
    DirectAccessEntity, EntityEvent, Event, LongOperationEvent, Origin, WorkManagementEvent,
};
use frontend::work_management::LoadWorkDto;

use crate::app_ids::AppIds;
use crate::import_plume_panel::ImportPlumePanel;
use crate::intents::AppIntent;
use crate::new_work_panel::NewWorkPanel;
use crate::settings_panel::SettingsPanel;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::tabs::{ContentTab, tab_pane};
use crate::view_models::{
    EditorsViewModel, ImportPlumeViewModel, OutlineViewModel, SettingsViewModel,
};
use crate::welcome_panel::WelcomePanel;

/// A close gesture deferred until the in-flight save finishes. The close guard
/// (and the `work.close` action) sets this, `App` kicks the save, and the
/// SaveWork-completion event performs the action — so the async save is awaited.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PendingExit {
    #[default]
    None,
    /// Close the window (Quit / title-bar X / Alt+F4) once saved.
    CloseWindow,
    /// Close the open work once saved.
    CloseWork,
}

pub struct App {
    app_ctx: Rc<AppContext>,
    /// The outline view-model is created in `main` (the title-bar menu needs a
    /// handle to it for the reactive checkmark) and shared with `App`.
    outline: OutlineViewModel,
    /// Plain mirror of the persisted autosave setting, read by the title-bar menu
    /// (outside `App`) to hide the manual "Save" item. `App::build` mirrors the
    /// store-backed setting into it.
    autosave_menu: Signal<bool>,
    /// `true` while the open work has edits not yet written to disk. Maintained by
    /// `App` (set on mutations, cleared on SaveWork/LoadWork/CloseWork); read by
    /// the close guard + `work.close` to decide whether to prompt.
    unsaved: Signal<bool>,
    /// A deferred close (set by the guard/menu, performed on SaveWork). Shared with
    /// `main`'s window close guard.
    pending_exit: Signal<PendingExit>,
    /// A `.skrib` path given as the launch argument — opened once on first build
    /// (after the `LoadWork` subscription is live so the full load flow runs).
    initial_project: Option<String>,
    /// One-shot guard so the launch project loads only on the first build.
    initial_loaded: bool,
    /// Created once on first build (its column-width signal needs `ctx.settings()`).
    editors: Option<EditorsViewModel>,
    root_child: Option<WidgetId>,
}

impl App {
    pub fn new(
        app_ctx: Rc<AppContext>,
        outline: OutlineViewModel,
        autosave_menu: Signal<bool>,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        initial_project: Option<String>,
    ) -> Self {
        Self {
            app_ctx,
            outline,
            autosave_menu,
            unsaved,
            pending_exit,
            initial_project,
            initial_loaded: false,
            editors: None,
            root_child: None,
        }
    }
}

/// The backend mutation events that mark the work "unsaved" (and reschedule the
/// autosave debounce). Editor *typing* is caught separately via the editors'
/// `edited` signal; `Content` events (which fire only on flush) are excluded so a
/// save's own flush doesn't loop the debounce.
fn mutation_origins() -> Vec<Origin> {
    use DirectAccessEntity::{Binder, BinderItem, BinderTag, DictWord, Work};
    let mut v = Vec::new();
    for ent in [
        Work(EntityEvent::Updated),
        BinderItem(EntityEvent::Created),
        BinderItem(EntityEvent::Updated),
        BinderItem(EntityEvent::Removed),
        Binder(EntityEvent::Created),
        Binder(EntityEvent::Updated),
        Binder(EntityEvent::Removed),
        BinderTag(EntityEvent::Created),
        BinderTag(EntityEvent::Updated),
        BinderTag(EntityEvent::Removed),
        DictWord(EntityEvent::Created),
        DictWord(EntityEvent::Updated),
        DictWord(EntityEvent::Removed),
    ] {
        v.push(Origin::DirectAccess(ent));
    }
    v
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

impl Widget for App {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // ── Layer-B view-models: created once, then shared by clone ──────────
        let settings = SettingsViewModel::new(ctx.settings());

        let app_ctx = self.app_ctx.clone();
        let column_width = settings.column_width();
        let show_synopsis = settings.synopsis_pane();
        let typography = settings.editor_typography();
        let ids = self.outline.ids();
        let editors = self
            .editors
            .get_or_insert_with(|| {
                EditorsViewModel::new(app_ctx, column_width, show_synopsis, typography, ids)
            })
            .clone();

        let outline = self.outline.clone();

        // Persist live theme / interface-language changes into the keys the
        // startup restore reads. The Settings window drives these via the
        // framework's `ThemeSwitcher` / `LanguageSwitcher`, which apply the change
        // live (`EventContext::set_theme` / `set_locale`) but do not themselves
        // persist; these effects mirror the live value into `DARK_KEY` /
        // `LOCALE_KEY` so it restores next launch (see `main::read_prefs`).
        {
            let dark = settings.dark();
            let theme_sig = ctx.theme_signal().clone();
            ctx.effect(&theme_sig, move |t| {
                let is_dark = t.is_dark();
                if dark.get() != is_dark {
                    dark.set(is_dark);
                }
            });
        }
        if let Some(locale_sig) = bastyde::i18n::current_locale() {
            let persisted = settings.locale();
            ctx.effect(&locale_sig, move |l| {
                let tag = l.to_string();
                if persisted.get() != tag {
                    persisted.set(tag);
                }
            });
        }

        // ── Layer-A singles: id-only global state + reactive entity handles ──
        // Created in `main`, shared via `app_state`. `wire` installs each single's
        // event subscriptions on this (process-lifetime) widget; they are
        // re-pointed on `LoadWork` below.
        let ids = ctx
            .app_state::<AppIds>()
            .cloned()
            .expect("AppIds registered in main");
        let single_work = ctx
            .app_state::<SingleWork>()
            .cloned()
            .expect("SingleWork registered in main");
        let single_work_info = ctx
            .app_state::<SingleWorkInfo>()
            .cloned()
            .expect("SingleWorkInfo registered in main");
        single_work.wire(ctx);
        single_work_info.wire(ctx);
        // Keep the outline tree reactive to *all* structural mutations (incl. the
        // Full Chapter view's rename/merge/split/add), not just the outline's own.
        self.outline.wire(ctx);

        // ── App-global commands (the scriptable surface) ─────────────────────
        // Registered with `register_action_global` so they're reachable as a
        // dispatch fallback regardless of where the intent originates — the
        // title-bar menu (which renders in an overlay, NOT under `App`), a global
        // shortcut anchored at the root, or any content handler. A plain
        // `register_action` would only fire on `App`'s own source→root path,
        // which the chrome-fired menu never touches.
        // F9, not Ctrl+B: Ctrl+B is the editor's built-in bold command, and a
        // Global shortcut is resolved *before* the focused widget sees the raw
        // key — so a Ctrl+B binding here would shadow `RichTextEditor`'s bold.
        ctx.register_shortcut_global(
            Shortcut::new("outline.toggle")
                .name("Toggle Outline")
                .primary(KeyStroke::new(Key::F9, Modifiers::NONE))
                .build(),
        );
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("outline.toggle").on_invoke(move |_i, _c| outline.toggle()),
            );
        }
        {
            let editors = editors.clone();
            ctx.register_action_global(Action::new("editor.open_item").on_invoke(move |i, _c| {
                if let Some(AppIntent::OpenItem { item_id, title }) = AppIntent::from_intent(i) {
                    editors.open_or_focus(*item_id, title);
                }
            }));
        }
        // Ctrl+S: flush every editor to the store, then save the project to disk.
        ctx.register_shortcut_global(
            Shortcut::new("editor.save")
                .name("Save")
                .primary(KeyStroke::ctrl(Key::S))
                .build(),
        );
        {
            let editors = editors.clone();
            ctx.register_action_global(
                Action::new("editor.save").on_invoke(move |_i, _c| editors.save_to_disk()),
            );
        }
        // ── File / app commands (the scriptable surface for the title bar). ──
        // Global (not `register_action`/`register_shortcut`) so they're reached
        // from the title-bar overlay menu — which renders as a sibling of `App`,
        // NOT on `App`'s source→root path — as well as from their shortcuts.
        // New Work (Ctrl+N): present the New Work modal (name/format/location/
        // language/template), which creates the work on confirm.
        ctx.register_shortcut_global(
            Shortcut::new("work.new")
                .name("New Work")
                .primary(KeyStroke::ctrl(Key::N))
                .build(),
        );
        {
            let app_ctx = self.app_ctx.clone();
            ctx.register_action_global(Action::new("work.new").on_invoke(move |_i, c| {
                let app_ctx = app_ctx.clone();
                c.present_modal(
                    ModalRequest::deferred(move |t| t.add(NewWorkPanel::new(app_ctx)))
                        .presentation(ModalPresentation::InTree)
                        .title("New Work")
                        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                        .size(600, 680),
                );
            }));
        }
        // Open Work (Ctrl+O): native picker for an existing `.skrib`, then load.
        ctx.register_shortcut_global(
            Shortcut::new("work.open")
                .name("Open Work")
                .primary(KeyStroke::ctrl(Key::O))
                .build(),
        );
        {
            let app_ctx = self.app_ctx.clone();
            ctx.register_action_global(
                Action::new("work.open").on_invoke(move |_i, c| open_work_flow(app_ctx.clone(), c)),
            );
        }
        // Import from Plume Creator: present the Import Plume modal (menu-only, no
        // shortcut). Global so the title-bar overlay menu reaches it — like work.new.
        // The panel is built over the shared, app-state `ImportPlumeViewModel` (the
        // same instance the long-operation events are routed to below), reset first
        // so a previous session's paths don't linger.
        ctx.register_action_global(Action::new("work.import_plume").on_invoke(move |_i, c| {
            let Some(vm) = c.app_state::<ImportPlumeViewModel>().cloned() else {
                return;
            };
            vm.reset_form();
            c.present_modal(
                ModalRequest::deferred(move |t| t.add(ImportPlumePanel::new(vm)))
                    .presentation(ModalPresentation::InTree)
                    .title("Import Plume Creator project")
                    .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                    .size(600, 500),
            );
        }));
        // Close Work (Ctrl+W): the `work.close` *action* is registered further
        // down (it shares the unsaved-changes guard with the window close); here
        // we only add its global shortcut.
        ctx.register_shortcut_global(
            Shortcut::new("work.close")
                .name("Close Work")
                .primary(KeyStroke::ctrl(Key::W))
                .build(),
        );
        // Settings (Ctrl+,): present the settings modal.
        ctx.register_shortcut_global(
            Shortcut::new("app.settings")
                .name("Settings")
                .primary(KeyStroke::ctrl(Key::Character(',')))
                .build(),
        );
        ctx.register_action_global(Action::new("app.settings").on_invoke(|_i, c| {
            c.present_modal(
                ModalRequest::deferred(|t| t.add(SettingsPanel::new()))
                    .presentation(ModalPresentation::InTree)
                    .title("Settings")
                    .size(920, 620)
                    // Not easily dismissable — like a critical MessageBox. Only
                    // the panel's own close button / Cancel / OK close it (each
                    // calls `ctx.dismiss_modal()`); Escape and outside clicks do
                    // not, so a stray click never discards a settings session.
                    .close_behavior(ModalCloseBehavior::Manual),
            );
        }));
        // Quit (Ctrl+Q): routes through the window close guard (unsaved prompt).
        ctx.register_shortcut_global(
            Shortcut::new("app.quit")
                .name("Quit")
                .primary(KeyStroke::ctrl(Key::Q))
                .build(),
        );
        ctx.register_action_global(Action::new("app.quit").on_invoke(|_i, c| c.close_window()));
        // Welcome modal: presented at startup (gated below), and on demand from
        // File ▸ Welcome… and the brand icon button — all dispatch `welcome.show`.
        // Global so the title-bar overlay menu/button reach it (house rule).
        {
            let app_ctx = self.app_ctx.clone();
            ctx.register_action_global(Action::new("welcome.show").on_invoke(move |_i, c| {
                let app_ctx = app_ctx.clone();
                c.present_modal(
                    ModalRequest::deferred(move |t| t.add(WelcomePanel::new(app_ctx)))
                        .presentation(ModalPresentation::InTree)
                        .title("Welcome to Skribisto")
                        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                        .size(780, 548),
                );
            }));
        }

        // ── Binder-tree commands (the scriptable surface for the outline). ───
        // Each drives an `OutlineViewModel` method; the context menu and key
        // handlers below also call these methods directly.
        {
            let outline = outline.clone();
            ctx.register_action_global(Action::new("binder.new_item").on_invoke(move |i, _c| {
                if let Some(AppIntent::NewItem { role, sub_role }) = AppIntent::from_intent(i) {
                    outline.new_item(role.clone(), sub_role.clone());
                }
            }));
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.rename").on_invoke(move |_i, c| outline.rename_selected(c)),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.duplicate")
                    .on_invoke(move |_i, _c| outline.duplicate_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.trash_selected")
                    .on_invoke(move |_i, _c| outline.trash_selected()),
            );
        }
        {
            // Trash one specific binder (id in the intent payload) — fired from
            // the switcher popover's context menu after its confirmation.
            let outline = outline.clone();
            ctx.register_action_global(Action::new("binder.trash").on_invoke(move |i, _c| {
                if let Some(AppIntent::TrashBinder { binder_id }) = AppIntent::from_intent(i) {
                    outline.trash_binder(*binder_id as u64);
                }
            }));
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.indent").on_invoke(move |_i, _c| outline.indent_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.outdent").on_invoke(move |_i, _c| outline.outdent_selected()),
            );
        }
        ctx.register_shortcut_global(
            Shortcut::new("binder.duplicate")
                .name("Duplicate")
                .primary(KeyStroke::ctrl(Key::D))
                .build(),
        );

        // On project load: seed the id-only global state from the freshly-loaded
        // project, open the per-Work undo stack, re-point the singles, rebuild the
        // tree and drop now-stale editor tabs.
        {
            let ids = ids.clone();
            let app_ctx = self.app_ctx.clone();
            let outline = outline.clone();
            let editors = editors.clone();
            let single_work = single_work.clone();
            let single_work_info = single_work_info.clone();
            let unsaved = self.unsaved.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| {
                    ids.seed(&app_ctx);
                    ids.open_stack(&app_ctx);
                    // Start the freshly-loaded Work unfiltered: a stale binder
                    // filter or query from the previous Work would empty the tree.
                    outline.set_binder_filter(None);
                    outline.clear_search();
                    outline.reload();
                    editors.close_all();
                    single_work.set_id(ids.work_id.get());
                    single_work_info.set_id(ids.work_info_id.get());
                    unsaved.set(false);
                    // Advertise this project as open so other instances' switchers
                    // list it (and can raise this window).
                    if let Some(path) = single_work_info.file_name().get() {
                        crate::open_registry::claim(&path, &single_work.title().get());
                    }
                },
            );
        }

        // Route the Plume-import long operation's events to the shared
        // `ImportPlumeViewModel`, which drives its progress / cancel / success /
        // error toast. `subscribe_event_with_ctx` (not `subscribe_event`) because
        // each callback needs a fresh `EventContext` to show/replace the toast —
        // a plain subscription callback gets none. The VM filters by operation id,
        // so events from other long operations (save / backup) are ignored.
        if let Some(import_vm) = ctx.app_state::<ImportPlumeViewModel>().cloned() {
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Progress),
                    move |e: &Event, c| vm.on_long_op_progress(c, e),
                );
            }
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Completed),
                    move |e: &Event, c| vm.on_long_op_completed(c, e),
                );
            }
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Cancelled),
                    move |e: &Event, c| vm.on_long_op_cancelled(c, e),
                );
            }
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Failed),
                    move |e: &Event, c| vm.on_long_op_failed(c, e),
                );
            }
        }

        // On new work: same seeding as load (a project is now open), then write
        // the freshly-created project to the chosen path immediately — a
        // create-and-save. `save_to_disk` resolves the target + shape from the
        // `WorkInfo` the use case just set (from the picker path + is_folder).
        // The new project isn't on disk yet, so it starts `unsaved = true`; the
        // async save is a long op, and the SaveWork-completion handler clears
        // `unsaved` only once the write actually lands — so an exit/close during
        // the in-flight write is caught by the guards instead of dropping the file.
        {
            let ids = ids.clone();
            let app_ctx = self.app_ctx.clone();
            let outline = outline.clone();
            let editors = editors.clone();
            let single_work = single_work.clone();
            let single_work_info = single_work_info.clone();
            let unsaved = self.unsaved.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |_event: &Event| {
                    ids.seed(&app_ctx);
                    ids.open_stack(&app_ctx);
                    outline.set_binder_filter(None);
                    outline.clear_search();
                    outline.reload();
                    editors.close_all();
                    single_work.set_id(ids.work_id.get());
                    single_work_info.set_id(ids.work_info_id.get());
                    unsaved.set(true);
                    if let Some(path) = single_work_info.file_name().get() {
                        crate::open_registry::claim(&path, &single_work.title().get());
                    }
                    editors.save_to_disk();
                },
            );
        }

        // On work close: forget the ids, empty the tree, drop the tabs, and clear
        // the singles (the store no longer holds the work).
        {
            let ids = ids.clone();
            let outline = outline.clone();
            let editors = editors.clone();
            let single_work = single_work.clone();
            let single_work_info = single_work_info.clone();
            let unsaved = self.unsaved.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::CloseWork),
                move |_event: &Event| {
                    ids.clear();
                    outline.set_binder_filter(None);
                    outline.clear_search();
                    outline.reload();
                    editors.close_all();
                    single_work.set_id(None);
                    single_work_info.set_id(None);
                    unsaved.set(false);
                    crate::open_registry::release();
                },
            );
        }

        // App mediates the two peer view-models: *activating* a binder item
        // (click or Enter — NOT arrow navigation, which only moves the selection)
        // opens (or focuses) its editor tab. The tree fires this via
        // `TreeView::on_activate`; App supplies the open callback so neither
        // view-model imports the other.
        let on_open: crate::docks::outline::OpenItemFn = {
            let editors = editors.clone();
            Rc::new(move |item_id, title| editors.open_or_focus(item_id, &title))
        };

        // Keep the "open document" id in sync with the active tab (open, close,
        // or a tab-bar click), so the binder's open-item marker tracks it.
        // Switching tabs also flushes pending edits to the store — autosave on a
        // natural boundary (changed fields only; clean tabs are a no-op).
        {
            let editors = editors.clone();
            ctx.effect(&editors.selected_tab(), move |_| {
                editors.flush_all();
                editors.sync_active_item();
            });
        }

        // ── Autosave ─────────────────────────────────────────────────────────
        // Mirror the persisted setting into the menu's plain signal (the title-bar
        // menu lives outside `App` and can't read `ctx.settings()`).
        {
            self.autosave_menu.set(settings.autosave().get());
            let menu = self.autosave_menu.clone();
            ctx.effect(&settings.autosave(), move |a| menu.set(*a));
        }
        // Dirty tracking + debounced autosave-to-disk. Every mutation (editor
        // typing via the editors' `edited` signal, plus tree/metadata events)
        // marks the work `unsaved` and — when autosave is on — (re)schedules a
        // one-shot wake ~1.5 s out. `wake_at` keeps the loop asleep until the
        // deadline (no 60 fps drain); the `frame_tick` effect only runs on the
        // frames that actually pump, and fires the save when the deadline passes.
        {
            use std::time::{Duration, Instant};
            let deadline: Rc<std::cell::Cell<Option<Instant>>> =
                Rc::new(std::cell::Cell::new(None));
            let wake = ctx.wake_at_handle();
            let autosave = settings.autosave();

            let on_mutation = {
                let deadline = deadline.clone();
                let wake = wake.clone();
                let autosave = autosave.clone();
                let unsaved = self.unsaved.clone();
                Rc::new(move || {
                    unsaved.set(true);
                    if autosave.get() {
                        let at = Instant::now() + Duration::from_millis(1500);
                        deadline.set(Some(at));
                        wake.set(Some(at));
                    }
                })
            };
            {
                let oc = on_mutation.clone();
                ctx.effect(&editors.edited_signal(), move |_| oc());
            }
            for origin in mutation_origins() {
                let oc = on_mutation.clone();
                ctx.subscribe_event(origin, move |_e: &Event| oc());
            }
            {
                let editors = editors.clone();
                let deadline = deadline.clone();
                let autosave = autosave.clone();
                let tick = ctx.frame_tick();
                ctx.effect(&tick, move |_| {
                    let Some(at) = deadline.get() else { return };
                    if Instant::now() >= at {
                        deadline.set(None);
                        if autosave.get() {
                            editors.save_to_disk();
                        }
                    } else {
                        wake.set(Some(at));
                    }
                });
            }
        }

        // ── Exit guards (Close Work / Quit / window close) ───────────────────
        // The window close guard (in `main`) and the `work.close` action set
        // `pending_exit`; that kicks a disk save, and the SaveWork-completion event
        // performs the deferred close — so the async save is awaited, never raced.
        {
            let editors = editors.clone();
            ctx.effect(&self.pending_exit, move |pe| {
                if *pe != PendingExit::None {
                    editors.save_to_disk();
                }
            });
        }
        {
            let unsaved = self.unsaved.clone();
            let pending = self.pending_exit.clone();
            let app_ctx2 = self.app_ctx.clone();
            let window = ctx.window().cloned();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::SaveWork),
                move |_e: &Event| {
                    unsaved.set(false);
                    let pe = pending.get();
                    if pe != PendingExit::None {
                        pending.set(PendingExit::None);
                        match pe {
                            PendingExit::CloseWindow => {
                                if let Some(w) = &window {
                                    w.close();
                                }
                            }
                            PendingExit::CloseWork => {
                                let _ = work_management_commands::close_work(&app_ctx2);
                            }
                            PendingExit::None => {}
                        }
                    }
                },
            );
        }
        // `work.close` — the Close Work menu command. Guards unsaved changes just
        // like the window close: clean → close now; autosave → save then close;
        // else prompt.
        {
            let app_ctx2 = self.app_ctx.clone();
            let unsaved = self.unsaved.clone();
            let autosave = settings.autosave();
            let pending = self.pending_exit.clone();
            ctx.register_action_global(Action::new("work.close").on_invoke(move |_i, ctx| {
                if !unsaved.get() {
                    let _ = work_management_commands::close_work(&app_ctx2);
                    return;
                }
                if autosave.get() {
                    pending.set(PendingExit::CloseWork);
                    return;
                }
                let app_ctx3 = app_ctx2.clone();
                let pe = pending.clone();
                ctx.present_message_box(
                    MessageBox::question(tr!(close_work_question()))
                        .text(tr!(unsaved_changes()))
                        .buttons(MessageBoxButtons::SaveDiscardCancel)
                        .default_button(StandardButton::Save)
                        .escape_button(StandardButton::Cancel)
                        .on_result(move |r, _ctx| match r.button {
                            StandardButton::Save => pe.set(PendingExit::CloseWork),
                            StandardButton::Discard => {
                                let _ = work_management_commands::close_work(&app_ctx3);
                            }
                            _ => {}
                        }),
                );
            }));
        }
        let active_item = editors.active_item();

        // ── Center: dynamic editor tabs ──────────────────────────────────────
        // Closing a tab saves it first (`on_close` is a pre-close intercept):
        // never drop unsaved edits.
        let close_editors = editors.clone();
        let center = TabWidget::new(editors.selected_tab())
            .dynamic_tab::<ContentTab>("editor", |_handle, state| tab_pane(state))
            .dynamic_model(editors.tabs())
            .on_close(move |tab_id, _ctx| close_editors.flush_and_close(tab_id))
            .bar_visibility(TabBarVisibility::Always)
            .compact_bar()
            .selected_tab_background(SurfaceRole::Content)
            .hover_tab_background(Hover)
            .tab_dividers()
            .active_indicator(bastyde::widgets::TabIndicatorPosition::InnerEdge);

        // ── Leading dock: the binder tree, fronted by a VS Code-style activity
        //    bar (icon rail). The OutlineViewModel owns the DockingModel; the
        //    dock content itself lives in `docks::outline`. ───────────────────
        let layout = DockingLayout::new(outline.docking())
            .rail(
                DockRail::new(DockSide::Leading)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .center(center)
            .dock(crate::docks::outline::outline_dock(
                outline.clone(),
                self.app_ctx.clone(),
                on_open,
                active_item,
            ));
        outline.open_in_layout();

        // ── Status bar (thin) with the notification bell ─────────────────────
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");
        let status = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new()
                .spacing(8.0)
                .child(Spacer::new())
                .child(NotificationCenterButton::new(archive).size(IconButtonSize::Compact)),
        );

        let root = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(Divider::new())
                .child(Expand::new().child(layout))
                .child(status),
        );
        self.root_child = Some(root);

        // Open the launch project (argv[1]) exactly once — now that the `LoadWork`
        // subscription above is live, so its handler runs the full load flow
        // (seed ids, reload the tree, point the singles).
        if !self.initial_loaded {
            self.initial_loaded = true;
            if let Some(path) = self.initial_project.clone() {
                if let Err(e) = work_management_commands::load_work(
                    &self.app_ctx,
                    &LoadWorkDto {
                        file_name: path.clone(),
                    },
                ) {
                    eprintln!("skribisto: could not open '{path}': {e}");
                }
            } else if SettingsViewModel::new(ctx.settings()).show_welcome().get() {
                // No work on the command line + "show at startup" on → pop the
                // Welcome modal once the tree mounts. `present_modal` needs an
                // `EventContext` (unavailable in `build`); `run_after_mount`
                // supplies one, so we present directly here (no intent hop).
                let app_ctx = self.app_ctx.clone();
                ctx.run_after_mount(move |ectx| {
                    ectx.present_modal(
                        ModalRequest::deferred(move |t| t.add(WelcomePanel::new(app_ctx)))
                            .presentation(ModalPresentation::InTree)
                            .title("Welcome to Skribisto")
                            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                            .size(780, 548),
                    );
                });
            }
        }

        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// Present the native picker for an existing `.skrib` and load it. Backs the
/// global `work.open` command (File ▸ Open Work… and Ctrl+O).
fn open_work_flow(app_ctx: Rc<AppContext>, ctx: &mut EventContext) {
    let req = FileDialogRequest::pick_file()
        .title("Open Skribisto work")
        .add_filter("Skribisto work", &["skrib"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            let file = path.to_string_lossy().into_owned();
            if let Err(e) =
                work_management_commands::load_work(&app_ctx, &LoadWorkDto { file_name: file })
            {
                ectx.show_toast(Toast::error(tr!(could_not_open_work(
                    error = e.to_string()
                ))));
            }
        }
    });
}
