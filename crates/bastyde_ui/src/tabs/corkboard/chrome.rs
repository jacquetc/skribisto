// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The footer count, the per-card menu, and the a11y / badge labels.

#[allow(unused_imports)]
use super::*;

/// A card's footer count: a container shows its child count; a leaf shows its word
/// count (reactive — it loads lazily), gated by the show-word-count setting.
pub(super) struct FooterCount {
    pub(super) is_container: bool,
    pub(super) child_count: usize,
    pub(super) word_count: Signal<Option<usize>>,
    pub(super) show_wc: Signal<bool>,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for FooterCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FooterCount").finish()
    }
}
impl Widget for FooterCount {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.word_count
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.show_wc
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let label = if self.is_container {
            tr!(corkboard_child_count(count = self.child_count as i64))
        } else if self.show_wc.get() {
            match self.word_count.get() {
                Some(n) => tr!(statusbar_word_count(count = n as i64)),
                None => lit!(""),
            }
        } else {
            lit!("")
        };
        let id = ctx.add(TextWidget::new(label).color(TextRole::Secondary));
        self.root = Some(id);
        vec![id]
    }
    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The per-card "More actions" menu — the same actions as the Full-Synopsis row
/// menu (insert · set label · move up / down · merge · trash), minus rename. Merge
/// is offered only where the model allows it. Bare kebab `IconButton`.
pub(super) fn card_menu(vm: &CorkboardViewModel, card: &CorkboardCard) -> impl Widget {
    let id = card.item_id;
    let mk = |f: fn(&CorkboardViewModel, &mut EventContext, u64)| {
        let vm = vm.clone();
        move |ctx: &mut EventContext| f(&vm, ctx, id)
    };

    let mut list = MenuList::new()
        // Rename opens the inline title editor on this card (same as F2 /
        // double-clicking the title) — no modal.
        .item(MenuItem::new(tr!(rename())).on_activate_fn(mk(|v, _c, id| v.begin_rename(id))))
        .item(
            MenuItem::new(insert_label(card))
                .on_activate_fn(mk(|v, c, id| v.begin_insert_after(c, id))),
        )
        .item(
            MenuItem::new(tr!(set_label())).on_activate_fn(mk(|v, c, id| v.begin_set_label(c, id))),
        )
        .separator()
        .item(MenuItem::new(tr!(move_up())).on_activate_fn(mk(|v, c, id| v.move_up(c, id))))
        .item(MenuItem::new(tr!(move_down())).on_activate_fn(mk(|v, c, id| v.move_down(c, id))));

    if vm.can_merge_into_previous(id) {
        list = list.item(
            MenuItem::new(tr!(merge_with_previous()))
                .on_activate_fn(mk(|v, c, id| v.merge_into_previous(c, id))),
        );
    }

    list = list.separator().item(
        MenuItem::new(tr!(move_to_trash()))
            .text_role(TextRole::Error)
            .on_activate_fn(mk(|v, c, id| v.trash(c, id))),
    );

    PopoverIconButton::new(IconButton::more())
        .bare()
        .content(list)
}

/// What "Insert …" on a card creates — the model's default recommendation for it
/// — so the menu item names that type rather than always saying "scene".
pub(super) fn insert_label(card: &CorkboardCard) -> LocalizedString {
    let recommended = skribisto_model::recommendations(&card.role, &card.sub_role)
        .first()
        .map(|r| r.create_type);
    match recommended {
        Some(skribisto_model::CreateType::Chapter) => tr!(insert_chapter()),
        _ => tr!(insert_scene()),
    }
}

/// The concise accessible name a screen reader announces for a card's `GridCell`:
/// a superset of the visible title (Label-in-Name) — "Title, Type[, status]" —
/// not the whole synopsis (which is scanned visually / read on demand).
pub(super) fn card_a11y_name(card: &CorkboardCard) -> String {
    let mut name = format!(
        "{}, {}",
        card.title,
        sub_role_badge_label(&card.sub_role).resolve_now()
    );
    if !card.label.is_empty() {
        name = format!("{name}, {}", card.label);
    }
    name
}

/// A short, sentence-case badge for a card's type. No existing sub_role→text map
/// (the icons are `binder_icons`), so it lives here, the only consumer.
pub(super) fn sub_role_badge_label(sub_role: &BinderItemSubRole) -> LocalizedString {
    use BinderItemSubRole::*;
    match sub_role {
        Scene => tr!(corkboard_badge_scene()),
        ChapterScene => tr!(corkboard_badge_chapter()),
        Part => tr!(corkboard_badge_part()),
        Book | BookBegin => tr!(corkboard_badge_book()),
        Note => tr!(corkboard_badge_note()),
        None => tr!(corkboard_badge_folder()),
        BookEnd => tr!(corkboard_badge_end()),
        Text => tr!(corkboard_badge_text()),
    }
}
