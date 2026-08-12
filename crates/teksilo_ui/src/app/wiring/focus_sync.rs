// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A run of small wiring blocks kept together because `App::build` executes them
//! consecutively, right after the LoadWork seed: Image-menu visibility, the
//! Insert-Template submenu refill, long-operation event routing (delegates to
//! [`super::long_ops::install`], which sits in this same slot in the original
//! sequence rather than being split around), the Document-menu selection gate,
//! the two per-pane focus/flush effects, and the BinderItem-Updated resync that
//! keeps a retyped item's `(role, sub_role)` current. None of the five UI-facing
//! pieces shares state with its neighbours; they are grouped by *when* they run
//! in that sequence, not by what they do.
//!
//! [`install`] also builds and returns the outline's `on_open` callback partway
//! through — the shell built further down `App::build` needs it once the rest
//! of this sequence has run.

use std::rc::Rc;

use teksilo::prelude::*;

use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

use crate::backup::{BackupRestoreViewModel, BackupSchedulerViewModel};
use crate::editors::{EditorsViewModel, Side};
use crate::export::ExportViewModel;
use crate::import_document::ImportDocumentViewModel;
use crate::mentions::MentionIndex;
use crate::note_templates::NoteTemplatesViewModel;
use crate::save::SaveAsViewModel;
use crate::shared::ProgressRecorder;

pub(in crate::app) struct FocusSyncDeps {
    pub templates_menu: Option<(
        teksilo::widgets::MenuModel,
        teksilo::core::menu_item_id::MenuItemId,
    )>,
    pub image_menu_id: teksilo::core::menu_item_id::MenuItemId,
    pub active_image: Signal<Option<(usize, String)>>,
    pub note_templates: NoteTemplatesViewModel,
    pub has_target: Signal<bool>,
    pub save_as_vm: SaveAsViewModel,
    pub backup_scheduler: BackupSchedulerViewModel,
    pub restore_vm: BackupRestoreViewModel,
    pub export_vm: ExportViewModel,
    pub import_document: ImportDocumentViewModel,
    pub mention_index: MentionIndex,
    pub progress_recorder: ProgressRecorder,
    pub binder_has_selection: Signal<bool>,
    pub outline_selection: Signal<std::collections::HashSet<crate::models::BinderTreeKey>>,
    pub editors: EditorsViewModel,
}

pub(in crate::app) fn install(
    ctx: &mut BuildContext,
    deps: FocusSyncDeps,
) -> crate::binder::dock::OpenItemFn {
    // The Image menu appears while a picture is selected and goes away
    // when it is not. Driven off the same signal the Document menu's image
    // rows used to gate on, so the menu and the commands cannot disagree
    // about whether there is an image in hand.
    if let Some((menu, _)) = deps.templates_menu.clone() {
        let image_menu_id = deps.image_menu_id;
        let active = deps.active_image.clone();
        ctx.effect(&active, move |image| {
            crate::shell::project_menus::sync_image_menu(&menu, image_menu_id, image.is_some());
        });
    }

    if let Some((menu, submenu_id)) = deps.templates_menu.clone() {
        let templates = deps.note_templates.clone();
        let has_editor = deps.has_target.clone();
        ctx.effect(&templates.changed_signal(), move |_| {
            crate::shell::project_menus::sync_insert_template_submenu(
                &menu,
                submenu_id,
                &templates,
                &has_editor,
            );
        });
    }

    // Long-operation routing: every background job (import, export, save-as, backup,
    // restore, the progress recorder) reports through the same four events and filters
    // by its own op id. Grouped in `app::wiring::long_ops`; the editors' own save
    // routing stays below, since it also drives the deferred close/switch resumption.
    super::long_ops::install(
        ctx,
        &deps.save_as_vm,
        &deps.backup_scheduler,
        &deps.restore_vm,
        &deps.export_vm,
        &deps.import_document,
        &deps.mention_index,
        &deps.progress_recorder,
    );

    // Keep the Document menu's per-item gate in step with this window's binder
    // selection. `MenuEntry::enabled` wants a concrete `Signal<bool>`, so the set is
    // projected into one here rather than handed over as a lazy `.map()`.
    {
        let has_selection = deps.binder_has_selection.clone();
        let sig = deps.outline_selection.clone();
        ctx.effect(&sig, move |set: &std::collections::HashSet<_>| {
            let now = !set.is_empty();
            if has_selection.get() != now {
                has_selection.set(now);
            }
        });
    }

    // App mediates the two peer view-models: *activating* a binder item
    // (click or Enter — NOT arrow navigation, which only moves the selection)
    // opens (or focuses) its editor tab. The tree fires this via
    // `TreeView::on_activate`; App supplies the open callback so neither
    // view-model imports the other.
    let on_open: crate::binder::dock::OpenItemFn = {
        let editors = deps.editors.clone();
        Rc::new(move |item_id, title| editors.open_or_focus(item_id, &title))
    };

    // Keep the "open document" id in sync with each pane's active tab (open,
    // close, or a tab-bar click), so the binder's open-item marker tracks the
    // focused pane. A selection change in a pane also marks it focused and
    // flushes pending edits to the store — autosave on a natural boundary
    // (changed fields only; clean tabs are a no-op). One effect per pane.
    //
    // Flush the *whole* store, not only the leaving tab: a shared
    // `OpenDoc` can be open in both panes (or in a stream row plus a
    // tab), and a title/prose edit on the tab we leave must land
    // before the next tab paints. Clean docs are a field-level no-op
    // (`is_modified` / title probe), so the cost stays proportional
    // to *dirty* open docs, not to how fast the user clicks tabs.
    for side in [Side::Primary, Side::Secondary] {
        let editors = deps.editors.clone();
        ctx.effect(&editors.selected(side), move |_| {
            editors.flush_all();
            editors.set_focused(side);
        });
    }

    // A focused item can be **retyped in place**: Promote rewrites a
    // `sub_role` (flat Chapter ↔ Chapter folder, Scene ↔ Note) leaving the id,
    // the pane and the selection exactly where they were. So the
    // `(role, sub_role)` `ActiveContext` publishes would stay at its old value
    // until the writer happened to click elsewhere — a stale answer to the one
    // question a dock asks it. Nothing else here needs the resync: `active_item`
    // is an id and the id does not move.
    {
        let editors = deps.editors.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
            move |e: &Event| {
                let Some(active) = editors.active_item().get() else {
                    return;
                };
                if e.ids.contains(&active) {
                    editors.sync_active_item();
                }
            },
        );
    }

    on_open
}
