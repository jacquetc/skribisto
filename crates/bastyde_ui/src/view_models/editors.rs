//! `EditorsViewModel` — the open-editor tab set and the open/focus/close logic.
//!
//! Single-instance live state: owns the tab `ListModel` and the selection signal,
//! so `App` creates exactly one and shares it by clone.

use std::rc::Rc;

use bastyde::data::ListModel;
use bastyde::prelude::*; // EventContext, Signal, tr!, lit!
use bastyde::widgets::{TabHandle, TabId, TabInfo};

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands, work_management_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::direct_access::ContentDto;
use frontend::work_management::SaveWorkDto;

use crate::singles::SingleBinderItem;
use crate::tabs::{self, ContentTab};

#[derive(Clone)]
pub struct EditorsViewModel {
    app_ctx: Rc<AppContext>,
    tabs: ListModel<TabHandle>,
    selected_tab: Signal<Option<TabId>>,
    /// The `BinderItem` of the currently-active editor tab — the "open
    /// document". Drives the binder's persistent open-item marker (independent
    /// of selection/focus). Kept in sync with `selected_tab`.
    active_item: Signal<Option<u64>>,
    column_width: Signal<f32>,
    /// Persisted "show synopsis pane" setting, threaded into every opened tab so
    /// the dual-pane editor shows/hides its synopsis live.
    show_synopsis: Signal<bool>,
    /// The per-`Work` undo stack id — shared with `OutlineViewModel` so editor
    /// write-back lands on the same Ctrl+Z history as tree edits. `App` wires it.
    stack_id: Signal<Option<u64>>,
    /// Reactive read handle re-pointed at an item when opening its tab — supplies
    /// the `(role, sub_role)` that selects the tab layout (Layer A single).
    item_probe: SingleBinderItem,
    /// Bumped by every open tab's editor `on_change` — the edit signal the
    /// debounced autosave timer (in `App`) observes.
    edited: Signal<u64>,
}

