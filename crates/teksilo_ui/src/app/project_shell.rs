// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project-window shell assembly: split editor, docks, status bar, banners.
//!
//! Extracted from `App::build` so that function is wiring + composition, not
//! 500 lines of widget tree.

use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{
    Divider, DockAction, DockActionId, DockActionPlacement, DockCorner, DockRail, DockRailItemSize,
    DockSide, DockingLayout, DropRegion, DropTarget, DropTargetVariant, Expand, HStack, IconButton,
    IconButtonSize, NotificationArchiveModel, RowDragData, Spacer, Splitter, StatusBar,
    TabBarVisibility, TextWidget, VStack,
};

/// The leading rail's Settings cog — a dockless [`DockAction`], not an activity:
/// it opens the Settings *window*, so there is no dock for it to front.
///
/// Pinned past the rail's spacer (the VS Code Manage-gear position). It fires
/// the existing `app.settings` global action rather than building the modal
/// itself, so the rail button, the Work ▸ Settings menu item and Ctrl+, can
/// never drift apart.
const SETTINGS_ACTION: DockActionId = DockActionId::named("skribisto.settings");

use crate::models::TreeNode;
use crate::tabs::shared::editor::VisibleWhen;
use crate::view_models::{
    EditorsViewModel, OutlineViewModel, RestoreRequest, SearchReplaceViewModel, SettingsViewModel,
    Side,
};

use super::{App, build_pane_tabs, drain_dropped};

/// Locals the shell tree needs from `App::build` (everything not already on `App`).
pub(super) struct ShellParts {
    pub editors: EditorsViewModel,
    pub outline: OutlineViewModel,
    pub search: SearchReplaceViewModel,
    pub trash: crate::view_models::TrashViewModel,
    pub comments: crate::view_models::CommentsViewModel,
    pub footnotes: crate::view_models::FootnotesViewModel,
    pub versions: crate::view_models::VersionsViewModel,
    pub timeline: crate::view_models::TimelineViewModel,
    pub format: crate::view_models::FormatViewModel,
    pub settings: SettingsViewModel,
    pub session: crate::sessions::WorkSession,
    pub ids: crate::app_ids::AppIds,
    pub on_open: crate::docks::outline::OpenItemFn,
    pub single_work: crate::singles::SingleWork,
    pub single_work_info: crate::singles::SingleWorkInfo,
    pub restore_vm: crate::view_models::BackupRestoreViewModel,
    pub save_as_vm: crate::view_models::SaveAsViewModel,
}

