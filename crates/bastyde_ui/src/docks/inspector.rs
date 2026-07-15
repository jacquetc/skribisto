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
    PopoverButton, TextWidget, Toggle, VStack,
};

use frontend::AppContext;

use crate::docks::outline::promote_menu;
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
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    probe: SingleBinderItem,
    root_child: Option<WidgetId>,
}

impl Inspector {
    fn new(app_ctx: Rc<AppContext>, outline: OutlineViewModel, focus: Signal<Option<u64>>) -> Self {
        Self {
            probe: SingleBinderItem::new(app_ctx.clone()),
            app_ctx,
            outline,
            focus,
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
        // Rebuild when focus moves to a different item...
        self.focus
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        // ...and when the focused item *itself* changes under us. Promote rewrites its
        // `(role, sub_role)` and a rename its title, neither of which moves focus — so
        // without this the panel kept offering the conversions of the item's *old* type.
        self.probe.dto_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // Re-subscribe on **every** build, not once: `BuildContext::subscribe_event`
        // scopes the subscription to the widget's *current* build and drops it on the
        // next one. Guarding this with a "wired" flag made the panel deaf the moment it
        // first rebuilt — which is exactly when it needed to keep listening.
        self.probe.wire(ctx);

        let item_id = self.focus.get();
        // Only re-point on a genuine focus change: `set_id` re-reads synchronously and
        // writes the dto signal, which is now a rebuild trigger.
        if self.probe.id() != item_id {
            self.probe.set_id(item_id);
        }
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
                // The headline affordance: convert this item to another type. A folder
                // can become any other kind of folder, so it is a menu, not a button.
                let key = BinderTreeKey::Item(d.id);
                let targets = self.outline.promote_targets_of(key);
                if !targets.is_empty() {
                    let outline = self.outline.clone();
                    col = col.child(
                        PopoverButton::new(
                            Button::new(tr!(inspector_promote())).variant(ButtonVariant::Tinted),
                        )
                        .content(promote_menu(outline, key)),
                    );
                }
                // Per-item language override (Step 9): the pill field over this item's own
                // `dict_language`, with the inherited list (item → Book → Work) as the
                // placeholder shown when the item declares none of its own.
                if let Some(spell) = ctx.app_state::<crate::spellcheck::SpellcheckService>().cloned()
                {
                    let inherited = ctx
                        .app_state::<crate::models::OpenDocsStore>()
                        .map(|s| s.effective_language(d.id));
                    let value = Signal::new(d.dict_language.clone());
                    // A probe fixed to *this* item, so the write targets it even after focus
                    // moves on (unlike the shared `self.probe`).
                    let item_probe = SingleBinderItem::new(self.app_ctx.clone());
                    item_probe.set_id(Some(d.id));
                    let stack = self.outline.ids().stack_id.get();
                    let set: crate::language_pill_field::SetLanguages = {
                        let value = value.clone();
                        Rc::new(move |new: String, _c| {
                            let _ = item_probe.set_dict_language(&new, stack);
                            value.set(new);
                        })
                    };
                    col = col
                        .child(TextWidget::new(tr!(inspector_dict_language())).style(TextStyleRole::Tiny).color(TextRole::Secondary))
                        .child(crate::language_pill_field::LanguagePillField::new(
                            value, set, spell, inherited,
                        ));
                }
                // Per-item **export** toggle (M3): whether this item is included when a
                // structural scope (Book / Chapter / Folder) sweeps it in. On by default; an
                // explicit Export Scene/Note or a checked Choose… item overrides it. Beside
                // it, "Apply to children" pushes this value across the whole subtree in one
                // undo step (shown only when the item actually has a subtree).
                {
                    let value = Signal::new(d.is_exportable);
                    let item_probe = SingleBinderItem::new(self.app_ctx.clone());
                    item_probe.set_id(Some(d.id));
                    let stack = self.outline.ids().stack_id.get();
                    {
                        let probe = item_probe.clone();
                        // Write only on a genuine change — never on the initial seed nor the
                        // post-write echo (the entity Updated event rebuilds this panel), so
                        // the toggle can't feed back into itself.
                        ctx.effect(&value, move |on| {
                            if probe.dto().map(|d| d.is_exportable) != Some(*on) {
                                let _ = probe.set_exportable(*on, stack);
                            }
                        });
                    }
                    col = col.child(
                        TextWidget::new(tr!(inspector_export()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    );
                    col = col.child(Toggle::new(value.clone()).label(tr!(inspector_exportable())));
                    if !self.outline.subtree_descendants(d.id).is_empty() {
                        let outline = self.outline.clone();
                        let id = d.id;
                        col = col.child(
                            Button::new(tr!(inspector_apply_to_children()))
                                .variant(ButtonVariant::Plain)
                                .on_activate_fn(move |_c| {
                                    outline.apply_exportable_to_subtree(id, value.get())
                                }),
                        );
                    }
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