impl EditorsViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        column_width: Signal<f32>,
        show_synopsis: Signal<bool>,
        stack_id: Signal<Option<u64>>,
    ) -> Self {
        Self {
            item_probe: SingleBinderItem::new(app_ctx.clone()),
            app_ctx,
            tabs: ListModel::from_vec(Vec::new()),
            selected_tab: Signal::new(None),
            active_item: Signal::new(None),
            column_width,
            show_synopsis,
            stack_id,
            edited: Signal::new(0),
        }
    }

    /// The "an edit happened" signal — bind the debounced autosave to it.
    pub fn edited_signal(&self) -> Signal<u64> {
        self.edited.clone()
    }

    /// The dynamic-tab model to hand to `TabWidget::dynamic_model`.
    pub fn tabs(&self) -> ListModel<TabHandle> {
        self.tabs.clone()
    }

    /// The selected-tab signal to hand to `TabWidget::new`.
    pub fn selected_tab(&self) -> Signal<Option<TabId>> {
        self.selected_tab.clone()
    }

    /// The currently-open item id (active editor tab). Bind a binder row's
    /// "open document" accent to this.
    pub fn active_item(&self) -> Signal<Option<u64>> {
        self.active_item.clone()
    }

    /// Recompute `active_item` from the currently-selected tab. Call whenever
    /// `selected_tab` changes (open, close, or a tab-bar click).
    pub fn sync_active_item(&self) {
        let active = self
            .selected_tab
            .get()
            .and_then(|tab| self.item_of_tab(tab));
        if self.active_item.get() != active {
            self.active_item.set(active);
        }
    }

    /// The `BinderItem` id behind a tab, if it's an editor tab.
    fn item_of_tab(&self, tab: TabId) -> Option<u64> {
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                if h.id == tab {
                    h.payload.downcast_ref::<ContentTab>().map(|e| e.item_id)
                } else {
                    None
                }
            });
            if let Some(Some(id)) = hit {
                return Some(id);
            }
        }
        None
    }

    /// Open the editor tab for `item_id`, or focus it if already open. The view
    /// is chosen per `(role, sub_role)` — not every row is the prose editor.
    pub fn open_or_focus(&self, item_id: u64, title: &str) {
        if let Some(tid) = self.find_open(item_id) {
            self.selected_tab.set(Some(tid));
            return;
        }
        // Read the item's `(role, sub_role)` through the reactive single rather
        // than an ad-hoc `get_binder_item` (Layer A).
        self.item_probe.set_id(Some(item_id));
        let Some(item) = self.item_probe.dto() else {
            return;
        };
        let contents = self.load_contents(item_id, &item.role, &item.sub_role);
        let mut tab = tabs::tab_for(
            &self.app_ctx,
            item_id,
            &item.role,
            &item.sub_role,
            &contents,
            self.column_width.clone(),
            self.show_synopsis.clone(),
        );
        // Every tab bumps the shared edit signal, so the autosave timer sees edits
        // from whichever tab is active.
        tab.edited = Some(self.edited.clone());
        // The item's title (data), or a translated "Untitled" fallback for
        // empty ones — locale-reactive so a language switch re-labels the tab.
        let tab_title = if title.is_empty() {
            tr!(untitled())
        } else {
            lit!(title.to_string())
        };
        // Leading icon by sub_role — matches the outline row's glyph. The
        // factory is re-invoked per header build, so capture an owned sub_role.
        let sub_role = item.sub_role.clone();
        let id = TabId::fresh();
        self.tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new()
                .title(tab_title)
                .closable(true)
                .icon(move || crate::binder_icons::sub_role_icon(&sub_role)),
            tab,
        ));
        self.selected_tab.set(Some(id));
    }

    /// Persist every open tab's edits back to its `Content` rows (changed fields
    /// only), through the per-Work undo stack.
    pub fn flush_all(&self) {
        let stack = self.stack_id.get();
        for i in 0..self.tabs.len() {
            self.tabs.with_item(i, |h| {
                if let Some(t) = h.payload.downcast_ref::<ContentTab>() {
                    let _ = t.flush(stack);
                }
            });
        }
    }

    /// Save the tab with `tab_id` (if any) then remove it — the `TabWidget`'s
    /// `on_close` hook, so closing never drops unsaved edits.
    pub fn flush_and_close(&self, tab_id: TabId) {
        let stack = self.stack_id.get();
        let mut pos = None;
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                if h.id == tab_id {
                    if let Some(t) = h.payload.downcast_ref::<ContentTab>() {
                        let _ = t.flush(stack);
                    }
                    true
                } else {
                    false
                }
            });
            if hit == Some(true) {
                pos = Some(i);
                break;
            }
        }
        if let Some(p) = pos {
            self.tabs.remove(p);
        }
        if self.selected_tab.get() == Some(tab_id) {
            let next = (0..self.tabs.len()).find_map(|i| self.tabs.with_item(i, |h| h.id));
            self.selected_tab.set(next);
        }
    }

    /// Flush all editors to the store, then write the project to disk
    /// (`save_work`). The save is a long operation; we kick it and let the
    /// long-operation manager run it.
    pub fn save_to_disk(&self) {
        self.flush_all();
        let _ = work_management_commands::save_work(
            &self.app_ctx,
            &SaveWorkDto {
                file_name: String::new(),
                overwrite: true,
            },
        );
    }

    /// Close every open tab (e.g. on project load).
    pub fn close_all(&self) {
        while !self.tabs.is_empty() {
            self.tabs.remove(0);
        }
        self.selected_tab.set(None);
    }

    /// `Some(tab id)` if an editor for `item_id` is already open.
    fn find_open(&self, item_id: u64) -> Option<TabId> {
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<ContentTab>()
                    .filter(|e| e.item_id == item_id)
                    .map(|_| h.id)
            });
            if let Some(Some(tid)) = hit {
                return Some(tid);
            }
        }
        None
    }

    /// Read an item's content rows, keeping only the roles the constraint
    /// matrix allows for its `(role, sub_role)`. `Content.data` is Djot (the
    /// canonical store format).
    fn load_contents(
        &self,
        item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Vec<ContentDto> {
        let ctx = &*self.app_ctx;
        let allowed = skribisto_model::allowed_content(role, sub_role);
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(|c| allowed.contains(&c.role))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn editors() -> EditorsViewModel {
        EditorsViewModel::new(
            Rc::new(AppContext::new()),
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(None),
        )
    }

    /// Push an editor tab directly (bypassing the backend) so tab-management
    /// logic can be tested without a loaded project.
    fn push_tab(p: &EditorsViewModel, item_id: u64) -> TabId {
        let id = TabId::fresh();
        let tab = tabs::tab_for(
            &p.app_ctx,
            item_id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            p.column_width.clone(),
            p.show_synopsis.clone(),
        );
        p.tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new().closable(true),
            tab,
        ));
        id
    }

    #[test]
    fn open_or_focus_dedupes_an_already_open_tab() {
        let vm = editors();
        let id = push_tab(&vm, 42);
        assert_eq!(vm.tabs().len(), 1);
        // Already open → focuses it, no backend hit, no new tab.
        vm.open_or_focus(42, "Scene");
        assert_eq!(vm.tabs().len(), 1);
        assert_eq!(vm.selected_tab().get(), Some(id));
    }

    #[test]
    fn close_all_empties_and_clears_selection() {
        let vm = editors();
        push_tab(&vm, 1);
        push_tab(&vm, 2);
        vm.selected_tab().set(Some(TabId::fresh()));
        vm.close_all();
        assert_eq!(vm.tabs().len(), 0);
        assert_eq!(vm.selected_tab().get(), None);
    }
}
