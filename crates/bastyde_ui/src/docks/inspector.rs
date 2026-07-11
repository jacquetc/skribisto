//! The trailing **Inspector** dock: a context panel that adapts to the focused
//! binder item (the active editor tab). It shows the item's title and — for any
//! item with a promote pair (Chapter, ChapterScene, Scene, Note, Folder, Note
//! folder) — a "Promote to `<target>`" button, the Chapter/ChapterScene inspectors'
//! headline affordance. Rebuilds when the focused item changes.
//!
//! It reuses the shared [`promote_with_guard`] so the button behaves exactly like
//! the outline context menu (incl. the demote-empty MessageBox).

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, DockOpenLocation, DockSide, DockWidget, DockWidgetId, Padding,
    TextWidget, VStack,
};

use frontend::AppContext;

use crate::create_labels::promote_target_label;
use crate::docks::outline::promote_with_guard;
use crate::models::BinderTreeKey;
use crate::singles::SingleBinderItem;
use crate::view_models::OutlineViewModel;

/// Package the inspector as a trailing `DockWidget`. `focus` is the active
/// editor tab's item id (the "focused" binder item).
pub fn inspector_dock(
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    dock_id: DockWidgetId,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(inspector()), move |_id| {
        Inspector::new(app_ctx.clone(), outline.clone(), focus.clone())
    })
    .icon(crate::activity_icons::inspector_icon)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

struct Inspector {
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    probe: SingleBinderItem,
    root_child: Option<WidgetId>,
}

impl Inspector {
    fn new(app_ctx: Rc<AppContext>, outline: OutlineViewModel, focus: Signal<Option<u64>>) -> Self {
        Self {
            outline,
            focus,
            probe: SingleBinderItem::new(app_ctx),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for Inspector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inspector").finish()
    }
}

impl Widget for Inspector {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Follow the focused item — rebuild the panel when it changes.
        self.focus
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let item_id = self.focus.get();
        self.probe.set_id(item_id);
        let dto = item_id.and_then(|_| self.probe.dto());

        let body: Box<dyn Widget> = match dto {
            None => Box::new(
                Padding::symmetric(16.0, 16.0)
                    .child(TextWidget::new(tr!(inspector_empty())).color(TextRole::Secondary)),
            ),
            Some(d) => {
                let mut col = VStack::new()
                    .spacing(12.0)
                    .child(TextWidget::new(lit!(d.title.clone())).style(TextStyleRole::BodyBold));
                // The headline affordance: convert to the paired type.
                if let Some((target_role, target_sub_role)) =
                    skribisto_model::promote_target(&d.role, &d.sub_role)
                {
                    let label = promote_target_label(&target_role, &target_sub_role);
                    let outline = self.outline.clone();
                    let key = BinderTreeKey::Item(d.id);
                    col = col.child(
                        Button::new(tr!(inspector_promote_to(target = label.resolve_now())))
                            .variant(ButtonVariant::Tinted)
                            .on_activate_fn(move |ctx| promote_with_guard(&outline, key, ctx)),
                    );
                }
                Box::new(Padding::symmetric(16.0, 16.0).child(col))
            }
        };

        let id = ctx.add_boxed(body);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }
}