impl App {
    /// Build the docking layout + status bar + banners; return the root widget id.
    pub(super) fn build_shell(&mut self, ctx: &mut BuildContext, parts: ShellParts) -> WidgetId {
        let ShellParts {
            editors,
            outline,
            search,
            trash,
            comments,
            footnotes,
            versions,
            timeline,
            format,
            settings,
            session,
            ids,
            on_open,
            single_work,
            single_work_info,
            restore_vm,
            save_as_vm,
        } = parts;

        let active_item = editors.active_item();
        let split_active = editors.split_active();

        // Tell the Versions dock where this project's past is kept, and how to turn
        // the focused tab's store id into the durable uid a timeline keys on.
        //
        // Both are resolved here rather than inside the dock so it imports neither
        // `EditorsViewModel` nor the store — the discipline the outline and trash
        // docks already follow with `OpenItemFn`.
        //
        // Pushed from an **effect**, not once from this build. `WorkInfo.file_name`
        // is filled in by a `WorkInfo::Updated` event that lands after the shell is
        // built, so a value read here is empty — and this shell does not rebuild
        // when it arrives. Read once, the dock would spend the whole session
        // believing the project had never been saved and so could have no past.
        {
            let backup_settings = ctx
                .app_state::<crate::view_models::BackupSettingsViewModel>()
                .cloned();
            let file_name = single_work_info.file_name();
            let unique_id = single_work.unique_id();
            // Bumped whenever something records a new version — see
            // `ProjectHandle::revision`. Owned here rather than derived from the
            // two source counters because *what* changed does not matter, only
            // that something did; one monotone number cannot collide the way
            // adding two independent epochs together can.
            let revision = std::rc::Rc::new(std::cell::Cell::new(0u64));
            let push = {
                let versions = versions.clone();
                let timeline = timeline.clone();
                let file_name = file_name.clone();
                let unique_id = unique_id.clone();
                let revision = revision.clone();
                std::rc::Rc::new(move || {
                    let uid = unique_id.get();
                    // The project's *effective* policy, so a per-project override of
                    // the destinations is honoured — the timeline must look where
                    // the backups actually go, not where the general policy says.
                    let configured = backup_settings
                        .as_ref()
                        .map(|vm| vm.effective_for(&uid).destinations)
                        .unwrap_or_default();
                    let handle = crate::view_models::versions::ProjectHandle {
                        path: file_name.get().unwrap_or_default(),
                        unique_id: uid,
                        // `search_destinations`, not `effective_destinations`:
                        // reading history is a wider question than writing it.
                        // Backups made before the default moved sit beside the
                        // project, and a reader that only looked where the *next*
                        // one will go showed every existing user an empty pane.
                        destinations: crate::backup_paths::search_destinations(&configured),
                        revision: revision.get(),
                    };
                    versions.set_project(handle.clone());
                    timeline.set_project(handle);
                })
            };
            push();
            let on_name = push.clone();
            ctx.effect(&file_name, move |_| on_name());
            let on_uid = push.clone();
            ctx.effect(&unique_id, move |_| on_uid());

            // A backup and a save each write a version. Neither changes where
            // the project lives, so without this the panes' caches — which are
            // keyed on the handle — never came back for the new one.
            //
            // Bumping *then* pushing is what makes the handle differ by value,
            // which is the only thing `set_project`'s idempotence looks at. The
            // effects also fire once at registration; that costs a single extra
            // scan at startup and keeps the wiring free of a "have I run yet"
            // flag that would have to be right in every window.
            let bump = {
                let revision = revision.clone();
                let push = push.clone();
                std::rc::Rc::new(move || {
                    revision.set(revision.get().wrapping_add(1));
                    push();
                })
            };
            let backup_epoch = session.backup_scheduler.completed_epoch_signal();
            let on_backup = bump.clone();
            ctx.effect(&backup_epoch, move |_| on_backup());
            let saved_seq = session.save_state.saved_seq();
            let on_save = bump.clone();
            ctx.effect(&saved_seq, move |_| on_save());

            // Editing the backup policy moves the *destinations* — where these
            // panes look for the project's past. Not a new version, so `push` and
            // not `bump`: nothing was recorded, the map changed. Without this a
            // writer who added a destination in Settings saw the two surfaces keep
            // reading the old list until a save or a backup happened to fire.
            if let Some(policy_revision) = ctx
                .app_state::<crate::view_models::BackupSettingsViewModel>()
                .map(|vm| vm.policy_revision())
            {
                let on_policy = push.clone();
                ctx.effect(&policy_revision, move |_| on_policy());
            }
        }
        // Which backups this project keeps whatever retention decides. Read and
        // written here for the same reason the handle above is pushed from here:
        // pins live in the app's backup settings, and a view-model does not
        // import a peer.
        {
            let settings = ctx
                .app_state::<crate::view_models::BackupSettingsViewModel>()
                .cloned();
            let unique_id = single_work.unique_id();
            let file_name = single_work_info.file_name();
            let read = {
                let (settings, unique_id) = (settings.clone(), unique_id.clone());
                std::rc::Rc::new(move |path: &std::path::Path| {
                    settings.as_ref().is_some_and(|s| {
                        s.service()
                            .is_pinned(&unique_id.get(), &path.to_string_lossy())
                    })
                })
            };
            let write = std::rc::Rc::new(move |path: &std::path::Path, pinned: bool| {
                let Some(settings) = settings.as_ref() else {
                    return;
                };
                if let Err(e) = settings.service().set_pinned(
                    &unique_id.get(),
                    &file_name.get().unwrap_or_default(),
                    &path.to_string_lossy(),
                    pinned,
                ) {
                    eprintln!("versions: could not record the pin: {e}");
                }
            });
            versions.set_pins(crate::view_models::versions::Pins {
                is_pinned: read,
                set: write,
            });
        }

        // The manuscript as it stands now — the "now" side of the Timeline band's
        // comparison. Read on the UI thread (an `AppContext` cannot cross one) and
        // handed over as plain data, so the band's blocking half can carry it.
        {
            let app_ctx = self.app_ctx.clone();
            let work_id = ids.work_id.clone();
            timeline.set_live_source(std::rc::Rc::new(move || match work_id.get() {
                Some(wid) => crate::models::live_manuscript(&app_ctx, wid),
                None => Vec::new(),
            }));
        }

        let uid_of: crate::docks::versions::UidLookup = {
            let app_ctx = self.app_ctx.clone();
            let work_id = ids.work_id.clone();
            std::rc::Rc::new(move |id: u64| {
                let wid = work_id.get()?;
                crate::models::ordered_binder_items(&app_ctx, wid)
                    .into_iter()
                    .find(|r| r.id == id)
                    .map(|r| r.uid)
            })
        };

        // Putting a past version back — assembled here for the same reason the
        // two above are: it needs the open-documents store, the backup scheduler,
        // the save state and this Work's undo stack, none of which the dock may
        // reach for itself. Every one of them is the *session's*, never
        // `ctx.app_state::<T>()`, which is one process-wide slot that answers
        // with whichever window registered last (see `app.rs`'s own note).
        let restore: crate::docks::versions::RestoreFn = {
            let app_ctx = self.app_ctx.clone();
            let docs = session.open_docs.clone();
            let scheduler = session.backup_scheduler.clone();
            let editors_for_save = editors.clone();
            let stack_id = ids.stack_id.clone();
            std::rc::Rc::new(move |ctx: &mut EventContext, req: RestoreRequest| {
                crate::app::restore_version(
                    &app_ctx,
                    &docs,
                    &scheduler,
                    &editors_for_save,
                    &stack_id,
                    ctx,
                    req,
                )
            })
        };

        // ── Center: split editor — two panes in a Splitter ───────────────────
        // Each pane is a zoned `DropTarget` wrapping a `TabWidget`, so a binder
        // row dragged from the outline opens on the pane it's dropped over. The
        // primary pane's `Trailing` (right-edge) zone opens to the side; it
        // deactivates once split (`enabled(split_active.not())`). Closing a tab
        // saves it first (`on_close` is a pre-close intercept). Tabs migrate
        // between panes (`accept_external_tabs` + `on_tab_received` dedup).
        let split_button = {
            let editors = editors.clone();
            IconButton::new(crate::icons::editor::split())
                .tooltip(tr!(split_editor()))
                .icon_role(split_active.map(|on| {
                    if *on {
                        TextRole::Accent
                    } else {
                        TextRole::Primary
                    }
                }))
                .on_activate_fn(move |_ctx| editors.toggle_split())
        };
        let close_split_button = {
            let editors = editors.clone();
            IconButton::new(crate::icons::editor::close_split())
                .tooltip(tr!(close_split_view()))
                .on_activate_fn(move |_ctx| editors.close_split())
        };

        // Follow keyboard focus, not just tab selection: when focus enters a
        // pane's content (e.g. clicking into its editor), mark that pane focused so
        // the Inspector + open-item marker track the pane you're actually working
        // in. `focus_within` is set by the framework when a descendant has focus.
        let primary_focus = Signal::new(false);
        let secondary_focus = Signal::new(false);
        for (sig, side) in [
            (&primary_focus, Side::Primary),
            (&secondary_focus, Side::Secondary),
        ] {
            let editors = editors.clone();
            ctx.effect(sig, move |&focused| {
                if focused {
                    editors.set_focused(side);
                }
            });
        }

        // The editor tab strip is always shown. Distraction-free mode no longer
        // takes it away here: the mode does not undress this shell, it covers it
        // — and its own surface shows exactly one document, with no tab row at
        // all. (The "Editor tabs" setting that used to override this went with
        // it: there is nothing left for it to act on.)
        let tab_bar_visibility = Signal::new(TabBarVisibility::Always);

        let primary_pane = {
            let e = editors.clone();
            DropTarget::new()
                .variant(DropTargetVariant::Prominent)
                .zone_size_factor(0.3)
                .accept_when(|p| {
                    p.get_typed::<RowDragData<TreeNode>>()
                        .is_some_and(|d| d.is_export())
                })
                .region(DropRegion::Center, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_here())))
                })
                .region(DropRegion::Trailing, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_to_side())))
                        .enabled(split_active.not())
                })
                .on_region_drop(move |region, payload, _pos, _ctx| {
                    drain_dropped(payload, |item_id, title| match region {
                        DropRegion::Trailing => e.open_to_side(item_id, title),
                        _ => e.open_in(Side::Primary, item_id, title),
                    })
                })
                .child(build_pane_tabs(
                    &editors,
                    Side::Primary,
                    split_button,
                    tab_bar_visibility.clone(),
                ))
                .focus_within(primary_focus.clone())
        };

        let secondary_pane = {
            let e = editors.clone();
            DropTarget::new()
                .variant(DropTargetVariant::Prominent)
                .accept_when(|p| {
                    p.get_typed::<RowDragData<TreeNode>>()
                        .is_some_and(|d| d.is_export())
                })
                .region(DropRegion::Center, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_here())))
                })
                .on_region_drop(move |_region, payload, _pos, _ctx| {
                    drain_dropped(payload, |item_id, title| {
                        e.open_in(Side::Secondary, item_id, title)
                    })
                })
                .child(build_pane_tabs(
                    &editors,
                    Side::Secondary,
                    close_split_button,
                    tab_bar_visibility,
                ))
                .focus_within(secondary_focus.clone())
        };

        let center = Splitter::new(editors.splitter())
            .pane(primary_pane)
            .pane(secondary_pane);

        // ── Leading dock: the binder tree, fronted by a VS Code-style activity
        //    bar (icon rail). The OutlineViewModel owns the DockingModel; the
        //    dock content itself lives in `docks::outline`. ───────────────────
        // The trailing side hosts the context Inspector (a rail dock, like the
        // outline), sized + rail-fronted on the shared DockingModel.
        //
        // These are the DEFAULT dock config + arrangement, established **once** (on
        // first build): the shared `DockingModel` persists across widget rebuilds,
        // and the per-work restore below imports each project's saved sizes /
        // selected side-tab on `LoadWork`, so re-running these on every rebuild would
        // stomp a restored (or user-adjusted) layout. The `.dock(...)` registrations
        // on `DockingLayout::new(...)` still run every build — they rebuild the dock
        // *content*, not the arrangement.
        if !self.initial_loaded {
            let docking = outline.docking();
            docking.set_side_size(DockSide::Trailing, 300.0);
            docking.set_side_rail(DockSide::Trailing, 48.0);
            // The bottom search-preview band. The bottom-LEADING corner belongs to
            // the Leading side, so the binder column runs full height and the preview
            // spans only the width beside it. (Default is `Bottom`, i.e. a full-width
            // band under everything.)
            docking.set_side_size(DockSide::Bottom, 180.0);
            docking.set_corner(DockCorner::BottomLeading, DockSide::Leading);
            // An activity bar, not a tab strip: `set_side_rail` switches the side's
            // presentation from tabs to a rail of activity glyphs, and `Compact` keeps
            // them at the standard icon-button size so the band spends its height on
            // prose rather than on chrome.
            docking.set_side_rail(DockSide::Bottom, 36.0);
            docking.set_side_rail_size(DockSide::Bottom, DockRailItemSize::Compact);
        }
        let layout = DockingLayout::new(outline.docking())
            .rail(
                DockRail::new(DockSide::Leading)
                    .background(SurfaceRole::Main)
                    .divider()
                    .action(
                        DockAction::new(
                            SETTINGS_ACTION,
                            tr!(rail_settings()),
                            crate::icons::activity::settings_icon,
                            |ctx| ctx.send_intent(Intent::new("app.settings")),
                        )
                        .placement(DockActionPlacement::Pinned),
                    ),
            )
            .rail(
                DockRail::new(DockSide::Trailing)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .rail(
                DockRail::new(DockSide::Bottom)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .center(center)
            .dock(crate::docks::outline::outline_dock(
                outline.clone(),
                self.app_ctx.clone(),
                on_open.clone(),
                active_item.clone(),
                settings.counting_method(),
            ))
            .dock(crate::docks::inspector::inspector_dock(
                self.app_ctx.clone(),
                outline.clone(),
                active_item,
                self.inspector_dock,
                session.tags.clone(),
                session.mention_index.clone(),
                session.open_docs.clone(),
            ))
            .dock(crate::docks::format::format_dock(
                format.clone(),
                self.format_dock,
            ))
            .dock(crate::docks::search::search_dock(
                search.clone(),
                self.search_dock,
            ))
            .dock(crate::docks::search_preview::search_preview_dock(
                search.clone(),
                self.format.clone(),
                self.preview_dock,
            ))
            .dock(crate::docks::trash::trash_dock(
                trash.clone(),
                self.trash_dock,
                on_open.clone(),
            ))
            .dock(crate::docks::comments::comments_project_dock(
                comments.clone(),
                self.comments_dock,
                on_open.clone(),
            ))
            .dock(crate::docks::comments::comments_document_dock(
                comments.clone(),
                self.doc_comments_dock,
                editors.active_item(),
                on_open.clone(),
            ))
            .dock(crate::docks::footnotes::footnotes_dock(
                footnotes.clone(),
                self.footnotes_dock,
                editors.active_item(),
                on_open,
            ))
            .dock(crate::docks::versions::versions_dock(
                versions.clone(),
                self.versions_dock,
                editors.active_item(),
                uid_of,
                restore,
            ))
            .dock(crate::docks::timeline::timeline_dock(
                timeline.clone(),
                self.timeline_dock,
            ));
        // The docks used to be *disabled* while the mode was active, so "the
        // editor takes the whole surface". They are not any more: the mode
        // parks this entire shell dormant behind its own surface, so there is
        // nothing to disable — and disabling a side never stopped the dock
        // *commands* from firing anyway, which is handled where those commands
        // live (`app/commands/view.rs`).
        // First-build-only default arrangement (see the config block above on why
        // it must not re-run on rebuilds).
        if !self.initial_loaded {
            // Arrange the pristine desk straight from the dock roster, in its
            // declared order. The roster — not this sequence — is the single source
            // of truth, because the restore-time reconcile reads the same table to
            // mount docks a saved desk has never heard of; see `docks::APP_DOCKS`
            // for why a dock listed in only one of the two goes missing.
            //
            // Each side hosts its docks as separate switchable rail tabs — VS Code
            // style: the rail shows every glyph, selecting one shows only its panel.
            // That is `own_tab`; the stacking placement instead shows both at once in
            // a vertical split, which starves the binder.
            let docking = outline.docking();
            for dock in &crate::docks::all_docks() {
                docking.open_dock(dock.widget_id(), dock.location());
            }
            // The binder is the selected leading panel on launch, and the inspector
            // the trailing one: both are the older habit, and their rail siblings are
            // one click away. Revealed after the whole roster is mounted, since each
            // `own_tab` open selects the tab it just created.
            docking.reveal_dock(outline.dock_id());
            docking.reveal_dock(self.inspector_dock);
            // The bottom band is the transient search-preview: always hidden at start
            // (a result click reveals it thereafter). `open_dock` makes its side
            // visible as a side effect, so the hide must follow the mount — and it is
            // *immediate* to avoid an opening-then-closing flash on launch.
            docking.set_side_visible_immediate(DockSide::Bottom, false);
            // Snapshot this pristine arrangement as the reset target for a project
            // that has no saved layout (so an in-place switch to an unconfigured
            // project doesn't inherit the previous one's docks).
            //
            // Skipped for an attached window — see [`WindowRole::owns_desk`].
            if self.role.owns_desk() {
                session
                    .workspace_layout
                    .set_default_docks(docking.export_state());
            }
        }

        // ── Status bar (thin) with the notification bell ─────────────────────
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");
        // Status-bar dock toggles: hide/show the leading (binder) and trailing
        // (inspector) sides — like Teksilo's `docking` example.
        let dock_lead = outline.docking();
        let dock_trail = outline.docking();
        // The save indicator sits right after the binder toggle: the quiet, always-
        // there answer to "is my last paragraph on disk?" — the one thing autosave
        // mode had no way to tell you (it hides Save + Ctrl+S). Failures are toasts;
        // this is only the steady state.
        let save_indicator = crate::statusbar::save_indicator::SaveIndicator::new(
            editors.clone(),
            self.unsaved.clone(),
            settings.autosave(),
            self.backup_mode.clone(),
            // A work is open iff its `WorkInfo` shape is known (same test the File
            // menu uses to collapse its project-only items).
            single_work_info.shape().map(|s| s.is_some()),
            self.save_spinner.clone(),
            self.save_spinner_visible.clone(),
        );
        // The focused item's live word count sits right after the save glyph — the
        // quiet "how many words in this scene" a writer glances at. Counts the open
        // document's live text (so it tracks typing), off `StatsModel` over the shared
        // `OpenDocsStore`.
        let stats = crate::models::StatsModel::new(
            session.open_docs.clone(),
            editors.active_item(),
            settings.counting_method(),
        );
        let word_count_indicator = crate::statusbar::word_count_indicator::WordCountIndicator::new(
            stats.clone(),
            single_work_info.shape().map(|s| s.is_some()),
            settings.show_characters(),
        );
        // The writing session: a play/pause sprint timer + word tracker (ephemeral —
        // only its targets persist). Sits on the right of the status bar.
        let session_vm =
            crate::view_models::WritingSessionViewModel::new(stats.clone(), ctx.settings());
        let session_item = crate::statusbar::session_status_item::SessionStatusItem::new(
            session_vm.clone(),
            single_work_info.shape().map(|s| s.is_some()),
        );
        // `session.toggle` — scriptable start/pause (palette / automation); the play
        // button is the primary control. Global so it fires regardless of focus.
        {
            let vm = session_vm.clone();
            ctx.register_action_global(
                Action::new("session.toggle").on_invoke(move |_i, _c| vm.toggle()),
            );
        }
        // A work is open iff its `WorkInfo` shape is known — the same test the
        // word count, the session readout and the File menu already use.
        let has_work = single_work_info.shape().map(|s| s.is_some());
        // The Go-to popup's own tree model needs the backend subscription, like
        // the outline's; and `App` is the only place that can hand it the
        // "open this item" edge, since a view-model may not import a peer.
        self.go_to.wire(ctx);
        {
            let e = editors.clone();
            self.go_to
                .set_open_fn(std::rc::Rc::new(move |item_id, title| {
                    e.open_or_focus(item_id, title)
                }));
        }
        let status = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new()
                .spacing(8.0)
                // Leading (binder) toggle on the left; trailing (inspector) toggle
                // pushed to the right next to the notification bell.
                .child(
                    IconButton::new(crate::icons::activity::sidebar_icon())
                        .size(IconButtonSize::Compact)
                        .tooltip(tr!(statusbar_toggle_outline()))
                        .on_activate_fn(move |_| dock_lead.toggle_side_visible(DockSide::Leading)),
                )
                .child(save_indicator)
                .child(word_count_indicator)
                .child(Spacer::new())
                // "Go to…" sits before the session readout, on the trailing
                // side: it is an action, and the two items to its right are
                // readouts. Hidden with no project — there is nothing to jump
                // to, the same `has_work` test the readouts already use.
                .child(VisibleWhen::new(
                    has_work.clone(),
                    crate::statusbar::go_to_button::GoToButton::new(
                        self.go_to.clone(),
                        crate::statusbar::go_to_button::GO_TO_MAIN,
                    ),
                ))
                .child(session_item)
                .child(
                    IconButton::new(crate::icons::activity::inspector_icon())
                        .size(IconButtonSize::Compact)
                        .tooltip(tr!(statusbar_toggle_inspector()))
                        .on_activate_fn(move |_| {
                            dock_trail.toggle_side_visible(DockSide::Trailing)
                        }),
                )
                .child(
                    crate::statusbar::notification_bell::NotificationBell::new(
                        archive,
                        ids.work_id.clone(),
                    )
                    .size(IconButtonSize::Compact),
                ),
        );

        // The permanent backup banner sits above everything while a backup file is
        // open (zero height otherwise).
        let backup_banner = crate::backup::banner::BackupBanner::new(
            self.backup_context.clone(),
            restore_vm.clone(),
            save_as_vm.clone(),
            single_work.clone(),
        );

        // Distraction-free mode collapses **nothing** here any more. The whole
        // shell — this tree and the title bar above it — is parked dormant by a
        // single `VisibleWhen` at the window root while the mode's own surface
        // is up (`shell::windows`), so the banner, the divider and the status bar
        // need no gates of their own, and the control strip lives on the surface
        // rather than standing in for the status bar here.
        //
        // Hand the surface what only exists in here. Idempotent: `build` re-runs
        // on every rebuild of the shell and re-attaching just re-points the
        // handles.
        self.df_surface.attach(crate::view_models::SurfaceDeps {
            editors: editors.clone(),
            stats: stats.clone(),
            session_vm: session_vm.clone(),
            has_work: single_work_info.shape().map(|s| s.is_some()),
            show_characters: settings.show_characters(),
            chrome: crate::statusbar::focus_strip::FocusStripChrome::from_settings(&settings),
            themes: ctx
                .app_state::<crate::view_models::DistractionFreeThemesViewModel>()
                .cloned()
                .expect("main registers the distraction-free theme library"),
            theme_id: settings.distraction_free_theme(),
            settings: settings.clone(),
        });

        // Escape-to-leave-the-mode lives on the surface itself
        // (`distraction_free::surface`), not here: this subtree is dormant while
        // the mode is up, and a dormant widget receives no events at all.
        ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(backup_banner)
                .child(Divider::new())
                .child(Expand::new().child(layout))
                .child(status),
        )
    }
}
