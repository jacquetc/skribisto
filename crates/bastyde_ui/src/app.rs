//! The root application widget: a toolbar (open + settings) and the binder tree.
//!
//! The window's custom `TitleBar` is set up at the window root in `main.rs`;
//! this widget is the body below it (toolbar + tree).

use std::rc::Rc;

use bastyde::core::modal::ModalRequest;
use bastyde::core::widget::WidgetPlacement;
use bastyde::data::{FlatEntry, SelectionMode, SelectionModel};
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, Expand, HStack, IconButtonSize, NotificationArchiveModel, NotificationCenterButton,
    Spacer, StandardTreeItem, StatusBar, Toast, Toolbar, TreeView, VStack,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::event::{Event, Origin, WorkManagementEvent};
use frontend::work_management::LoadWorkDto;

use crate::models::{BinderBinderItemsTreeModel, TreeNode};
use crate::settings_panel::SettingsPanel;

fn sample_project_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

pub struct App {
    app_ctx: Rc<AppContext>,
    model: BinderBinderItemsTreeModel,
    root_child: Option<WidgetId>,
}

impl App {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        // In `--features mocks` the model fills itself with fabricated data; the
        // real model starts empty and is filled by `reload()` on project load.
        let model = BinderBinderItemsTreeModel::new(app_ctx.clone());
        Self { app_ctx, model, root_child: None }
    }
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

impl Widget for App {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Reactive refresh: also rebuild the tree if a project loads elsewhere.
        {
            let model = self.model.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| model.reload(),
            );
        }

        // Clones captured by the reactive closures below.
        let open_ctx = self.app_ctx.clone();
        let open_model = self.model.clone();
        let tree_model = self.model.tree().clone();

        // The toast/notification archive is registered app-wide by
        // `install_toast_default()`; the status-bar bell reads its unread count.
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");

        let root = bati!(ctx =>
            VStack {
                spacing: 0.0
                Toolbar {
                    HStack {
                        spacing: 8.0
                        // Open: load the sample project, then refresh the tree
                        // immediately (not relying on the async event hop).
                        Button::new(tr!(open_project())) {
                            on_activate_fn: move |ctx| {
                                let path = sample_project_path();
                                match work_management_commands::load_work(
                                    &open_ctx,
                                    &LoadWorkDto { file_name: path },
                                ) {
                                    Ok(()) => open_model.reload(),
                                    Err(e) => {
                                        ctx.show_toast(Toast::error(lit!(format!(
                                            "Could not open project: {e}"
                                        ))));
                                    }
                                }
                            }
                        }
                        // Settings as an *in-tree* modal (scrim + centered panel)
                        // in this window's tree, not a native window — still modal
                        // (scrim blocks the UI; Escape / click-outside dismiss),
                        // but theme/locale changes restyle the live tree at once.
                        Button::new(tr!(settings())) {
                            on_activate_fn: |ctx| {
                                ctx.present_modal(
                                    ModalRequest::deferred(|tree| tree.add(SettingsPanel::new()))
                                        .presentation(ModalPresentation::InTree)
                                        .title("Settings")
                                        .size(520, 320),
                                );
                            }
                        }
                    }
                }
                // Binder-item tree. `StandardTreeItem` gives the expand chevron
                // and renders the user-note `label` as the row subtitle.
                Expand {
                    TreeView::new_with_context(
                        tree_model,
                        |node: &TreeNode, entry: &FlatEntry, selected: bool, ctx| {
                            let mut row = StandardTreeItem::new(lit!(node.title.clone()))
                                .from_entry(entry)
                                .selected(selected)
                                .on_toggle_rc(ctx.toggle_callback());
                            if !node.label.is_empty() {
                                row = row.subtitle(lit!(node.label.clone()));
                            }
                            Box::new(row) as Box<dyn Widget>
                        },
                    ) {
                        item_height: 40.0
                        selection: SelectionModel::new(SelectionMode::Single)
                        row_click_expands: false
                    }
                }
                // Status bar with the toast/notification indicator on the right.
                // Compact (22 dp) bell keeps the bar thin.
                StatusBar {
                    HStack {
                        spacing: 8.0
                        Spacer
                        NotificationCenterButton::new(archive) {
                            size: IconButtonSize::Compact
                        }
                    }
                }
            }
        );
        self.root_child = Some(root);
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
