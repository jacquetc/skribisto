//! The root application widget: a toolbar (open + settings) and the binder tree.
//!
//! The window's custom `TitleBar` is set up at the window root in `main.rs`;
//! this widget is the body below it (toolbar + tree).

use std::rc::Rc;

use bastyde::core::widget::WidgetPlacement;
use bastyde::data::{FlatEntry, SelectionMode, SelectionModel, TreeModel};
use bastyde::prelude::*;
use bastyde::widgets::{Button, Expand, HStack, StandardTreeItem, Toast, Toolbar, TreeView, VStack};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::event::{Event, Origin, WorkManagementEvent};
use frontend::work_management::LoadWorkDto;

use crate::models::{self, TreeNode};
use crate::settings_panel::SettingsPanel;

fn sample_project_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

pub struct App {
    app_ctx: Rc<AppContext>,
    tree_model: TreeModel<TreeNode>,
    root_child: Option<WidgetId>,
}

impl App {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        let tree_model = TreeModel::new();
        #[cfg(feature = "mocks")]
        crate::mock::populate(&tree_model);
        Self { app_ctx, tree_model, root_child: None }
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
            let model = self.tree_model.clone();
            let app_ctx = self.app_ctx.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| {
                    models::populate_from_backend(&model, &app_ctx);
                },
            );
        }

        // Open: load the sample project, then refresh the tree immediately
        // (not relying on the async event hop).
        let open_ctx = self.app_ctx.clone();
        let open_model = self.tree_model.clone();
        let open_btn = Button::new(tr!(open_project())).on_activate_fn(move |ctx| {
            let path = sample_project_path();
            match work_management_commands::load_work(&open_ctx, &LoadWorkDto { file_name: path }) {
                Ok(()) => models::populate_from_backend(&open_model, &open_ctx),
                Err(e) => {
                    ctx.show_toast(Toast::error(lit!(format!("Could not open project: {e}"))));
                }
            }
        });

        let settings_btn = Button::new(tr!(settings())).on_activate_fn(|ctx| {
            ctx.open_window(
                WindowConfig::new()
                    .title("Settings")
                    .size(520, 320)
                    .root(|tree, _state| tree.add(SettingsPanel::new())),
            );
        });

        // Binder-item tree. `StandardTreeItem` provides the expand chevron and
        // renders the user-note `label` as the row subtitle.
        let tree = TreeView::new_with_context(
            self.tree_model.clone(),
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
        )
        .item_height(40.0)
        .selection(SelectionModel::new(SelectionMode::Single))
        .row_click_expands(false);

        let header = Toolbar::new().child(
            HStack::new().spacing(8.0).child(open_btn).child(settings_btn),
        );

        let root = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(header)
                .child(Expand::new().child(tree)),
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
