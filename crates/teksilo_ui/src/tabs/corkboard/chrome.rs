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

/// The kebab, wrapped so its menu **rebuilds when the selection changes**.
///
/// The labels below are counted ("Delete 4 cards"), and the count comes from the
/// live selection — but `GridView` never rebuilds a tile on selection (its
/// `is_selected` is a repaint-only binding, see [`CorkboardTile`]), so a menu
/// built with the tile would keep whatever count was current when the card was
/// first realized. A menu reading "Delete" that deletes four cards is exactly the
/// kind of quiet mismatch these labels exist to prevent, so the binding lives
/// here, on a widget of its own, rather than on the tile.
pub(super) struct CardMenu {
    pub(super) vm: CorkboardViewModel,
    pub(super) card: CorkboardCard,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CardMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardMenu").finish()
    }
}
impl Widget for CardMenu {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.selection().selection_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // A dead zone for the same reason the synopsis is one: the kebab captures
        // the pointer for its own tap, which would otherwise arm the tile's drag and
        // let a click-with-jitter drag the card. See `CardSynopsis::build`.
        let id = ctx.add(DeadZone::new().child(card_menu(&self.vm, &self.card)));
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

/// The per-card "More actions" menu — the Full-Synopsis row menu's actions
/// (rename · insert · set label · move up / down · merge · trash) plus the
/// board's own (duplicate · move to… · reveal in the outline). Merge is offered
/// only where the writing model allows it. Bare kebab `IconButton`.
///
/// Built through [`CardMenu`], never directly, so its counted labels stay in step
/// with the selection.
fn card_menu(vm: &CorkboardViewModel, card: &CorkboardCard) -> impl Widget {
    let id = card.item_id;
    let mk = |f: fn(&CorkboardViewModel, &mut EventContext, u64)| {
        let vm = vm.clone();
        move |ctx: &mut EventContext| f(&vm, ctx, id)
    };

    // How many cards the batch actions below will touch — the whole selection when
    // this card is inside it, otherwise just this one (`batch_for`). Kept fresh by
    // [`CardMenu`], which rebuilds this list whenever the selection changes.
    let batch = vm.batch_len(id);

    let mut list = MenuList::new()
        // Rename opens the inline title editor on this card (same as F2 /
        // double-clicking the title) — no modal. Single-card by nature: there is
        // one title field, and it is this card's.
        .item(MenuItem::new(tr!(rename())).on_activate_fn(mk(|v, _c, id| v.begin_rename(id))))
        .item(
            MenuItem::new(insert_label(card))
                .on_activate_fn(mk(|v, c, id| v.begin_insert_after(c, id))),
        )
        .item(
            MenuItem::new(batched(
                tr!(set_label()),
                tr!(set_label_n(count = batch as i64)),
                batch,
            ))
            .on_activate_fn(mk(|v, c, id| v.begin_set_label(c, id))),
        )
        .separator()
        .item(
            MenuItem::new(batched(
                tr!(duplicate()),
                tr!(duplicate_n(count = batch as i64)),
                batch,
            ))
            .on_activate_fn(mk(|v, _c, id| v.duplicate_many(&v.batch_for(id)))),
        )
        .item(
            MenuItem::new(batched(
                tr!(corkboard_move_to()),
                tr!(corkboard_move_to_n(count = batch as i64)),
                batch,
            ))
            .on_activate_fn({
                let vm = vm.clone();
                move |ctx: &mut EventContext| {
                    super::move_target::present_move_target(ctx, vm.clone(), vm.batch_for(id))
                }
            }),
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

    list = list
        .separator()
        // "Where does this sit in the project?" — the question a drilled-into board
        // makes easy to lose. Fires the same intent the Overview row menu does.
        .item(
            MenuItem::new(tr!(reveal_in_outline()))
                .on_activate_fn(mk(|v, c, id| v.reveal_in_outline(c, id))),
        )
        .separator()
        .item(
            MenuItem::new(batched(
                tr!(move_to_trash()),
                tr!(move_to_trash_n(count = batch as i64)),
                batch,
            ))
            .text_role(TextRole::Error)
            .on_activate_fn(mk(|v, c, id| v.trash_many(c, &v.batch_for(id)))),
        );

    // The kebab is already the "there is more here" glyph, so the disclosure
    // caret `PopoverIconButton` paints in its corner would be a second one
    // competing with it — the same reason the comments card suppresses it under
    // its chevron.
    PopoverIconButton::new(IconButton::more())
        .bare()
        .show_disclosure_caret(false)
        .content(list)
}

/// Pick the plain or the counted label for a batch action.
///
/// A menu that always said "Delete 1 card" would be noise; one that always said
/// "Delete" would hide that four cards are about to go. So the count appears only
/// when it is news — the same rule the outline's own batch entries follow.
fn batched(one: LocalizedString, many: LocalizedString, count: usize) -> LocalizedString {
    if count > 1 { many } else { one }
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
/// a superset of the visible title (Label-in-Name) — "[Card N, ]Title, Type[, status]"
/// — not the whole synopsis (which is scanned visually / read on demand).
///
/// The ordinal is spoken only when the writer has card numbers on. The card face
/// shows a bare numeral (as an index card does); "Card 3" is the *spoken* form,
/// because "3, Copyright, Scene" would leave a screen-reader user to infer what
/// the 3 counts.
pub(super) fn card_a11y_name(card: &CorkboardCard, number: Option<usize>) -> String {
    let mut name = format!(
        "{}, {}",
        card.title,
        sub_role_badge_label(&card.sub_role).resolve_now()
    );
    if !card.label.is_empty() {
        name = format!("{name}, {}", card.label);
    }
    match number {
        Some(n) => format!(
            "{}, {name}",
            tr!(corkboard_card_number(number = n as i64)).resolve_now()
        ),
        None => name,
    }
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
        Paratext => tr!(corkboard_badge_paratext()),
        None => tr!(corkboard_badge_folder()),
        BookEnd => tr!(corkboard_badge_end()),
        Text => tr!(corkboard_badge_text()),
    }
}
